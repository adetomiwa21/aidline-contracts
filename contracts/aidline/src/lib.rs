#![no_std]

//! Aidline: milestone based escrow for disaster relief and climate campaigns.
//!
//! Donations are held by the contract and only reach the beneficiary one
//! milestone at a time, after the campaign's verifier confirms the work was
//! done. If a campaign is cancelled or runs out of time, donors can reclaim
//! their share of whatever has not been released yet.

mod errors;
mod events;
mod storage;
mod types;

#[cfg(test)]
mod test;

use soroban_sdk::{Address, Env, String, Vec, contract, contractimpl, token};

pub use errors::Error;
pub use types::{Campaign, CampaignKind, CampaignStatus};

use events::{
    CampaignCancelled, CampaignCreated, Donated, MilestoneReleased, Refunded, VerifierUpdated,
};

const MAX_MILESTONES: u32 = 20;

#[contract]
pub struct Aidline;

#[contractimpl]
impl Aidline {
    /// Sets the admin and the token (a Stellar Asset Contract such as USDC)
    /// that every campaign raises in.
    pub fn __constructor(env: Env, admin: Address, token: Address) {
        storage::set_admin(&env, &admin);
        storage::set_token(&env, &token);
        storage::bump_instance(&env);
    }

    // Admin

    pub fn add_verifier(env: Env, verifier: Address) {
        storage::admin(&env).require_auth();
        storage::set_verifier(&env, &verifier, true);
        VerifierUpdated {
            verifier,
            active: true,
        }
        .publish(&env);
    }

    /// Removing a verifier freezes milestone approvals on their campaigns
    /// until the campaign is cancelled or reassigned.
    pub fn remove_verifier(env: Env, verifier: Address) {
        storage::admin(&env).require_auth();
        storage::set_verifier(&env, &verifier, false);
        VerifierUpdated {
            verifier,
            active: false,
        }
        .publish(&env);
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        storage::admin(&env).require_auth();
        storage::set_admin(&env, &new_admin);
    }

    // Campaigns

    pub fn create_campaign(
        env: Env,
        creator: Address,
        beneficiary: Address,
        verifier: Address,
        kind: CampaignKind,
        metadata_uri: String,
        deadline: u64,
        milestones: Vec<i128>,
    ) -> Result<u64, Error> {
        creator.require_auth();
        storage::bump_instance(&env);

        if !storage::is_verifier(&env, &verifier) {
            return Err(Error::NotVerifier);
        }
        if deadline <= env.ledger().timestamp() {
            return Err(Error::DeadlineInPast);
        }
        if milestones.is_empty() || milestones.len() > MAX_MILESTONES {
            return Err(Error::InvalidMilestones);
        }
        let mut goal: i128 = 0;
        for amount in milestones.iter() {
            if amount <= 0 {
                return Err(Error::InvalidMilestones);
            }
            goal = goal.checked_add(amount).ok_or(Error::InvalidMilestones)?;
        }

        let id = storage::next_campaign_id(&env);
        let campaign = Campaign {
            id,
            creator: creator.clone(),
            beneficiary,
            verifier,
            kind,
            metadata_uri,
            goal,
            deadline,
            milestones,
            milestones_released: 0,
            raised: 0,
            released: 0,
            status: CampaignStatus::Active,
        };
        storage::save_campaign(&env, &campaign);

        CampaignCreated {
            campaign_id: id,
            creator,
            kind,
            goal,
            deadline,
        }
        .publish(&env);
        Ok(id)
    }

    pub fn donate(env: Env, donor: Address, campaign_id: u64, amount: i128) -> Result<(), Error> {
        donor.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let mut campaign = storage::campaign(&env, campaign_id)?;
        Self::ensure_open(&env, &campaign)?;
        let raised = campaign
            .raised
            .checked_add(amount)
            .ok_or(Error::InvalidAmount)?;
        if raised > campaign.goal {
            return Err(Error::GoalExceeded);
        }

        token::Client::new(&env, &storage::token(&env)).transfer(
            &donor,
            env.current_contract_address(),
            &amount,
        );

        campaign.raised = raised;
        storage::save_campaign(&env, &campaign);
        let previous = storage::contribution(&env, campaign_id, &donor);
        storage::set_contribution(&env, campaign_id, &donor, previous + amount);

        Donated {
            campaign_id,
            donor,
            amount,
        }
        .publish(&env);
        Ok(())
    }

    /// Called by the campaign's verifier once the next milestone is done.
    /// Pays that milestone to the beneficiary. `proof_uri` points at the
    /// evidence (photos, receipts, reports) and is emitted for indexers.
    pub fn approve_milestone(env: Env, campaign_id: u64, proof_uri: String) -> Result<i128, Error> {
        let mut campaign = storage::campaign(&env, campaign_id)?;
        campaign.verifier.require_auth();
        if !storage::is_verifier(&env, &campaign.verifier) {
            return Err(Error::NotVerifier);
        }
        Self::ensure_open(&env, &campaign)?;

        let index = campaign.milestones_released;
        let amount = campaign
            .milestones
            .get(index)
            .ok_or(Error::NoMilestonesLeft)?;
        if campaign.raised - campaign.released < amount {
            return Err(Error::MilestoneNotFunded);
        }

        token::Client::new(&env, &storage::token(&env)).transfer(
            &env.current_contract_address(),
            &campaign.beneficiary,
            &amount,
        );

        campaign.released += amount;
        campaign.milestones_released += 1;
        if campaign.milestones_released == campaign.milestones.len() {
            campaign.status = CampaignStatus::Completed;
        }
        storage::save_campaign(&env, &campaign);

        MilestoneReleased {
            campaign_id,
            index,
            amount,
            proof_uri,
        }
        .publish(&env);
        Ok(amount)
    }

    /// Stops a campaign early so donors can reclaim unreleased funds.
    /// Allowed for the creator or the admin.
    pub fn cancel_campaign(env: Env, caller: Address, campaign_id: u64) -> Result<(), Error> {
        caller.require_auth();
        let mut campaign = storage::campaign(&env, campaign_id)?;
        if caller != campaign.creator && caller != storage::admin(&env) {
            return Err(Error::Unauthorized);
        }
        if campaign.status != CampaignStatus::Active {
            return Err(Error::CampaignNotActive);
        }

        campaign.status = CampaignStatus::Cancelled;
        storage::save_campaign(&env, &campaign);

        CampaignCancelled { campaign_id }.publish(&env);
        Ok(())
    }

    /// Returns the donor's pro rata share of funds that were never released.
    /// Available once a campaign is cancelled, or has passed its deadline
    /// without completing.
    pub fn refund(env: Env, donor: Address, campaign_id: u64) -> Result<i128, Error> {
        donor.require_auth();
        let campaign = storage::campaign(&env, campaign_id)?;

        let expired = campaign.status == CampaignStatus::Active
            && env.ledger().timestamp() > campaign.deadline;
        if campaign.status != CampaignStatus::Cancelled && !expired {
            return Err(Error::RefundNotAvailable);
        }

        let contributed = storage::contribution(&env, campaign_id, &donor);
        if contributed == 0 {
            return Err(Error::NothingToRefund);
        }
        // `raised` and `released` are frozen once refunds open, so every donor
        // is measured against the same pool.
        let unreleased = campaign.raised - campaign.released;
        let amount = contributed * unreleased / campaign.raised;

        storage::set_contribution(&env, campaign_id, &donor, 0);
        if amount > 0 {
            token::Client::new(&env, &storage::token(&env)).transfer(
                &env.current_contract_address(),
                &donor,
                &amount,
            );
        }

        Refunded {
            campaign_id,
            donor,
            amount,
        }
        .publish(&env);
        Ok(amount)
    }

    // Views

    pub fn get_campaign(env: Env, campaign_id: u64) -> Result<Campaign, Error> {
        storage::campaign(&env, campaign_id)
    }

    pub fn campaign_count(env: Env) -> u64 {
        storage::campaign_count(&env)
    }

    pub fn contribution_of(env: Env, campaign_id: u64, donor: Address) -> i128 {
        storage::contribution(&env, campaign_id, &donor)
    }

    pub fn is_verifier(env: Env, who: Address) -> bool {
        storage::is_verifier(&env, &who)
    }

    pub fn admin(env: Env) -> Address {
        storage::admin(&env)
    }

    pub fn token(env: Env) -> Address {
        storage::token(&env)
    }

    // Internal

    fn ensure_open(env: &Env, campaign: &Campaign) -> Result<(), Error> {
        if campaign.status != CampaignStatus::Active {
            return Err(Error::CampaignNotActive);
        }
        if env.ledger().timestamp() > campaign.deadline {
            return Err(Error::CampaignExpired);
        }
        Ok(())
    }
}

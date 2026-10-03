use soroban_sdk::{Address, String, contractevent};

use crate::types::CampaignKind;

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignCreated {
    #[topic]
    pub campaign_id: u64,
    pub creator: Address,
    pub kind: CampaignKind,
    pub goal: i128,
    pub deadline: u64,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Donated {
    #[topic]
    pub campaign_id: u64,
    #[topic]
    pub donor: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneReleased {
    #[topic]
    pub campaign_id: u64,
    pub index: u32,
    pub amount: i128,
    pub proof_uri: String,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignCancelled {
    #[topic]
    pub campaign_id: u64,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Refunded {
    #[topic]
    pub campaign_id: u64,
    #[topic]
    pub donor: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifierUpdated {
    #[topic]
    pub verifier: Address,
    pub active: bool,
}

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    CampaignNotFound = 1,
    CampaignNotActive = 2,
    CampaignExpired = 3,
    InvalidAmount = 4,
    InvalidMilestones = 5,
    DeadlineInPast = 6,
    NotVerifier = 7,
    GoalExceeded = 8,
    MilestoneNotFunded = 9,
    NoMilestonesLeft = 10,
    RefundNotAvailable = 11,
    NothingToRefund = 12,
    Unauthorized = 13,
}

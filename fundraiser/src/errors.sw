library errors;

pub enum CreationError {
    CannotReinitialize: (),
    ContractNotInitialized: (),
    TargetAmountCannotBeZero: (),
}

pub enum UserError {
    //AlreadyClaimed: (),
    AmountCannotBeZero: (),
    IncorrectAssetSent: (),
    InvalidId: (),
    UnauthorizedUser: (),
    //UserHasNotPledged: (),
}

pub enum CampaignError {
    TargetNotReached: (),
    CampaignNoLongerActive: (),
}

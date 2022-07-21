library fundraiser_library;

use core::ops::Eq;

use std::{
    identity::Identity,
    contract_id::ContractId,
};

abi Fundraiser {
    // initialize the fundraiser contract
    #[storage(read, write)]
    fn initialize(token: ContractId);

    // checks to see if the contract is initialized
    #[storage(read)]
    fn is_initialized() -> bool;

    // get the campaign with the given campaign number
    #[storage(read)]
    fn get_campaign(campaign_id: u64) -> Campaign;

    // create a new campaign
    #[storage(read, write)]
    fn create_campaign(beneficiary: Identity, goal_amount: u64) -> u64;

    // cancel a campaign
    #[storage(read, write)]
    fn cancel_campaign(campaign_id: u64);

    // pledge an amount to a campaign
    #[storage(read, write)]
    fn pledge(campaign_id: u64);

    // completes a campaign and sends the total amount pledged to the campaign beneficiary
    #[storage(read, write)]
    fn complete_campaign(campaign_id: u64);
}

// represents a campaign
pub struct Campaign {
    author: Identity,
    beneficiary: Identity,
    goal_amount: u64,
    current_amount: u64,
    is_active: bool
}

pub enum State {
    NotInitialized: (),
    Initialized: (),
}

impl Eq for State {
    fn eq(self, other: Self) -> bool {
        match(self, other) {
            (State::Initialized, State::Initialized) => true,
            (State::NotInitialized, State::NotInitialized) => true,
            _ => false, 
        }
    }
}

library fundraiser_library;

use core::ops::Eq;

use std::{
    identity::Identity,
    contract_id::ContractId,
};

abi Fundraiser {
    #[storage(read, write)]
    fn initialize(token: ContractId);

    #[storage(read)]
    fn get_campaign(campaign_id: u64) -> Campaign;

    #[storage(read, write)]
    fn create_campaign(beneficiary: Identity, goal_amount: u64) -> u64;

    #[storage(read, write)]
    fn cancel_campaign(campaign_id: u64);

    #[storage(read, write)]
    fn pledge(campaign_id: u64);

    #[storage(read, write)]
    fn complete_campaign(campaign_id: u64);
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

pub struct Campaign {
    author: Identity,
    beneficiary: Identity,
    goal_amount: u64,
    current_amount: u64,
    is_active: bool
}

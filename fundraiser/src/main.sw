contract;

dep fundraiser_library;
dep errors;

use fundraiser_library::*;
use errors::*;

use std::{
    identity::Identity,
    contract_id::ContractId,
    assert::require,
    storage::StorageMap,
    chain::auth::{AuthError, msg_sender},
    result::*,
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    token::transfer,
    revert::revert,
};

storage {
    // tells us if it is initialized yet or not
    state: State = State::NotInitialized,

    // contract id of the accepted asset
    asset: ContractId = ContractId {
        value: 0x0000000000000000000000000000000000000000000000000000000000000000,
    },

    // the current total number of campaigns
    next_campaign_id: u64 = 0,

    // all of the campaigns
    campaigns: StorageMap<u64, Campaign> = StorageMap {},
}

impl Fundraiser for Contract {
    // initialize the fundraiser contract
    #[storage(read, write)]
    fn initialize(asset: ContractId) {
        require(storage.state == State::NotInitialized, CreationError::CannotReinitialize);

        /*

        TODO:

        1. set storage.state the State::Initialized
        2. set storage.asset to the asset passed into the function

        */
    }

    // checks to see if the contract is initialized
    #[storage(read)]
    fn is_initialized() -> bool {
        match storage.state {
            State::Initialized => true,
            State::NotInitialized => false,
        }
    }

    // get the campaign with the given campaign number
    #[storage(read)]
    fn get_campaign(campaign_id: u64) -> Campaign {
        require(storage.state == State::Initialized, CreationError::ContractNotInitialized);
        require(campaign_id < storage.next_campaign_id, UserError::InvalidId);
        storage.campaigns.get(campaign_id)
    }

    // create a new campaign
    #[storage(read, write)]
    fn create_campaign(beneficiary: Identity, goal_amount: u64) -> u64 {
        require(storage.state == State::Initialized, CreationError::ContractNotInitialized);

        0 // remove this

        /*

        TODO:

        1. get the user that called the contract with the function call "msg_sender()".
            This function returns a Result, so you will need to call "msg_sender().unwrap()"
        2. create the campaign using the struct syntax. set the author to the user from step (1),
            the current_amount to 0, is_active to true, and beneficiary and goal_amount to the
            arguments
        3. get the campaign id from storage.next_campaign_id
        4. insert the campaign into storage.campaigns using the id from step (3) and the campaign
            you created in step (2)
        5. increment storage.next_campaign_id
        6. return the campaign id from step (3)

        struct syntax:

        Campaign {
            author,
            beneficiary,
            goal_amount,
            current_amount: 0,
            is_active: true
        }

        NOTE:

        1. consider the case in which the goal amount is zero. use a require statement and return
            CreationError::TargetAmountCannotBeZero

        */
    }

    // cancel a campaign
    #[storage(read, write)]
    fn cancel_campaign(campaign_id: u64) {
        require(storage.state == State::Initialized, CreationError::ContractNotInitialized);

        /*

        TODO:

        1. get the campaign from storage.campaigns
        2. get the user that called the contract with the function call "msg_sender()".
            This function returns a Result, so you will need to call "msg_sender().unwrap()"
        3. set the campaign is_active field to false
        4. re-insert the campaign into storage.campaigns

        NOTE:

        1. consider the case in which the campaign_id is less than storage.next_campaign_id. use a
            require statement and return UserError::InvalidId
        2. consider the case in which the user that called the contract to cancel the campaign is
            not the author of the campaign that they want to cancel. use a require statement and
            return UserError::UnauthorizedUser

        */
    }

    // pledge an amount to a campaign
    #[storage(read, write)]
    fn pledge(campaign_id: u64) {
        require(storage.state == State::Initialized, CreationError::ContractNotInitialized);

        /*

        TODO:

        1. get the campaign from storage.campaigns
        2. get the pledge amount with the function call "msg_amount()". This function returns a Result,
            so you will need to call "msg_amount().unwrap()"
        3. increase the current_amount on the campaign by the pledge amount from step (2)
        4. re-insert the campaign into storage.campaigns

        NOTE:

        1. consider the case in which the campaign_id is less than storage.next_campaign_id. use a
            require statement and return UserError::InvalidId
        2. consider the case in which someone sends an incorrect asset. use a require statement and
            return UserError::IncorrectAssetSent
        3. consider the case in which the campaign is not active. use a require statement and return
            CampaignError::CampaignNoLongerActive
        4. consider the case in which someone attemps to pledge 0. use a require statement and
            return UserError::AmountCannotBeZero

        */
    }

    // completes a campaign and sends the total amount pledged to the campaign beneficiary
    #[storage(read, write)]
    fn complete_campaign(campaign_id: u64) {
        require(storage.state == State::Initialized, CreationError::ContractNotInitialized);

        /*

        TODO:

        1. get the campaign from storage.campaigns
        2. set the campaign is_active field to false
        3. re-insert the campaign into storage.campaigns
        4. transfer the total pledged to this campaign to the beneficiary using the "transfer" function

        function signature for the "transfer" function:
        fn transfer(amount: u64, asset_id: ContractId, to: Identity)

        NOTE:

        1. consider the case in which the campaign_id is less than storage.next_campaign_id. use a
            require statement and return UserError::InvalidId
        2. consider the case in which the campaign is not active. use a require statement and return
            CampaignError::CampaignNoLongerActive
        3. consider the case in which current amount pledged to the campaign is less than the goal.
            use a require statement and return CampaignError::TargetNotReached

        */
    }
}

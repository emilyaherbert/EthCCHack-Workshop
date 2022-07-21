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
};

storage {
    // tells us if it is initialized yet or not
    state: State = State::NotInitialized,

    // contract id of the accepted asset
    asset: ContractId = ContractId {
        value: 0x0000000000000000000000000000000000000000000000000000000000000000,
    },

    // the current total number of campaigns
    next_campaign_number: u64 = 0,

    // all of the campaigns
    campaigns: StorageMap<u64, Campaign> = StorageMap {},
}

impl Fundraiser for Contract {
    // initialize the fundraiser contract
    #[storage(read, write)]
    fn initialize(asset: ContractId) {
        require(storage.state == State::NotInitialized, CreationError::CannotReinitialize);
        storage.state = State::Initialized;
        storage.asset = asset;
    }

    // get the campaign with the given campaign number
    #[storage(read)]
    fn get_campaign(campaign_number: u64) -> Campaign {
        require(storage.state == State::Initialized, CreationError::ContractNotInitialized);
        require(campaign_number < storage.next_campaign_number, UserError::InvalidId);
        storage.campaigns.get(campaign_number)
    }

    // create a new campaign
    #[storage(read, write)]
    fn create_campaign(beneficiary: Identity, goal_amount: u64) -> u64 {
        require(storage.state == State::Initialized, CreationError::ContractNotInitialized);
        require(0 < goal_amount, CreationError::TargetAmountCannotBeZero);
        let author = msg_sender().unwrap();
        let campaign = Campaign {
            author,
            beneficiary,
            goal_amount,
            current_amount: 0,
            is_active: true
        };
        let campaign_number = storage.next_campaign_number;
        storage.campaigns.insert(campaign_number, campaign);
        storage.next_campaign_number += 1;
        campaign_number
    }

    // cancel a campaign
    #[storage(read, write)]
    fn cancel_campaign(campaign_number: u64) {
        require(storage.state == State::Initialized, CreationError::ContractNotInitialized);
        require(campaign_number < storage.next_campaign_number, UserError::InvalidId);

        let mut campaign = storage.campaigns.get(campaign_number);
        let user = msg_sender().unwrap();

        require(campaign.author == user, UserError::UnauthorizedUser);

        campaign.is_active = false;
        storage.campaigns.insert(campaign_number, campaign);
    }

    // pledge an amount to a campaign
    #[storage(read, write)]
    fn pledge(campaign_number: u64) {
        require(storage.state == State::Initialized, CreationError::ContractNotInitialized);
        require(campaign_number < storage.next_campaign_number, UserError::InvalidId);
        require(storage.asset == msg_asset_id(), UserError::IncorrectAssetSent);

        let mut campaign = storage.campaigns.get(campaign_number);
        let pledge_amount = msg_amount();

        require(campaign.is_active, CampaignError::CampaignNoLongerActive);
        require(pledge_amount > 0, UserError::AmountCannotBeZero);

        campaign.current_amount += pledge_amount;
    }

    // completes a campaign and sends the total amount pledged to the campaign beneficiary
    #[storage(read, write)]
    fn complete_compaign(campaign_number: u64) {
        require(storage.state == State::Initialized, CreationError::ContractNotInitialized);
        require(campaign_number < storage.next_campaign_number, UserError::InvalidId);

        let mut campaign = storage.campaigns.get(campaign_number);

        require(campaign.is_active, CampaignError::CampaignNoLongerActive);
        require(campaign.current_amount >= campaign.goal_amount, CampaignError::TargetNotReached);

        campaign.is_active = false;
        storage.campaigns.insert(campaign_number, campaign);

        // Transfer the total pledged to this campaign to the beneficiary
        transfer(campaign.current_amount, storage.asset, campaign.beneficiary);
    }
}

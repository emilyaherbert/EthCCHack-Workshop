#![allow(dead_code)]

mod utils;

use fuels::prelude::*;
use utils::*;

#[cfg(test)]
mod should_pass {
    use super::*;

    #[tokio::test]
    async fn deployer_can_mint() {
        let (asset_contract_id, asset_handle, [_, user_1, user_2, user_3]) = setup_tests().await;

        // expect user_1, user_2, and user_3 to each have no tokens
        let user_1_tokens =
            get_asset_balance_in_wallet(asset_contract_id, user_1.wallet.clone()).await;
        let user_2_tokens =
            get_asset_balance_in_wallet(asset_contract_id, user_2.wallet.clone()).await;
        let user_3_tokens =
            get_asset_balance_in_wallet(asset_contract_id, user_3.wallet.clone()).await;
        assert_eq!(user_1_tokens, None);
        assert_eq!(user_2_tokens, None);
        assert_eq!(user_3_tokens, None);

        // mint tokens and send them to users
        mint_and_send_to_address(&asset_handle, 1_000_000, user_1.wallet.address()).await;
        mint_and_send_to_address(&asset_handle, 2_000_000, user_2.wallet.address()).await;
        mint_and_send_to_address(&asset_handle, 3_000_000, user_3.wallet.address()).await;

        // expect users to have tokens
        assert_eq!(
            get_asset_balance_in_wallet(asset_contract_id, user_1.wallet).await,
            Some(1_000_000)
        );
        assert_eq!(
            get_asset_balance_in_wallet(asset_contract_id, user_2.wallet).await,
            Some(2_000_000)
        );
        assert_eq!(
            get_asset_balance_in_wallet(asset_contract_id, user_3.wallet).await,
            Some(3_000_000)
        );
    }

    #[tokio::test]
    async fn deployer_can_initialize_the_fundraiser_contract() {
        let (asset_contract_id, _, [deployer, _, _, _]) = setup_tests().await;

        // initialize the voting contract
        initialize_fundraiser_contract(asset_contract_id, &deployer.fundraiser_handle).await;

        assert!(is_initialized(&deployer.fundraiser_handle).await);
    }

    #[tokio::test]
    async fn users_can_create_campaigns() {
        let (asset_contract_id, asset_handle, [deployer, user_1, user_2, user_3]) =
            setup_tests().await;

        // initialize the voting contract
        initialize_fundraiser_contract(asset_contract_id, &deployer.fundraiser_handle).await;

        // mint tokens and send them to users
        mint_and_send_to_address(&asset_handle, 1_000_000, user_1.wallet.address()).await;
        mint_and_send_to_address(&asset_handle, 1_000_000, user_2.wallet.address()).await;
        mint_and_send_to_address(&asset_handle, 1_000_000, user_3.wallet.address()).await;

        // create a campaign for 100_000 with user_3 as the beneficiary
        let id_a = create_campaign(&user_1.fundraiser_handle, &user_3.identity, 100_000).await;
        let campaign_a = get_campaign(&deployer.fundraiser_handle, id_a).await;
        assert_eq!(campaign_a.beneficiary, user_3.identity);

        // create a campaign for 200_000 with user_2 as the beneficiary
        let id_b = create_campaign(&user_3.fundraiser_handle, &user_2.identity, 200_000).await;
        let campaign_b = get_campaign(&deployer.fundraiser_handle, id_b).await;
        assert_eq!(campaign_b.beneficiary, user_2.identity);
    }

    #[tokio::test]
    async fn users_can_cancel_campaigns() {
        let (asset_contract_id, asset_handle, [deployer, user_1, user_2, user_3]) =
            setup_tests().await;

        // initialize the voting contract
        initialize_fundraiser_contract(asset_contract_id, &deployer.fundraiser_handle).await;

        // mint tokens and send them to users
        mint_and_send_to_address(&asset_handle, 1_000_000, user_1.wallet.address()).await;
        mint_and_send_to_address(&asset_handle, 1_000_000, user_2.wallet.address()).await;
        mint_and_send_to_address(&asset_handle, 1_000_000, user_3.wallet.address()).await;

        // create a campaign for 100_000 with user_3 as the beneficiary
        let id_a = create_campaign(&user_1.fundraiser_handle, &user_3.identity, 100_000).await;
        let campaign_a = get_campaign(&deployer.fundraiser_handle, id_a).await;
        assert!(campaign_a.is_active);

        // create a campaign for 200_000 with user_2 as the beneficiary
        let id_b = create_campaign(&user_3.fundraiser_handle, &user_2.identity, 200_000).await;
        let campaign_b = get_campaign(&deployer.fundraiser_handle, id_b).await;
        assert!(campaign_b.is_active);

        // cancel the first campaign
        cancel_campaign(&user_1.fundraiser_handle, id_a).await;
        let campaign_a = get_campaign(&deployer.fundraiser_handle, id_a).await;
        assert!(!campaign_a.is_active);

        // cancel the second campaign
        cancel_campaign(&user_3.fundraiser_handle, id_b).await;
        let campaign_b = get_campaign(&deployer.fundraiser_handle, id_b).await;
        assert!(!campaign_b.is_active);
    }

    #[tokio::test]
    async fn users_can_pledge() {
        let (asset_contract_id, asset_handle, [deployer, user_1, user_2, user_3]) =
            setup_tests().await;

        // initialize the voting contract
        initialize_fundraiser_contract(asset_contract_id, &deployer.fundraiser_handle).await;

        // mint tokens and send them to users
        mint_and_send_to_address(&asset_handle, 1_000_000, user_1.wallet.address()).await;
        mint_and_send_to_address(&asset_handle, 1_000_000, user_2.wallet.address()).await;
        mint_and_send_to_address(&asset_handle, 1_000_000, user_3.wallet.address()).await;

        // create a campaign for 100_000 with user_3 as the beneficiary
        let id_a = create_campaign(&user_1.fundraiser_handle, &user_3.identity, 100_000).await;

        // create a campaign for 200_000 with user_2 as the beneficiary
        let id_b = create_campaign(&user_3.fundraiser_handle, &user_2.identity, 200_000).await;

        // make the pledges
        pledge(&user_2.fundraiser_handle, asset_contract_id, id_a, 300_000).await;
        pledge(&user_1.fundraiser_handle, asset_contract_id, id_b, 200_000).await;

        // check that the campaigns have the amounts we expect
        let campaign_a = get_campaign(&deployer.fundraiser_handle, id_a).await;
        assert_eq!(campaign_a.current_amount, 300_000);
        let campaign_b = get_campaign(&deployer.fundraiser_handle, id_b).await;
        assert_eq!(campaign_b.current_amount, 200_000);

        // check that the user accounts have the amounts we expect
        assert_eq!(
            get_asset_balance_in_wallet(asset_contract_id, user_1.wallet).await,
            Some(800_000)
        );
        assert_eq!(
            get_asset_balance_in_wallet(asset_contract_id, user_2.wallet).await,
            Some(700_000)
        );
        assert_eq!(
            get_asset_balance_in_wallet(asset_contract_id, user_3.wallet).await,
            Some(1_000_000)
        );
    }

    #[tokio::test]
    async fn users_can_complete_campaigns() {
        let (asset_contract_id, asset_handle, [deployer, user_1, user_2, user_3]) =
            setup_tests().await;

        // initialize the voting contract
        initialize_fundraiser_contract(asset_contract_id, &deployer.fundraiser_handle).await;

        // mint tokens and send them to users
        mint_and_send_to_address(&asset_handle, 1_000_000, user_1.wallet.address()).await;
        mint_and_send_to_address(&asset_handle, 1_000_000, user_2.wallet.address()).await;
        mint_and_send_to_address(&asset_handle, 1_000_000, user_3.wallet.address()).await;

        // create a campaign for 100_000 with user_3 as the beneficiary
        let id_a = create_campaign(&user_1.fundraiser_handle, &user_3.identity, 100_000).await;

        // create a campaign for 200_000 with user_2 as the beneficiary
        let id_b = create_campaign(&user_3.fundraiser_handle, &user_2.identity, 200_000).await;

        // make the pledges
        pledge(&user_2.fundraiser_handle, asset_contract_id, id_a, 300_000).await;
        pledge(&user_1.fundraiser_handle, asset_contract_id, id_b, 200_000).await;

        // complete the campaigns
        complete_campaign(&user_1.fundraiser_handle, id_a).await;
        complete_campaign(&user_3.fundraiser_handle, id_b).await;

        // check that the user accounts have the amounts we expect
        assert_eq!(
            get_asset_balance_in_wallet(asset_contract_id, user_1.wallet).await,
            Some(800_000)
        );
        assert_eq!(
            get_asset_balance_in_wallet(asset_contract_id, user_2.wallet).await,
            Some(900_000)
        );
        assert_eq!(
            get_asset_balance_in_wallet(asset_contract_id, user_3.wallet).await,
            Some(1_300_000)
        );
    }
}

#![allow(dead_code)]

mod utils;

use fuels::prelude::*;
use utils::*;

#[cfg(test)]
mod should_pass {
    use super::*;

    #[tokio::test]
    async fn deployer_can_mint() {
        let (asset_contract_id, asset_handle, [_, user_1, user_2, user_3]) =
            setup_tests().await;

        // expect user_1, user_2, and user_3 to each have no tokens
        let user_1_tokens =
            get_token_balance_in_wallet(asset_contract_id, user_1.wallet.clone()).await;
        let user_2_tokens =
            get_token_balance_in_wallet(asset_contract_id, user_2.wallet.clone()).await;
        let user_3_tokens =
            get_token_balance_in_wallet(asset_contract_id, user_3.wallet.clone()).await;
        assert_eq!(user_1_tokens, None);
        assert_eq!(user_2_tokens, None);
        assert_eq!(user_3_tokens, None);

        // mint tokens and send them to users
        mint_and_send_to_address(&asset_handle, 1_000_000, user_1.wallet.address()).await;
        mint_and_send_to_address(&asset_handle, 2_000_000, user_2.wallet.address()).await;
        mint_and_send_to_address(&asset_handle, 3_000_000, user_3.wallet.address()).await;

        // expect users to have tokens
        assert_eq!(
            get_token_balance_in_wallet(asset_contract_id, user_1.wallet).await,
            Some(1_000_000)
        );
        assert_eq!(
            get_token_balance_in_wallet(asset_contract_id, user_2.wallet).await,
            Some(2_000_000)
        );
        assert_eq!(
            get_token_balance_in_wallet(asset_contract_id, user_3.wallet).await,
            Some(3_000_000)
        );
    }

}

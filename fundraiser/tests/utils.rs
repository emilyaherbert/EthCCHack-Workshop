#![allow(dead_code)]

use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(Fundraiser, "./out/debug/fundraiser-abi.json");
abigen!(Asset, "../asset/out/debug/asset-abi.json");

pub(crate) struct User {
    pub(crate) fundraiser_handle: Fundraiser,
    pub(crate) wallet: LocalWallet,
}

pub(crate) async fn setup_tests() -> (ContractId, Asset, [User; 4]) {
    let num_wallets = 4;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000;
    let config = WalletsConfig::new(
        Some(num_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );
    let mut wallets = launch_custom_provider_and_get_wallets(config, None).await;
    let deployer_wallet = wallets.pop().unwrap();
    let user_1_wallet = wallets.pop().unwrap();
    let user_2_wallet = wallets.pop().unwrap();
    let user_3_wallet = wallets.pop().unwrap();

    let asset_contract_id = Contract::deploy(
        "../asset/out/debug/asset.bin",
        &deployer_wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "../asset/out/debug/asset-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let asset_handle = Asset::new(asset_contract_id.to_string(), deployer_wallet.clone());

    let fundraiser_contract_id = Contract::deploy(
        "./out/debug/fundraiser.bin",
        &deployer_wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/fundraiser-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let deployer = User {
        fundraiser_handle: Fundraiser::new(fundraiser_contract_id.to_string(), deployer_wallet.clone()),
        wallet: deployer_wallet,
    };
    let user_1 = User {
        fundraiser_handle: Fundraiser::new(fundraiser_contract_id.to_string(), user_1_wallet.clone()),
        wallet: user_1_wallet,
    };
    let user_2 = User {
        fundraiser_handle: Fundraiser::new(fundraiser_contract_id.to_string(), user_2_wallet.clone()),
        wallet: user_2_wallet,
    };
    let user_3 = User {
        fundraiser_handle: Fundraiser::new(fundraiser_contract_id.to_string(), user_3_wallet.clone()),
        wallet: user_3_wallet,
    };

    (
        asset_contract_id,
        asset_handle,
        [deployer, user_1, user_2, user_3],
    )
}

pub(crate) async fn get_token_balance_in_wallet(
    token_contract_id: ContractId,
    wallet: LocalWallet,
) -> Option<u64> {
    let mut x_string = "0x".to_string();
    x_string.push_str(&token_contract_id.to_string());
    let balances = wallet.get_balances().await.unwrap();
    balances.get(&x_string).cloned()
}

pub(crate) async fn mint_and_send_to_address(
    asset_handle: &Asset,
    asset_amount: u64,
    address: Address,
) {
    asset_handle
        .mint_and_send_to_address(asset_amount, address)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
}

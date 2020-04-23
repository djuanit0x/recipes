//! Helper module to build a genesis configuration for the weight-fee-runtime

use super::{
	AccountId, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, GenericAssetConfig, WASM_BINARY,
};

use sp_finality_grandpa::{AuthorityId as GrandpaId};

/// Helper function to build a genesis configuration
pub fn testnet_genesis(initial_authorities: Vec<GrandpaId>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		generic_asset: Some(GenericAssetConfig {
			assets: vec![
				13,
				1,
			],
			initial_balance: 10u128.pow(18 + 9), // 1 billion token with 18 decimals
			endowed_accounts: endowed_accounts.clone().into_iter().map(Into::into).collect(),
			next_asset_id: 100,
			staking_asset_id: 1,
			spending_asset_id: 1,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.clone(), 1)).collect(),
		}),
	}
}
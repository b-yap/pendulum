
//! Autogenerated weights for replace
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-09-11, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Gianfrancos-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("foucoco"), DB CACHE: 1024

// Executed Command:
// ../target/production/pendulum-node
// benchmark
// pallet
// --chain
// foucoco
// --wasm-execution=compiled
// --pallet
// replace
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ../runtime/foucoco/src/weights/replace.rs
// --template
// frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for replace using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> replace::WeightInfo for SubstrateWeight<T> {
	/// Storage: `VaultRegistry::Vaults` (r:1 w:1)
	/// Proof: `VaultRegistry::Vaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Nomination::Vaults` (r:1 w:0)
	/// Proof: `Nomination::Vaults` (`max_values`: None, `max_size`: Some(141), added: 2616, mode: `MaxEncodedLen`)
	/// Storage: `Replace::ReplaceMinimumTransferAmount` (r:1 w:0)
	/// Proof: `Replace::ReplaceMinimumTransferAmount` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Security::ParachainStatus` (r:1 w:0)
	/// Proof: `Security::ParachainStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetRegistry::Metadata` (r:2 w:0)
	/// Proof: `AssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(897), added: 3372, mode: `MaxEncodedLen`)
	/// Storage: `Fee::ReplaceGriefingCollateral` (r:1 w:0)
	/// Proof: `Fee::ReplaceGriefingCollateral` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn request_replace() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1532`
		//  Estimated: `7734`
		// Minimum execution time: 58_000_000 picoseconds.
		Weight::from_parts(59_000_000, 7734)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `VaultRegistry::Vaults` (r:1 w:1)
	/// Proof: `VaultRegistry::Vaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn withdraw_replace() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `617`
		//  Estimated: `4082`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(22_000_000, 4082)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `VaultRegistry::Vaults` (r:2 w:2)
	/// Proof: `VaultRegistry::Vaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Replace::ReplaceMinimumTransferAmount` (r:1 w:0)
	/// Proof: `Replace::ReplaceMinimumTransferAmount` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `VaultRegistry::TotalUserVaultCollateral` (r:1 w:1)
	/// Proof: `VaultRegistry::TotalUserVaultCollateral` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultRegistry::SystemCollateralCeiling` (r:1 w:0)
	/// Proof: `VaultRegistry::SystemCollateralCeiling` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::Accounts` (r:1 w:1)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(150), added: 2625, mode: `MaxEncodedLen`)
	/// Storage: `VaultStaking::RewardCurrencies` (r:1 w:0)
	/// Proof: `VaultStaking::RewardCurrencies` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PooledVaultRewards::Stake` (r:1 w:1)
	/// Proof: `PooledVaultRewards::Stake` (`max_values`: None, `max_size`: Some(202), added: 2677, mode: `MaxEncodedLen`)
	/// Storage: `PooledVaultRewards::RewardPerToken` (r:1 w:0)
	/// Proof: `PooledVaultRewards::RewardPerToken` (`max_values`: None, `max_size`: Some(140), added: 2615, mode: `MaxEncodedLen`)
	/// Storage: `PooledVaultRewards::RewardTally` (r:1 w:1)
	/// Proof: `PooledVaultRewards::RewardTally` (`max_values`: None, `max_size`: Some(264), added: 2739, mode: `MaxEncodedLen`)
	/// Storage: `PooledVaultRewards::TotalRewards` (r:1 w:1)
	/// Proof: `PooledVaultRewards::TotalRewards` (`max_values`: None, `max_size`: Some(78), added: 2553, mode: `MaxEncodedLen`)
	/// Storage: `VaultStaking::Nonce` (r:1 w:0)
	/// Proof: `VaultStaking::Nonce` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::TotalCurrentStake` (r:1 w:1)
	/// Proof: `VaultStaking::TotalCurrentStake` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::RewardPerToken` (r:1 w:1)
	/// Proof: `VaultStaking::RewardPerToken` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::Stake` (r:1 w:1)
	/// Proof: `VaultStaking::Stake` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::SlashPerToken` (r:1 w:0)
	/// Proof: `VaultStaking::SlashPerToken` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::SlashTally` (r:1 w:1)
	/// Proof: `VaultStaking::SlashTally` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::TotalStake` (r:1 w:1)
	/// Proof: `VaultStaking::TotalStake` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::RewardTally` (r:1 w:1)
	/// Proof: `VaultStaking::RewardTally` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PooledVaultRewards::TotalStake` (r:1 w:1)
	/// Proof: `PooledVaultRewards::TotalStake` (`max_values`: None, `max_size`: Some(78), added: 2553, mode: `MaxEncodedLen`)
	/// Storage: `PooledVaultRewards::RewardCurrencies` (r:1 w:0)
	/// Proof: `PooledVaultRewards::RewardCurrencies` (`max_values`: None, `max_size`: Some(523), added: 2998, mode: `MaxEncodedLen`)
	/// Storage: `VaultRegistry::SecureCollateralThreshold` (r:1 w:0)
	/// Proof: `VaultRegistry::SecureCollateralThreshold` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Security::ParachainStatus` (r:1 w:0)
	/// Proof: `Security::ParachainStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetRegistry::Metadata` (r:2 w:0)
	/// Proof: `AssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(897), added: 3372, mode: `MaxEncodedLen`)
	/// Storage: `Security::Nonce` (r:1 w:1)
	/// Proof: `Security::Nonce` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Security::ActiveBlockCount` (r:1 w:0)
	/// Proof: `Security::ActiveBlockCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Replace::ReplacePeriod` (r:1 w:0)
	/// Proof: `Replace::ReplacePeriod` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Replace::ReplaceRequests` (r:0 w:1)
	/// Proof: `Replace::ReplaceRequests` (`max_values`: None, `max_size`: Some(431), added: 2906, mode: `MaxEncodedLen`)
	fn accept_replace() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3688`
		//  Estimated: `9628`
		// Minimum execution time: 195_000_000 picoseconds.
		Weight::from_parts(199_000_000, 9628)
			.saturating_add(T::DbWeight::get().reads(29_u64))
			.saturating_add(T::DbWeight::get().writes(16_u64))
	}
	/// Storage: `Replace::ReplaceRequests` (r:1 w:1)
	/// Proof: `Replace::ReplaceRequests` (`max_values`: None, `max_size`: Some(431), added: 2906, mode: `MaxEncodedLen`)
	/// Storage: `StellarRelay::NewValidatorsEnactmentBlockHeight` (r:1 w:0)
	/// Proof: `StellarRelay::NewValidatorsEnactmentBlockHeight` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `StellarRelay::Validators` (r:1 w:0)
	/// Proof: `StellarRelay::Validators` (`max_values`: Some(1), `max_size`: Some(70382), added: 70877, mode: `MaxEncodedLen`)
	/// Storage: `StellarRelay::Organizations` (r:1 w:0)
	/// Proof: `StellarRelay::Organizations` (`max_values`: Some(1), `max_size`: Some(37232), added: 37727, mode: `MaxEncodedLen`)
	/// Storage: `VaultRegistry::Vaults` (r:2 w:2)
	/// Proof: `VaultRegistry::Vaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn execute_replace() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1935`
		//  Estimated: `71867`
		// Minimum execution time: 7_910_000_000 picoseconds.
		Weight::from_parts(8_096_000_000, 71867)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Replace::ReplaceRequests` (r:1 w:1)
	/// Proof: `Replace::ReplaceRequests` (`max_values`: None, `max_size`: Some(431), added: 2906, mode: `MaxEncodedLen`)
	/// Storage: `Replace::ReplacePeriod` (r:1 w:0)
	/// Proof: `Replace::ReplacePeriod` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Security::ActiveBlockCount` (r:1 w:0)
	/// Proof: `Security::ActiveBlockCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `VaultRegistry::Vaults` (r:2 w:2)
	/// Proof: `VaultRegistry::Vaults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::Nonce` (r:1 w:0)
	/// Proof: `VaultStaking::Nonce` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::TotalCurrentStake` (r:1 w:0)
	/// Proof: `VaultStaking::TotalCurrentStake` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultRegistry::SecureCollateralThreshold` (r:1 w:0)
	/// Proof: `VaultRegistry::SecureCollateralThreshold` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Security::ParachainStatus` (r:1 w:0)
	/// Proof: `Security::ParachainStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetRegistry::Metadata` (r:2 w:0)
	/// Proof: `AssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(897), added: 3372, mode: `MaxEncodedLen`)
	/// Storage: `VaultRegistry::TotalUserVaultCollateral` (r:1 w:1)
	/// Proof: `VaultRegistry::TotalUserVaultCollateral` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::RewardCurrencies` (r:1 w:0)
	/// Proof: `VaultStaking::RewardCurrencies` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PooledVaultRewards::Stake` (r:1 w:0)
	/// Proof: `PooledVaultRewards::Stake` (`max_values`: None, `max_size`: Some(202), added: 2677, mode: `MaxEncodedLen`)
	/// Storage: `PooledVaultRewards::RewardPerToken` (r:1 w:0)
	/// Proof: `PooledVaultRewards::RewardPerToken` (`max_values`: None, `max_size`: Some(140), added: 2615, mode: `MaxEncodedLen`)
	/// Storage: `PooledVaultRewards::RewardTally` (r:1 w:1)
	/// Proof: `PooledVaultRewards::RewardTally` (`max_values`: None, `max_size`: Some(264), added: 2739, mode: `MaxEncodedLen`)
	/// Storage: `PooledVaultRewards::TotalRewards` (r:1 w:1)
	/// Proof: `PooledVaultRewards::TotalRewards` (`max_values`: None, `max_size`: Some(78), added: 2553, mode: `MaxEncodedLen`)
	/// Storage: `VaultStaking::RewardPerToken` (r:1 w:1)
	/// Proof: `VaultStaking::RewardPerToken` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::Stake` (r:1 w:1)
	/// Proof: `VaultStaking::Stake` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::SlashPerToken` (r:1 w:0)
	/// Proof: `VaultStaking::SlashPerToken` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::SlashTally` (r:1 w:1)
	/// Proof: `VaultStaking::SlashTally` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VaultStaking::TotalStake` (r:1 w:1)
	/// Proof: `VaultStaking::TotalStake` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn cancel_replace() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3221`
		//  Estimated: `9161`
		// Minimum execution time: 119_000_000 picoseconds.
		Weight::from_parts(127_000_000, 9161)
			.saturating_add(T::DbWeight::get().reads(22_u64))
			.saturating_add(T::DbWeight::get().writes(10_u64))
	}
	/// Storage: `Replace::ReplacePeriod` (r:0 w:1)
	/// Proof: `Replace::ReplacePeriod` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_replace_period() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(7_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Replace::ReplaceMinimumTransferAmount` (r:0 w:1)
	/// Proof: `Replace::ReplaceMinimumTransferAmount` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn minimum_transfer_amount_update() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(7_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

//! Autogenerated weights for fee
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-09-11, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Gianfrancos-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("amplitude"), DB CACHE: 1024

// Executed Command:
// ../target/production/pendulum-node
// benchmark
// pallet
// --chain
// amplitude
// --wasm-execution=compiled
// --pallet
// fee
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ../runtime/amplitude/src/weights/fee.rs
// --template
// frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for fee using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> fee::WeightInfo for SubstrateWeight<T> {
	/// Storage: `Fee::IssueFee` (r:0 w:1)
	/// Proof: `Fee::IssueFee` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_issue_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_000_000 picoseconds.
		Weight::from_parts(4_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Fee::IssueGriefingCollateral` (r:0 w:1)
	/// Proof: `Fee::IssueGriefingCollateral` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_issue_griefing_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_000_000 picoseconds.
		Weight::from_parts(4_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Fee::RedeemFee` (r:0 w:1)
	/// Proof: `Fee::RedeemFee` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_redeem_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_000_000 picoseconds.
		Weight::from_parts(4_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Fee::PremiumRedeemFee` (r:0 w:1)
	/// Proof: `Fee::PremiumRedeemFee` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_premium_redeem_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_000_000 picoseconds.
		Weight::from_parts(3_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Fee::PunishmentFee` (r:0 w:1)
	/// Proof: `Fee::PunishmentFee` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_punishment_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_000_000 picoseconds.
		Weight::from_parts(4_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Fee::ReplaceGriefingCollateral` (r:0 w:1)
	/// Proof: `Fee::ReplaceGriefingCollateral` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_replace_griefing_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_000_000 picoseconds.
		Weight::from_parts(3_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
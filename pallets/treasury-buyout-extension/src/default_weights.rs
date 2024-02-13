
//! Autogenerated weights for treasury_buyout_extension
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Bogdans-M2-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("foucoco"), DB CACHE: 1024

// Executed Command:
// ./target/release/pendulum-node
// benchmark
// pallet
// --chain
// foucoco
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// treasury-buyout-extension
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/treasury-buyout-extension/src/default_weights.rs
// --template
// .maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for treasury_buyout_extension.
pub trait WeightInfo {
	fn buyout() -> Weight;
	fn update_buyout_limit() -> Weight;
	fn update_allowed_assets(n: u32, ) -> Weight;
}

/// Weights for treasury_buyout_extension using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: TreasuryBuyoutExtension AllowedCurrencies (r:1 w:0)
	/// Proof: TreasuryBuyoutExtension AllowedCurrencies (max_values: None, max_size: Some(62), added: 2537, mode: MaxEncodedLen)
	/// Storage: TreasuryBuyoutExtension BuyoutLimit (r:1 w:0)
	/// Proof: TreasuryBuyoutExtension BuyoutLimit (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: TreasuryBuyoutExtension Buyouts (r:1 w:1)
	/// Proof: TreasuryBuyoutExtension Buyouts (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(150), added: 2625, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Security ParachainStatus (r:0 w:1)
	/// Proof Skipped: Security ParachainStatus (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle OracleKeys (r:1 w:1)
	/// Proof Skipped: Oracle OracleKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn buyout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `863`
		//  Estimated: `26038`
		// Minimum execution time: 81_000_000 picoseconds.
		Weight::from_parts(83_000_000, 26038)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: TreasuryBuyoutExtension BuyoutLimit (r:0 w:1)
	/// Proof: TreasuryBuyoutExtension BuyoutLimit (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn update_buyout_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(8_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: TreasuryBuyoutExtension AllowedCurrencies (r:1 w:1)
	/// Proof: TreasuryBuyoutExtension AllowedCurrencies (max_values: None, max_size: Some(62), added: 2537, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 20]`.
	fn update_allowed_assets(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3527`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(15_935_593, 3527)
			// Standard Error: 3_669
			.saturating_add(Weight::from_parts(1_123_793, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: TreasuryBuyoutExtension AllowedCurrencies (r:1 w:0)
	/// Proof: TreasuryBuyoutExtension AllowedCurrencies (max_values: None, max_size: Some(62), added: 2537, mode: MaxEncodedLen)
	/// Storage: TreasuryBuyoutExtension BuyoutLimit (r:1 w:0)
	/// Proof: TreasuryBuyoutExtension BuyoutLimit (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: TreasuryBuyoutExtension Buyouts (r:1 w:1)
	/// Proof: TreasuryBuyoutExtension Buyouts (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(150), added: 2625, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Security ParachainStatus (r:0 w:1)
	/// Proof Skipped: Security ParachainStatus (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle OracleKeys (r:1 w:1)
	/// Proof Skipped: Oracle OracleKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn buyout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `863`
		//  Estimated: `26038`
		// Minimum execution time: 81_000_000 picoseconds.
		Weight::from_parts(83_000_000, 26038)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: TreasuryBuyoutExtension BuyoutLimit (r:0 w:1)
	/// Proof: TreasuryBuyoutExtension BuyoutLimit (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn update_buyout_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(8_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: TreasuryBuyoutExtension AllowedCurrencies (r:1 w:1)
	/// Proof: TreasuryBuyoutExtension AllowedCurrencies (max_values: None, max_size: Some(62), added: 2537, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 20]`.
	fn update_allowed_assets(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3527`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(15_935_593, 3527)
			// Standard Error: 3_669
			.saturating_add(Weight::from_parts(1_123_793, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
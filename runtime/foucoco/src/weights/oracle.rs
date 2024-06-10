
//! Autogenerated weights for oracle
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-04-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// oracle
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/foucoco/src/weights/
// --template
// .maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for oracle using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> oracle::WeightInfo for SubstrateWeight<T> {
	/// Storage: Timestamp Now (r:0 w:1)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Timestamp DidUpdate (r:0 w:1)
	/// Proof: Timestamp DidUpdate (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_000_000 picoseconds.
		Weight::from_parts(2_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Oracle OracleKeys (r:0 w:1)
	/// Proof Skipped: Oracle OracleKeys (max_values: Some(1), max_size: None, mode: Measured)
	fn update_oracle_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Oracle MaxDelay (r:0 w:1)
	/// Proof Skipped: Oracle MaxDelay (max_values: Some(1), max_size: None, mode: Measured)
	fn set_max_delay() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(10_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

//! Autogenerated weights for orml_asset_registry
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
// orml_asset_registry
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ../runtime/foucoco/src/weights/orml_asset_registry.rs
// --template
// frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for orml_asset_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_asset_registry::WeightInfo for SubstrateWeight<T> {
	/// Storage: `AssetRegistry::Metadata` (r:1 w:1)
	/// Proof: `AssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(897), added: 3372, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::LocationToAssetId` (r:1 w:1)
	/// Proof: `AssetRegistry::LocationToAssetId` (`max_values`: None, `max_size`: Some(656), added: 3131, mode: `MaxEncodedLen`)
	fn register_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `138`
		//  Estimated: `4362`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(16_000_000, 4362)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `AssetRegistry::Metadata` (r:1 w:1)
	/// Proof: `AssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(897), added: 3372, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::LocationToAssetId` (r:1 w:2)
	/// Proof: `AssetRegistry::LocationToAssetId` (`max_values`: None, `max_size`: Some(656), added: 3131, mode: `MaxEncodedLen`)
	fn update_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `801`
		//  Estimated: `4362`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(22_000_000, 4362)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn set_asset_location() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 0_000 picoseconds.
		Weight::from_parts(0, 0)
	}
}
// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Expose the auto generated weight files.

pub mod block_weights;
pub mod extrinsic_weights;
pub mod pallet_xcm;
pub mod paritydb_weights;
pub mod rocksdb_weights;
pub mod parachain_staking;
pub mod orml_asset_registry;
pub mod orml_currencies_allowance_extension;
pub mod orml_tokens_management_extension;
pub mod treasury_buyout_extension;
pub mod fee;
pub mod issue;
pub mod nomination;
pub mod oracle;
pub mod redeem;
pub mod replace;
pub mod stellar_relay;
pub mod vault_registry;

pub use block_weights::constants::BlockExecutionWeight;
pub use extrinsic_weights::constants::ExtrinsicBaseWeight;
pub use rocksdb_weights::constants::RocksDbWeight;

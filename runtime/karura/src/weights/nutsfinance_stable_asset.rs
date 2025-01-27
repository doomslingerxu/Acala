// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for nutsfinance_stable_asset
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-35-209`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for nutsfinance_stable_asset.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> nutsfinance_stable_asset::WeightInfo for WeightInfo<T> {
	// Storage: StableAsset PoolCount (r:1 w:1)
	// Proof Skipped: StableAsset PoolCount (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1312`
		//  Estimated: `11167`
		// Minimum execution time: 33_490 nanoseconds.
		Weight::from_parts(34_416_000, 11167)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	fn modify_a() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1505`
		//  Estimated: `4970`
		// Minimum execution time: 26_479 nanoseconds.
		Weight::from_parts(27_333_000, 4970)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	fn modify_fees() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1505`
		//  Estimated: `4970`
		// Minimum execution time: 25_592 nanoseconds.
		Weight::from_parts(26_214_000, 4970)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	fn modify_recipients() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1505`
		//  Estimated: `4970`
		// Minimum execution time: 25_205 nanoseconds.
		Weight::from_parts(25_924_000, 4970)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:10 w:10)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	/// The range of component `u` is `[2, 5]`.
	fn mint(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2010 + u * (238 ±0)`
		//  Estimated: `31290 + u * (7371 ±0)`
		// Minimum execution time: 153_643 nanoseconds.
		Weight::from_parts(83_710_313, 31290)
			// Standard Error: 255_432
			.saturating_add(Weight::from_parts(39_018_822, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 7371).saturating_mul(u.into()))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:6 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	/// The range of component `u` is `[2, 5]`.
	fn swap(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2202 + u * (236 ±0)`
		//  Estimated: `32182 + u * (4761 ±32)`
		// Minimum execution time: 1_499_866 nanoseconds.
		Weight::from_parts(143_703_049, 32182)
			// Standard Error: 4_316_544
			.saturating_add(Weight::from_parts(716_003_841, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(Weight::from_parts(0, 4761).saturating_mul(u.into()))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:10 w:10)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	/// The range of component `u` is `[2, 5]`.
	fn redeem_proportion(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2179 + u * (300 ±0)`
		//  Estimated: `29495 + u * (7687 ±0)`
		// Minimum execution time: 172_495 nanoseconds.
		Weight::from_parts(105_118_606, 29495)
			// Standard Error: 216_530
			.saturating_add(Weight::from_parts(37_613_057, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 7687).saturating_mul(u.into()))
	}
	// Storage: StableAsset Pools (r:1 w:0)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:0)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:0)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `u` is `[2, 5]`.
	fn redeem_single(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1592 + u * (227 ±0)`
		//  Estimated: `26267 + u * (1622 ±0)`
		// Minimum execution time: 983_391 nanoseconds.
		Weight::from_parts(464_129_247, 26267)
			// Standard Error: 942_806
			.saturating_add(Weight::from_parts(261_402_253, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 1622).saturating_mul(u.into()))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:10 w:10)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	/// The range of component `u` is `[2, 5]`.
	fn redeem_multi(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2179 + u * (300 ±0)`
		//  Estimated: `29495 + u * (7687 ±2)`
		// Minimum execution time: 151_266 nanoseconds.
		Weight::from_parts(90_953_279, 29495)
			// Standard Error: 231_772
			.saturating_add(Weight::from_parts(33_895_450, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 7687).saturating_mul(u.into()))
	}
}

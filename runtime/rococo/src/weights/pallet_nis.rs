// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_nis`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_nis
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_nis`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_nis::WeightInfo for WeightInfo<T> {
	/// Storage: Nis Queues (r:1 w:1)
	/// Proof: Nis Queues (max_values: None, max_size: Some(48022), added: 50497, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	/// Storage: Nis QueueTotals (r:1 w:1)
	/// Proof: Nis QueueTotals (max_values: Some(1), max_size: Some(6002), added: 6497, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 999]`.
	fn place_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6209 + l * (48 ±0)`
		//  Estimated: `51487`
		// Minimum execution time: 44_501_000 picoseconds.
		Weight::from_parts(44_545_106, 0)
			.saturating_add(Weight::from_parts(0, 51487))
			// Standard Error: 746
			.saturating_add(Weight::from_parts(71_836, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Nis Queues (r:1 w:1)
	/// Proof: Nis Queues (max_values: None, max_size: Some(48022), added: 50497, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	/// Storage: Nis QueueTotals (r:1 w:1)
	/// Proof: Nis QueueTotals (max_values: Some(1), max_size: Some(6002), added: 6497, mode: MaxEncodedLen)
	fn place_bid_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `54211`
		//  Estimated: `51487`
		// Minimum execution time: 127_017_000 picoseconds.
		Weight::from_parts(127_958_000, 0)
			.saturating_add(Weight::from_parts(0, 51487))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Nis Queues (r:1 w:1)
	/// Proof: Nis Queues (max_values: None, max_size: Some(48022), added: 50497, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	/// Storage: Nis QueueTotals (r:1 w:1)
	/// Proof: Nis QueueTotals (max_values: Some(1), max_size: Some(6002), added: 6497, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 1000]`.
	fn retract_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6209 + l * (48 ±0)`
		//  Estimated: `51487`
		// Minimum execution time: 47_921_000 picoseconds.
		Weight::from_parts(41_920_873, 0)
			.saturating_add(Weight::from_parts(0, 51487))
			// Standard Error: 722
			.saturating_add(Weight::from_parts(89_227, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Nis Summary (r:1 w:0)
	/// Proof: Nis Summary (max_values: Some(1), max_size: Some(40), added: 535, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn fund_deficit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `225`
		//  Estimated: `3593`
		// Minimum execution time: 38_180_000 picoseconds.
		Weight::from_parts(38_547_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nis Receipts (r:1 w:1)
	/// Proof: Nis Receipts (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Nis Summary (r:1 w:1)
	/// Proof: Nis Summary (max_values: Some(1), max_size: Some(40), added: 535, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances Account (r:1 w:1)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	fn communify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `469`
		//  Estimated: `3593`
		// Minimum execution time: 69_770_000 picoseconds.
		Weight::from_parts(70_201_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Nis Receipts (r:1 w:1)
	/// Proof: Nis Receipts (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Nis Summary (r:1 w:1)
	/// Proof: Nis Summary (max_values: Some(1), max_size: Some(40), added: 535, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances Account (r:1 w:1)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	fn privatize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `659`
		//  Estimated: `3593`
		// Minimum execution time: 86_145_000 picoseconds.
		Weight::from_parts(86_942_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Nis Receipts (r:1 w:1)
	/// Proof: Nis Receipts (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Nis Summary (r:1 w:1)
	/// Proof: Nis Summary (max_values: Some(1), max_size: Some(40), added: 535, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	fn thaw_private() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `387`
		//  Estimated: `3593`
		// Minimum execution time: 47_421_000 picoseconds.
		Weight::from_parts(47_960_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Nis Receipts (r:1 w:1)
	/// Proof: Nis Receipts (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Nis Summary (r:1 w:1)
	/// Proof: Nis Summary (max_values: Some(1), max_size: Some(40), added: 535, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances Account (r:1 w:1)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn thaw_communal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `604`
		//  Estimated: `3593`
		// Minimum execution time: 90_144_000 picoseconds.
		Weight::from_parts(91_018_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Nis Summary (r:1 w:1)
	/// Proof: Nis Summary (max_values: Some(1), max_size: Some(40), added: 535, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Nis QueueTotals (r:1 w:1)
	/// Proof: Nis QueueTotals (max_values: Some(1), max_size: Some(6002), added: 6497, mode: MaxEncodedLen)
	fn process_queues() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6658`
		//  Estimated: `7487`
		// Minimum execution time: 21_240_000 picoseconds.
		Weight::from_parts(21_613_000, 0)
			.saturating_add(Weight::from_parts(0, 7487))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nis Queues (r:1 w:1)
	/// Proof: Nis Queues (max_values: None, max_size: Some(48022), added: 50497, mode: MaxEncodedLen)
	fn process_queue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `51487`
		// Minimum execution time: 4_848_000 picoseconds.
		Weight::from_parts(5_021_000, 0)
			.saturating_add(Weight::from_parts(0, 51487))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nis Receipts (r:0 w:1)
	/// Proof: Nis Receipts (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	fn process_bid() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_080_000 picoseconds.
		Weight::from_parts(7_252_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

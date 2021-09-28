// This file is part of Acala.

// Copyright (C) 2020-2021 Acala Foundation.
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

//! Autogenerated weights for module_homa_lite
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-28, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 128

// Executed Command:
// target/release/acala
// benchmark
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=module-homa-lite
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

/// Weight functions for module_homa_lite.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_homa_lite::WeightInfo for WeightInfo<T> {
	fn on_idle() -> Weight {
		(0 as Weight)
	}
	fn mint() -> Weight {
		(129_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(14 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn mint_for_requests() -> Weight {
		(332_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(30 as Weight))
			.saturating_add(T::DbWeight::get().writes(21 as Weight))
	}
	fn set_total_staking_currency() -> Weight {
		(10_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn adjust_total_staking_currency() -> Weight {
		(12_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_minting_cap() -> Weight {
		(10_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_xcm_base_weight() -> Weight {
		(10_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn request_redeem() -> Weight {
		(69_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn schedule_unbond() -> Weight {
		(12_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn replace_schedule_unbond() -> Weight {
		(10_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

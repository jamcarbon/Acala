// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
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

//! Autogenerated weights for module_dex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_dex.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_dex::WeightInfo for WeightInfo<T> {
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	fn enable_trading_pair() -> Weight {
		(14_102_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	fn disable_trading_pair() -> Weight {
		(14_381_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Dex ProvisioningPool (r:1 w:0)
	fn list_provisioning() -> Weight {
		(20_603_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	fn update_provisioning_parameters() -> Weight {
		(8_443_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Dex InitialShareExchangeRates (r:0 w:1)
	fn end_provisioning() -> Weight {
		(41_347_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Storage: Dex ProvisioningPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	fn add_provision() -> Weight {
		(73_596_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex ProvisioningPool (r:2 w:1)
	// Storage: Dex InitialShareExchangeRates (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn claim_dex_share() -> Weight {
		(61_406_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn add_liquidity() -> Weight {
		(90_285_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	fn add_liquidity_and_stake() -> Weight {
		(124_460_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: System Account (r:1 w:1)
	fn remove_liquidity() -> Weight {
		(101_471_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: System Account (r:2 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn remove_liquidity_by_unstake() -> Weight {
		(136_571_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		(51_512_000 as Weight)
			// Standard Error: 130_000
			.saturating_add((9_294_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(u as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(u as Weight)))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	fn swap_with_exact_target(u: u32, ) -> Weight {
		(50_306_000 as Weight)
			// Standard Error: 102_000
			.saturating_add((9_717_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(u as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(u as Weight)))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex InitialShareExchangeRates (r:1 w:0)
	// Storage: Dex ProvisioningPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn refund_provision() -> Weight {
		(72_589_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	fn abort_provisioning() -> Weight {
		(18_831_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

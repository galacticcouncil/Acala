//! Benchmarks for the honzon module.
// This is separated into its own crate due to cyclic dependency issues.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "runtime-benchmarks", test))]
mod mock;

use sp_std::prelude::*;
use sp_std::vec;

use frame_benchmarking::{account, benchmarks};
use frame_support::traits::Get;
use frame_system::RawOrigin;
use sp_runtime::{
	traits::{StaticLookup, UniqueSaturatedInto},
	FixedPointNumber,
};

use cdp_engine::Module as CdpEngine;
use cdp_engine::*;
use dex::Module as Dex;
use orml_traits::{Change, DataFeeder, MultiCurrencyExtended};
use primitives::{Amount, Balance, CurrencyId, TokenSymbol};
use support::{DEXManager, Price, Rate, Ratio};

pub struct Module<T: Config>(cdp_engine::Module<T>);

pub trait Config:
	cdp_engine::Config
	+ orml_oracle::Config<orml_oracle::Instance1>
	+ prices::Config
	+ dex::Config
	+ emergency_shutdown::Config
{
}

const SEED: u32 = 0;

fn dollar(d: u32) -> Balance {
	let d: Balance = d.into();
	d.saturating_mul(1_000_000_000_000_000_000)
}

fn feed_price<T: Config>(currency_id: CurrencyId, price: Price) -> Result<(), &'static str> {
	let oracle_operators = orml_oracle::Module::<T, orml_oracle::Instance1>::members().0;
	for operator in oracle_operators {
		<T as prices::Config>::Source::feed_value(operator.clone(), currency_id, price)?;
	}
	Ok(())
}

fn inject_liquidity<T: Config>(
	maker: T::AccountId,
	currency_id: CurrencyId,
	max_amount: Balance,
	max_other_currency_amount: Balance,
) -> Result<(), &'static str> {
	let base_currency_id = <T as cdp_engine::Config>::GetStableCurrencyId::get();

	// set balance
	<T as dex::Config>::Currency::update_balance(currency_id, &maker, max_amount.unique_saturated_into())?;
	<T as dex::Config>::Currency::update_balance(
		base_currency_id,
		&maker,
		max_other_currency_amount.unique_saturated_into(),
	)?;

	Dex::<T>::add_liquidity(
		RawOrigin::Signed(maker).into(),
		base_currency_id,
		currency_id,
		max_amount,
		max_other_currency_amount,
		false,
	)?;

	Ok(())
}

fn emergency_shutdown<T: Config>() -> frame_support::dispatch::DispatchResultWithPostInfo {
	emergency_shutdown::Module::<T>::emergency_shutdown(RawOrigin::Root.into())
}

benchmarks! {
	set_collateral_params {
		let u in 0 .. 1000;
	}: _(
		RawOrigin::Root,
		CurrencyId::Token(TokenSymbol::DOT),
		Change::NewValue(Some(Rate::saturating_from_rational(1, 1000000))),
		Change::NewValue(Some(Ratio::saturating_from_rational(150, 100))),
		Change::NewValue(Some(Rate::saturating_from_rational(20, 100))),
		Change::NewValue(Some(Ratio::saturating_from_rational(180, 100))),
		Change::NewValue(dollar(100000))
	)

	set_global_params {
		let u in 0 .. 1000;
	}: _(RawOrigin::Root, Rate::saturating_from_rational(1, 1000000))

	// `liquidate` by_auction
	liquidate_by_auction {
		let u in 0 .. 1000;

		let owner: T::AccountId = account("owner", u, SEED);
		let owner_lookup = T::Lookup::unlookup(owner.clone());
		let currency_id: CurrencyId = <T as cdp_engine::Config>::CollateralCurrencyIds::get()[0];
		let min_debit_value = <T as cdp_engine::Config>::MinimumDebitValue::get();
		let debit_exchange_rate = CdpEngine::<T>::get_debit_exchange_rate(currency_id);
		let collateral_price = Price::one();		// 1 USD
		let min_debit_amount = debit_exchange_rate.reciprocal().unwrap().saturating_mul_int(min_debit_value);
		let min_debit_amount: Amount = min_debit_amount.unique_saturated_into();
		let collateral_amount = (min_debit_value * 2).unique_saturated_into();

		// set balance
		<T as loans::Config>::Currency::update_balance(currency_id, &owner, collateral_amount)?;

		// feed price
		feed_price::<T>(currency_id, collateral_price)?;

		// set risk params
		CdpEngine::<T>::set_collateral_params(
			RawOrigin::Root.into(),
			currency_id,
			Change::NoChange,
			Change::NewValue(Some(Ratio::saturating_from_rational(150, 100))),
			Change::NewValue(Some(Rate::saturating_from_rational(10, 100))),
			Change::NewValue(Some(Ratio::saturating_from_rational(150, 100))),
			Change::NewValue(min_debit_value * 100),
		)?;

		// adjust position
		CdpEngine::<T>::adjust_position(&owner, currency_id, collateral_amount, min_debit_amount)?;

		// modify liquidation rate to make the cdp unsafe
		CdpEngine::<T>::set_collateral_params(
			RawOrigin::Root.into(),
			currency_id,
			Change::NoChange,
			Change::NewValue(Some(Ratio::saturating_from_rational(1000, 100))),
			Change::NoChange,
			Change::NoChange,
			Change::NoChange,
		)?;
	}: liquidate(RawOrigin::None, currency_id, owner_lookup)

	// `liquidate` by dex
	liquidate_by_dex {
		let u in 0 .. 1000;

		let owner: T::AccountId = account("owner", u, SEED);
		let owner_lookup = T::Lookup::unlookup(owner.clone());
		let funder: T::AccountId = account("funder", u, SEED);
		let currency_id: CurrencyId = <T as cdp_engine::Config>::CollateralCurrencyIds::get()[0];
		let base_currency_id: CurrencyId = <T as cdp_engine::Config>::GetStableCurrencyId::get();
		let min_debit_value = <T as cdp_engine::Config>::MinimumDebitValue::get();
		let debit_exchange_rate = CdpEngine::<T>::get_debit_exchange_rate(currency_id);
		let collateral_price = Price::one();		// 1 USD
		let min_debit_amount = debit_exchange_rate.reciprocal().unwrap().saturating_mul_int(min_debit_value);
		let min_debit_amount: Amount = min_debit_amount.unique_saturated_into();
		let collateral_amount = (min_debit_value * 2).unique_saturated_into();

		let max_slippage_swap_with_dex = <T as cdp_engine::Config>::MaxSlippageSwapWithDEX::get();
		let collateral_amount_in_dex = max_slippage_swap_with_dex.reciprocal().unwrap().saturating_mul_int(min_debit_value * 10);
		let base_amount_in_dex = collateral_amount_in_dex * 2;

		inject_liquidity::<T>(funder, currency_id, base_amount_in_dex, collateral_amount_in_dex)?;

		// set balance
		<T as loans::Config>::Currency::update_balance(currency_id, &owner, collateral_amount)?;

		// feed price
		feed_price::<T>(currency_id, collateral_price)?;

		// set risk params
		CdpEngine::<T>::set_collateral_params(
			RawOrigin::Root.into(),
			currency_id,
			Change::NoChange,
			Change::NewValue(Some(Ratio::saturating_from_rational(150, 100))),
			Change::NewValue(Some(Rate::saturating_from_rational(10, 100))),
			Change::NewValue(Some(Ratio::saturating_from_rational(150, 100))),
			Change::NewValue(min_debit_value * 100),
		)?;

		// adjust position
		CdpEngine::<T>::adjust_position(&owner, currency_id, collateral_amount, min_debit_amount)?;

		// modify liquidation rate to make the cdp unsafe
		CdpEngine::<T>::set_collateral_params(
			RawOrigin::Root.into(),
			currency_id,
			Change::NoChange,
			Change::NewValue(Some(Ratio::saturating_from_rational(1000, 100))),
			Change::NoChange,
			Change::NoChange,
			Change::NoChange,
		)?;
	}: liquidate(RawOrigin::None, currency_id, owner_lookup)
	verify {
		let (other_currency_amount, base_currency_amount) = Dex::<T>::get_liquidity_pool(base_currency_id, currency_id);
		assert!(other_currency_amount > collateral_amount_in_dex);
		assert!(base_currency_amount < base_amount_in_dex);
	}

	settle {
		let u in 0 .. 1000;

		let owner: T::AccountId = account("owner", u, SEED);
		let owner_lookup = T::Lookup::unlookup(owner.clone());
		let currency_id: CurrencyId = <T as cdp_engine::Config>::CollateralCurrencyIds::get()[0];
		let min_debit_value = <T as cdp_engine::Config>::MinimumDebitValue::get();
		let debit_exchange_rate = CdpEngine::<T>::get_debit_exchange_rate(currency_id);
		let collateral_price = Price::one();		// 1 USD
		let min_debit_amount = debit_exchange_rate.reciprocal().unwrap().saturating_mul_int(min_debit_value);
		let min_debit_amount: Amount = min_debit_amount.unique_saturated_into();
		let collateral_amount = (min_debit_value * 2).unique_saturated_into();

		// set balance
		<T as loans::Config>::Currency::update_balance(currency_id, &owner, collateral_amount)?;

		// feed price
		feed_price::<T>(currency_id, collateral_price)?;

		// set risk params
		CdpEngine::<T>::set_collateral_params(
			RawOrigin::Root.into(),
			currency_id,
			Change::NoChange,
			Change::NewValue(Some(Ratio::saturating_from_rational(150, 100))),
			Change::NewValue(Some(Rate::saturating_from_rational(10, 100))),
			Change::NewValue(Some(Ratio::saturating_from_rational(150, 100))),
			Change::NewValue(min_debit_value * 100),
		)?;

		// adjust position
		CdpEngine::<T>::adjust_position(&owner, currency_id, collateral_amount, min_debit_amount)?;

		// shutdown
		emergency_shutdown::<T>()?;
	}: _(RawOrigin::None, currency_id, owner_lookup)
}

#[cfg(all(feature = "runtime-benchmarks", test))]
mod tests {
	use super::*;
	use crate::mock::{new_test_ext, Runtime};
	use frame_support::assert_ok;

	#[test]
	fn set_collateral_params() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_set_collateral_params::<Runtime>());
		});
	}

	#[test]
	fn set_global_params() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_set_global_params::<Runtime>());
		});
	}

	#[test]
	fn liquidate_by_auction() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_liquidate_by_auction::<Runtime>());
		});
	}

	#[test]
	fn liquidate_by_dex() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_liquidate_by_dex::<Runtime>());
		});
	}

	#[test]
	fn settle() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_settle::<Runtime>());
		});
	}
}

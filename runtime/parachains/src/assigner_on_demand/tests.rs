// Copyright 2023 Parity Technologies (UK) Ltd.
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

use super::*;
use crate::{
	assigner_on_demand::Error,
	mock::{
		new_test_ext, MockGenesisConfig, OnDemandAssigner, Paras, ParasShared, RuntimeOrigin,
		Scheduler, System, Test,
	},
	paras::{ParaGenesisArgs, ParaKind},
};
use frame_support::{assert_noop, assert_ok, error::BadOrigin};
use keyring::Sr25519Keyring;
use primitives::{Balance, BlockNumber, CollatorId, SessionIndex};
use sp_std::collections::btree_map::BTreeMap;

fn default_genesis_config() -> MockGenesisConfig {
	MockGenesisConfig {
		configuration: crate::configuration::GenesisConfig {
			config: crate::configuration::HostConfiguration { ..Default::default() },
		},
		..Default::default()
	}
}

fn schedule_blank_para(id: ParaId, parakind: ParaKind) {
	assert_ok!(Paras::schedule_para_initialize(
		id,
		ParaGenesisArgs {
			genesis_head: Vec::new().into(),
			validation_code: vec![1, 2, 3].into(),
			para_kind: parakind,
		}
	));
}

fn run_to_block(
	to: BlockNumber,
	new_session: impl Fn(BlockNumber) -> Option<SessionChangeNotification<BlockNumber>>,
) {
	while System::block_number() < to {
		let b = System::block_number();

		Scheduler::initializer_finalize();
		Paras::initializer_finalize(b);

		if let Some(notification) = new_session(b + 1) {
			let mut notification_with_session_index = notification;
			// We will make every session change trigger an action queue. Normally this may require 2 or more session changes.
			if notification_with_session_index.session_index == SessionIndex::default() {
				notification_with_session_index.session_index = ParasShared::scheduled_session();
			}
			Paras::initializer_on_new_session(&notification_with_session_index);
			Scheduler::initializer_on_new_session(&notification_with_session_index);
		}

		System::on_finalize(b);

		System::on_initialize(b + 1);
		System::set_block_number(b + 1);

		Paras::initializer_initialize(b + 1);
		Scheduler::initializer_initialize(b + 1);

		// In the real runtime this is expected to be called by the `InclusionInherent` pallet.
		Scheduler::update_claimqueue(BTreeMap::new(), b + 1);
	}
}

#[derive(Debug)]
pub(super) struct GenesisConfigBuilder {
	parathread_cores: u32,
	on_demand_base_fee: Balance,
	on_demand_fee_variability: Perbill,
	on_demand_max_queue_size: u32,
	on_demand_target_queue_utilization: Perbill,
}

impl Default for GenesisConfigBuilder {
	fn default() -> Self {
		Self {
			parathread_cores: 10,
			on_demand_base_fee: 10_000,
			on_demand_fee_variability: Perbill::from_percent(1),
			on_demand_max_queue_size: 100,
			on_demand_target_queue_utilization: Perbill::from_percent(25),
		}
	}
}

impl GenesisConfigBuilder {
	pub(super) fn build(self) -> crate::mock::MockGenesisConfig {
		let mut genesis = default_genesis_config();
		let config = &mut genesis.configuration.config;
		config.parathread_cores = self.parathread_cores;
		config.on_demand_base_fee = self.on_demand_base_fee;
		config.on_demand_fee_variability = self.on_demand_fee_variability;
		config.on_demand_queue_max_size = self.on_demand_max_queue_size;
		config.on_demand_target_queue_utilization = self.on_demand_target_queue_utilization;
		genesis
	}
}

#[test]
fn spot_traffic_capacity_zero_returns_none() {
	let res = OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from(u128::MAX),
		0u32,
		u32::MAX,
		Perbill::from_percent(100),
		Perbill::from_percent(1),
	);
	assert_eq!(res, None)
}

#[test]
fn spot_traffic_queue_size_larger_than_capacity_returns_none() {
	let res = OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from(u128::MAX),
		1u32,
		2u32,
		Perbill::from_percent(100),
		Perbill::from_percent(1),
	);
	assert_eq!(res, None)
}

#[test]
fn spot_traffic_calculation_identity() {
	let res = OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from_u32(1),
		1000,
		100,
		Perbill::from_percent(10),
		Perbill::from_percent(3),
	);
	assert_eq!(res.unwrap(), FixedU128::from_u32(1))
}

#[test]
fn spot_traffic_calculation_u32_max() {
	let res = OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from_u32(1),
		u32::MAX,
		u32::MAX,
		Perbill::from_percent(100),
		Perbill::from_percent(3),
	);
	assert_eq!(res.unwrap(), FixedU128::from_u32(1))
}

#[test]
fn spot_traffic_calculation_u32_traffic_max() {
	let res = OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from(u128::MAX),
		u32::MAX,
		u32::MAX,
		Perbill::from_percent(1),
		Perbill::from_percent(1),
	);
	assert_eq!(res, None)
}

#[test]
fn sustained_target_increases_spot_traffic() {
	let mut traffic = FixedU128::from_u32(1u32);
	for _ in 0..50 {
		traffic = OnDemandAssigner::calculate_spot_traffic(
			traffic,
			100,
			12,
			Perbill::from_percent(10),
			Perbill::from_percent(100),
		)
		.unwrap()
	}
	assert_eq!(traffic, FixedU128::from_inner(2_718_103_312_071_174_051u128))
}

#[test]
fn place_order_works() {
	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		let alice = 1u64;
		let amt = 10_000_000u128;
		let para_a = ParaId::from(111);
		let collator = CollatorId::from(Sr25519Keyring::Alice.public());

		// Initialize the parathread and wait for it to be ready.
		schedule_blank_para(para_a, ParaKind::Parathread);

		assert!(!Paras::is_parathread(para_a));

		run_to_block(10, |n| if n == 10 { Some(Default::default()) } else { None });

		assert!(Paras::is_parathread(para_a));

		// Does not work unsigned
		assert_noop!(
			OnDemandAssigner::place_order(RuntimeOrigin::none(), amt, para_a, collator.clone()),
			BadOrigin
		);

		// Does not work with max_amount lower than fee
		let low_max_amt = 1u128;
		assert_noop!(
			OnDemandAssigner::place_order(
				RuntimeOrigin::signed(alice),
				low_max_amt,
				para_a,
				collator.clone()
			),
			Error::<Test>::SpotPriceHigherThanMaxAmount,
		);
	});
}

#[test]
fn parathreadclaimqueue_add_claim_works() {
	let para_a = ParaId::from(111);
	let collator = CollatorId::from(Sr25519Keyring::Alice.public());
	//let mut queue = OnDemandQueue::get();
	let n_parathread_cores = 10;

	// Queue defaults to empty and the next offset is also empty
	//assert!(queue.len() == 0);

	//queue.enqueue_claim(SpotClaim { para_id: para_a, collator_id: collator });

	//assert!(queue.len() == 1);
}

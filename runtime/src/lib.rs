// This file is part of Darwinia.
//
// Copyright (C) 2018-2021 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

pub mod constants {
	// --- substrate ---
	use sp_staking::SessionIndex;
	// --- darwinia ---
	use crate::*;

	pub const NANO: Balance = 1;
	pub const MICRO: Balance = 1_000 * NANO;
	pub const MILLI: Balance = 1_000 * MICRO;
	pub const COIN: Balance = 1_000 * MILLI;

	pub const CAP: Balance = 10_000_000_000 * COIN;
	pub const TOTAL_POWER: Power = 1_000_000_000;

	pub const MILLISECS_PER_BLOCK: Moment = 6000;
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
	pub const BLOCKS_PER_SESSION: BlockNumber = 10 * MINUTES;
	pub const SESSIONS_PER_ERA: SessionIndex = 6;

	// Time is measured by number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = 60 * MINUTES;
	pub const DAYS: BlockNumber = 24 * HOURS;

	// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 20 * COIN + (bytes as Balance) * 100 * MICRO
	}
}

// pub mod impls {
// 	//! Some configurable implementations as associated type for the substrate runtime.

// 	pub mod relay {
// 		// --- darwinia ---
// 		use crate::*;
// 		use darwinia_relay_primitives::relayer_game::*;
// 		use ethereum_primitives::EthereumBlockNumber;

// 		pub struct EthereumRelayerGameAdjustor;
// 		impl AdjustableRelayerGame for EthereumRelayerGameAdjustor {
// 			type Moment = BlockNumber;
// 			type Balance = Balance;
// 			type RelayHeaderId = EthereumBlockNumber;

// 			fn max_active_games() -> u8 {
// 				32
// 			}

// 			fn affirm_time(round: u32) -> Self::Moment {
// 				match round {
// 					// 1.5 mins
// 					0 => 30,
// 					// 0.5 mins
// 					_ => 6,
// 				}
// 			}

// 			fn complete_proofs_time(round: u32) -> Self::Moment {
// 				match round {
// 					// 1.5 mins
// 					0 => 30,
// 					// 0.5 mins
// 					_ => 6,
// 				}
// 			}

// 			fn update_sample_points(sample_points: &mut Vec<Vec<Self::RelayHeaderId>>) {
// 				sample_points.push(vec![sample_points.last().unwrap().last().unwrap() - 1]);
// 			}

// 			fn estimate_stake(round: u32, affirmations_count: u32) -> Self::Balance {
// 				match round {
// 					0 => match affirmations_count {
// 						0 => 1000 * COIN,
// 						_ => 1500 * COIN,
// 					},
// 					_ => 100 * COIN,
// 				}
// 			}
// 		}
// 	}

// 	// --- crates ---
// 	use smallvec::smallvec;
// 	// --- substrate ---
// 	use frame_support::{
// 		traits::{Currency, Imbalance, OnUnbalanced},
// 		weights::{WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial},
// 	};
// 	// --- darwinia ---
// 	use crate::*;

// 	darwinia_support::impl_account_data! {
// 		struct AccountData<Balance>
// 		for
// 			RingInstance,
// 			KtonInstance
// 		where
// 			Balance = Balance
// 		{
// 			// other data
// 		}
// 	}

// 	pub struct Author;
// 	impl OnUnbalanced<NegativeImbalance> for Author {
// 		fn on_nonzero_unbalanced(amount: NegativeImbalance) {
// 			Ring::resolve_creating(&Authorship::author(), amount);
// 		}
// 	}

// 	pub struct DealWithFees;
// 	impl OnUnbalanced<NegativeImbalance> for DealWithFees {
// 		fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
// 			if let Some(fees) = fees_then_tips.next() {
// 				// for fees, 80% to treasury, 20% to author
// 				let mut split = fees.ration(80, 20);
// 				if let Some(tips) = fees_then_tips.next() {
// 					// for tips, if any, 80% to treasury, 20% to author (though this can be anything)
// 					tips.ration_merge_into(80, 20, &mut split);
// 				}
// 				Treasury::on_unbalanced(split.0);
// 				Author::on_unbalanced(split.1);
// 			}
// 		}
// 	}

// 	/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
// 	/// node's balance type.
// 	///
// 	/// This should typically create a mapping between the following ranges:
// 	///   - [0, MAXIMUM_BLOCK_WEIGHT]
// 	///   - [Balance::min, Balance::max]
// 	///
// 	/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
// 	///   - Setting it to `0` will essentially disable the weight fee.
// 	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
// 	pub struct WeightToFee;
// 	impl WeightToFeePolynomial for WeightToFee {
// 		type Balance = Balance;
// 		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
// 			// in Crab, extrinsic base weight (smallest non-zero weight) is mapped to 100 MILLI:
// 			let p = 100 * MILLI;
// 			let q = Balance::from(ExtrinsicBaseWeight::get());
// 			smallvec![WeightToFeeCoefficient {
// 				degree: 1,
// 				negative: false,
// 				coeff_frac: Perbill::from_rational_approximation(p % q, q),
// 				coeff_integer: p / q,
// 			}]
// 		}
// 	}
// }

pub mod wasm {
	//! Make the WASM binary available.

	#[cfg(all(feature = "std", any(target_arch = "x86_64", target_arch = "x86")))]
	include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

	#[cfg(all(feature = "std", not(any(target_arch = "x86_64", target_arch = "x86"))))]
	pub const WASM_BINARY: &[u8] = include_bytes!("../wasm/darwinia_pc2_runtime.compact.wasm");
	#[cfg(all(feature = "std", not(any(target_arch = "x86_64", target_arch = "x86"))))]
	pub const WASM_BINARY_BLOATY: &[u8] = include_bytes!("../wasm/darwinia_pc2_runtime.wasm");

	#[cfg(feature = "std")]
	/// Wasm binary unwrapped. If built with `BUILD_DUMMY_WASM_BINARY`, the function panics.
	pub fn wasm_binary_unwrap() -> &'static [u8] {
		#[cfg(all(feature = "std", any(target_arch = "x86_64", target_arch = "x86")))]
		return WASM_BINARY.expect(
			"Development wasm binary is not available. This means the client is \
							built with `BUILD_DUMMY_WASM_BINARY` flag and it is only usable for \
							production chains. Please rebuild with the flag disabled.",
		);
		#[cfg(all(feature = "std", not(any(target_arch = "x86_64", target_arch = "x86"))))]
		return WASM_BINARY;
	}
}

pub mod system;
pub use system::*;

pub mod timestamp;
pub use timestamp::*;

pub mod balances;
pub use balances::*;

pub mod transaction_payment;
pub use transaction_payment::*;

pub mod sudo;
pub use sudo::*;

pub mod parachain_system;
pub use parachain_system::*;

pub mod parachain_info_;
pub use parachain_info_::*;

pub mod xcm_handler;
pub use xcm_handler::*;

// --- darwinia ---
use constants::*;
use darwinia_pc2_primitives::*;
pub use wasm::*;

// --- substrate ---
use frame_support::{
	traits::Randomness,
	weights::{constants::WEIGHT_PER_SECOND, Weight},
};
use sp_api::impl_runtime_apis;
use sp_core::OpaqueMetadata;
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{Block as BlockT, IdentityLookup},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, Perbill,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

/// The address format for describing accounts.
pub type Address = AccountId;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllModules,
>;

type Ring = Balances;

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("Darwinia PC2"),
	impl_name: create_runtime_str!("Darwinia PC2"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
};

/// We assume that ~10% of the block weight is consumed by `on_initalize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used
/// by  Operational  extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 2 seconds of compute with a 6 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = 2 * WEIGHT_PER_SECOND;

impl_opaque_keys! {
	pub struct SessionKeys {}
}

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

frame_support::parameter_types! {
	pub const ExistentialDeposit: Balance = 500;
	pub const MaxLocks: u32 = 50;
}
impl pallet_balances::Config for Runtime {
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = MaxLocks;
}

frame_support::construct_runtime! {
	pub enum Runtime
	where
		Block = Block,
		NodeBlock = OpaqueBlock,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		// Basic stuff; balances is uncallable initially.
		System: frame_system::{Module, Call, Storage, Config, Event<T>} = 0,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage} = 1,

		Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent} = 2,
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>} = 3,
		// Kton: darwinia_balances::<Instance1>::{Module, Call, Storage, Config<T>, Event<T>} = 4,
		TransactionPayment: pallet_transaction_payment::{Module, Storage} = 5,

		// Governance stuff; uncallable initially.
		// Democracy: darwinia_democracy::{Module, Call, Storage, Config, Event<T>} = 6,
		// Council: pallet_collective::<Instance0>::{Module, Call, Storage, Origin<T>, Config<T>, Event<T>} = 7,
		// TechnicalCommittee: pallet_collective::<Instance1>::{Module, Call, Storage, Origin<T>, Config<T>, Event<T>} = 8,
		// ElectionsPhragmen: darwinia_elections_phragmen::{Module, Call, Storage, Config<T>, Event<T>} = 9,
		// TechnicalMembership: pallet_membership::<Instance0>::{Module, Call, Storage, Config<T>, Event<T>} = 10,
		// Treasury: darwinia_treasury::{Module, Call, Storage, Event<T>} = 11,

		Sudo: pallet_sudo::{Module, Call, Storage, Config<T>, Event<T>} = 12,

		// Claims. Usable initially.
		// Claims: darwinia_claims::{Module, Call, Storage, Config, Event<T>, ValidateUnsigned} = 13,

		// Vesting. Usable initially, but removed once all vesting is finished.
		// Vesting: darwinia_vesting::{Module, Call, Storage, Event<T>, Config<T>} = 14,

		// Utility module.
		// Utility: pallet_utility::{Module, Call, Event} = 15,

		// Less simple identity module.
		// Identity: pallet_identity::{Module, Call, Storage, Event<T>} = 16,

		// Society module.
		// Society: pallet_society::{Module, Call, Storage, Event<T>} = 17,

		// Social recovery module.
		// Recovery: pallet_recovery::{Module, Call, Storage, Event<T>} = 18,

		// Proxy module. Late addition.
		// Proxy: pallet_proxy::{Module, Call, Storage, Event<T>} = 19,

		// Multisig module. Late addition.
		// Multisig: pallet_multisig::{Module, Call, Storage, Event<T>} = 20,

		// HeaderMMR: darwinia_header_mmr::{Module, Call, Storage} = 21,

		// CrabIssuing: darwinia_crab_issuing::{Module, Call, Storage, Config, Event<T>} = 22,
		// CrabBacking: darwinia_crab_backing::{Module, Storage, Config<T>} = 23,

		// EthereumRelay: darwinia_ethereum_relay::{Module, Call, Storage, Config<T>, Event<T>} = 24,
		// EthereumBacking: darwinia_ethereum_backing::{Module, Call, Storage, Config<T>, Event<T>} = 25,
		// EthereumRelayerGame: darwinia_relayer_game::<Instance0>::{Module, Storage} = 26,
		// EthereumRelayAuthorities: darwinia_relay_authorities::<Instance0>::{Module, Call, Storage, Config<T>, Event<T>} = 27,

		// TronBacking: darwinia_tron_backing::{Module, Storage, Config<T>} = 28,

		ParachainSystem: cumulus_pallet_parachain_system::{Module, Call, Storage, Inherent, Event} = 29,
		ParachainInfo: parachain_info::{Module, Storage, Config} = 30,
		XcmHandler: cumulus_pallet_xcm_handler::{Module, Call, Event<T>, Origin} = 31,
	}
}

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			Runtime::metadata().into()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(
			extrinsic: <Block as BlockT>::Extrinsic,
		) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: sp_inherents::InherentData) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			RandomnessCollectiveFlip::random_seed()
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}

		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}
	}
}

cumulus_pallet_parachain_system::register_validate_block!(Runtime, Executive);

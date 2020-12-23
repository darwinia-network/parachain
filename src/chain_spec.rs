// This file is part of Darwinia.
//
// Copyright (C) 2018-2020 Darwinia Network
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

// --- crates ---
use serde::{Deserialize, Serialize};
// --- substrate ---
use cumulus_primitives::ParaId;
// --- substrate ---
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
// --- darwinia ---
use pangolin_primitives::{AccountId, Signature};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<pangolin_runtime::GenesisConfig, Extensions>;

type AccountPublic = <Signature as Verify>::Signer;

const PANGOLIN_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}
impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &Box<dyn sc_service::ChainSpec>) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn properties() -> Properties {
	let mut properties = Properties::new();

	properties.insert("ss58Format".into(), 42.into());
	properties.insert("tokenDecimals".into(), 9.into());
	properties.insert("tokenSymbol".into(), "PRING".into());
	properties.insert("ktonTokenDecimals".into(), 9.into());
	properties.insert("ktonTokenSymbol".into(), "PKTON".into());

	properties
}

pub fn pangolin_build_spec_config_of(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		"Pangolin",
		"Pangolin",
		ChainType::Live,
		move || pangolin_build_spec_genesis(id),
		vec![],
		Some(
			TelemetryEndpoints::new(vec![(PANGOLIN_TELEMETRY_URL.to_string(), 0)])
				.expect("Pangolin telemetry url is valid; qed"),
		),
		// None,
		None,
		Some(properties()),
		// None,
		Extensions {
			relay_chain: "rococo".into(),
			para_id: id.into(),
		},
	)
}

fn pangolin_build_spec_genesis(id: ParaId) -> pangolin_runtime::GenesisConfig {
	const ROOT: &'static str = "0x72819fbc1b93196fa230243947c1726cbea7e33044c7eb6f736ff345561f9e4c";

	let root = AccountId::from(array_bytes::bytes_array_unchecked!(ROOT, 32));
	let endowed_accounts = vec![(root.clone(), 1 << 56)];

	pangolin_runtime::GenesisConfig {
		frame_system: Some(pangolin_runtime::SystemConfig {
			code: pangolin_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(pangolin_runtime::BalancesConfig {
			balances: endowed_accounts,
		}),
		pallet_sudo: Some(pangolin_runtime::SudoConfig { key: root }),
		parachain_info: Some(pangolin_runtime::ParachainInfoConfig { parachain_id: id }),
	}
}

pub fn pangolin_development_config_of(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		"Pangolin",
		"Pangolin",
		ChainType::Development,
		move || {
			pangolin_development_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				id,
			)
		},
		vec![],
		None,
		None,
		Some(properties()),
		// None,
		Extensions {
			relay_chain: "rococo".into(),
			para_id: id.into(),
		},
	)
}

fn pangolin_development_genesis(
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> pangolin_runtime::GenesisConfig {
	pangolin_runtime::GenesisConfig {
		frame_system: Some(pangolin_runtime::SystemConfig {
			code: pangolin_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(pangolin_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		}),
		pallet_sudo: Some(pangolin_runtime::SudoConfig { key: root_key }),
		parachain_info: Some(pangolin_runtime::ParachainInfoConfig { parachain_id: id }),
	}
}
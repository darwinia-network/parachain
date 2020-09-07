// --- crates ---
use serde::{Deserialize, Serialize};
// --- substrate ---
use cumulus_primitives::ParaId;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
// --- darwinia ---
use array_bytes::fixed_hex_bytes_unchecked;
use parachain_runtime::{types::*, wasm::*, GenesisConfig as DarwiniaPC1GenesisConfig};

pub type DarwiniaParachainChainSpec =
	sc_service::GenericChainSpec<DarwiniaPC1GenesisConfig, Extensions>;

const DARWINIA_PARACHAIN_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "dar";

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

pub fn darwinia_parachain_config() -> Result<DarwiniaParachainChainSpec, String> {
	DarwiniaParachainChainSpec::from_json_bytes(&include_bytes!("../res/darwinia-pc1.json")[..])
}

pub fn darwinia_parachain_build_spec_genesis(id: ParaId) -> DarwiniaPC1GenesisConfig {
	let root_key: AccountId = fixed_hex_bytes_unchecked!(
		"0x469823d00af3dd2907d7b87210375ece08691d811c396f396d06a657d1db6b58",
		32
	)
	.into();

	DarwiniaPC1GenesisConfig {
		frame_system: Some(parachain_runtime::SystemConfig {
			code: wasm_binary_unwrap().to_vec(),
			..Default::default()
		}),
		darwinia_balances_Instance0: Some(parachain_runtime::BalancesConfig {
			balances: vec![(root_key.clone(), 1 << 60)],
		}),
		darwinia_balances_Instance1: Some(parachain_runtime::KtonConfig {
			balances: vec![(root_key.clone(), 1 << 60)],
		}),
		pallet_sudo: Some(parachain_runtime::SudoConfig {
			key: root_key.clone(),
		}),
		parachain_info: Some(parachain_runtime::ParachainInfoConfig { parachain_id: id }),
		orml_tokens: Some(parachain_runtime::TokensConfig {
			endowed_accounts: vec![(
				root_key.clone(),
				parachain_runtime::types::CurrencyId::XRing,
				1 << 60,
			)],
		}),
		pallet_balances: Some(parachain_runtime::DARConfig {
			balances: vec![(root_key, 1 << 60)],
		}),
	}
}

pub fn darwinia_parachain_build_spec_config(id: ParaId) -> DarwiniaParachainChainSpec {
	DarwiniaParachainChainSpec::from_genesis(
		"Darwinia PC1",
		"Darwinia PC1",
		ChainType::Live,
		move || darwinia_parachain_build_spec_genesis(id),
		vec![],
		Some(
			TelemetryEndpoints::new(vec![(DARWINIA_PARACHAIN_TELEMETRY_URL.to_string(), 0)])
				.expect("Darwinia PC1 telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		Some(darwinia_parachain_properties()),
		Extensions {
			relay_chain: "darwinia_parachain".into(),
			para_id: id.into(),
		},
	)
}

pub fn darwinia_parachain_properties() -> Properties {
	let mut properties = Properties::new();

	properties.insert("ss58Format".into(), 18.into());
	properties.insert("tokenDecimals".into(), 9.into());
	properties.insert("tokenSymbol".into(), "RING".into());
	properties.insert("ktonTokenDecimals".into(), 9.into());
	properties.insert("ktonTokenSymbol".into(), "KTON".into());

	properties
}

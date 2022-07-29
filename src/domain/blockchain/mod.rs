mod starknet;
pub use self::starknet::Chain as StarknetChain;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Network {
	Starknet(StarknetChain),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventFilter {
	pub contract_address: String,
	pub event_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmartContractEvent;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockchainReorg;

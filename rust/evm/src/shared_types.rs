///`Function(string,address,address,string,string,bytes32,address,uint256,uint256,string,uint256,uint8)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Function {
    pub name: ::std::string::String,
    pub authority: ::ethers::core::types::Address,
    pub quote_address: ::ethers::core::types::Address,
    pub container_registry: ::std::string::String,
    pub container: ::std::string::String,
    pub version: [u8; 32],
    pub queue_address: ::ethers::core::types::Address,
    pub last_execution_timestamp: ::ethers::core::types::U256,
    pub next_allowed_timestamp: ::ethers::core::types::U256,
    pub schedule: ::std::string::String,
    pub balance: ::ethers::core::types::U256,
    pub status: u8,
}

pub use i_switchboard::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_switchboard {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"aggregatorAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentIntervalId\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"aggregatorAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"round\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getIntervalResult\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"medianTimestamp\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"aggregatorAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"latestResult\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static ISWITCHBOARD_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct ISwitchboard<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISwitchboard<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISwitchboard<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISwitchboard<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISwitchboard<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ISwitchboard)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISwitchboard<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISWITCHBOARD_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getCurrentIntervalId` (0x1dc1da86) function
        pub fn get_current_interval_id(
            &self,
            aggregator_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([29, 193, 218, 134], aggregator_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getIntervalResult` (0x3d24ef6e) function
        pub fn get_interval_result(
            &self,
            aggregator_address: ::ethers::core::types::Address,
            round: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([61, 36, 239, 110], (aggregator_address, round))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestResult` (0xfab005a2) function
        pub fn latest_result(
            &self,
            aggregator_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::I256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([250, 176, 5, 162], aggregator_address)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ISwitchboard<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getCurrentIntervalId` function with signature `getCurrentIntervalId(address)` and selector `0x1dc1da86`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getCurrentIntervalId", abi = "getCurrentIntervalId(address)")]
    pub struct GetCurrentIntervalIdCall {
        pub aggregator_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getIntervalResult` function with signature `getIntervalResult(address,uint80)` and selector `0x3d24ef6e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getIntervalResult", abi = "getIntervalResult(address,uint80)")]
    pub struct GetIntervalResultCall {
        pub aggregator_address: ::ethers::core::types::Address,
        pub round: u128,
    }
    ///Container type for all input parameters for the `latestResult` function with signature `latestResult(address)` and selector `0xfab005a2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "latestResult", abi = "latestResult(address)")]
    pub struct LatestResultCall {
        pub aggregator_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISwitchboardCalls {
        GetCurrentIntervalId(GetCurrentIntervalIdCall),
        GetIntervalResult(GetIntervalResultCall),
        LatestResult(LatestResultCall),
    }
    impl ::ethers::core::abi::AbiDecode for ISwitchboardCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetCurrentIntervalIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetCurrentIntervalId(decoded));
            }
            if let Ok(decoded)
                = <GetIntervalResultCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetIntervalResult(decoded));
            }
            if let Ok(decoded)
                = <LatestResultCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LatestResult(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISwitchboardCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetCurrentIntervalId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetIntervalResult(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestResult(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ISwitchboardCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetCurrentIntervalId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetIntervalResult(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestResult(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetCurrentIntervalIdCall> for ISwitchboardCalls {
        fn from(value: GetCurrentIntervalIdCall) -> Self {
            Self::GetCurrentIntervalId(value)
        }
    }
    impl ::core::convert::From<GetIntervalResultCall> for ISwitchboardCalls {
        fn from(value: GetIntervalResultCall) -> Self {
            Self::GetIntervalResult(value)
        }
    }
    impl ::core::convert::From<LatestResultCall> for ISwitchboardCalls {
        fn from(value: LatestResultCall) -> Self {
            Self::LatestResult(value)
        }
    }
    ///Container type for all return fields from the `getCurrentIntervalId` function with signature `getCurrentIntervalId(address)` and selector `0x1dc1da86`
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
    pub struct GetCurrentIntervalIdReturn(pub u128);
    ///Container type for all return fields from the `getIntervalResult` function with signature `getIntervalResult(address,uint80)` and selector `0x3d24ef6e`
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
    pub struct GetIntervalResultReturn {
        pub value: ::ethers::core::types::I256,
        pub timestamp: ::ethers::core::types::U256,
        pub median_timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `latestResult` function with signature `latestResult(address)` and selector `0xfab005a2`
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
    pub struct LatestResultReturn {
        pub value: ::ethers::core::types::I256,
        pub timestamp: ::ethers::core::types::U256,
    }
}

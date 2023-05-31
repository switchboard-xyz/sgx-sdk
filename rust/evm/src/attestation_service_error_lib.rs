pub use attestation_service_error_lib::*;
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
pub mod attestation_service_error_lib {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"AttestationQueueAlreadyExists\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AttestationQueueDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FunctionDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InsufficientNodes\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"QuoteAlreadyExists\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"QuoteDoesNotExist\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ATTESTATIONSERVICEERRORLIB_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        86,
        96,
        55,
        96,
        11,
        130,
        130,
        130,
        57,
        128,
        81,
        96,
        0,
        26,
        96,
        115,
        20,
        96,
        42,
        87,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        0,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        48,
        96,
        0,
        82,
        96,
        115,
        129,
        83,
        130,
        129,
        243,
        254,
        115,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
        253,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        171,
        179,
        12,
        230,
        40,
        99,
        56,
        184,
        132,
        219,
        46,
        111,
        250,
        185,
        152,
        134,
        189,
        126,
        137,
        102,
        229,
        207,
        247,
        7,
        166,
        103,
        13,
        235,
        7,
        110,
        219,
        14,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static ATTESTATIONSERVICEERRORLIB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        115,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
        253,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        171,
        179,
        12,
        230,
        40,
        99,
        56,
        184,
        132,
        219,
        46,
        111,
        250,
        185,
        152,
        134,
        189,
        126,
        137,
        102,
        229,
        207,
        247,
        7,
        166,
        103,
        13,
        235,
        7,
        110,
        219,
        14,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static ATTESTATIONSERVICEERRORLIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AttestationServiceErrorLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AttestationServiceErrorLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AttestationServiceErrorLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AttestationServiceErrorLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AttestationServiceErrorLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(AttestationServiceErrorLib))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AttestationServiceErrorLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ATTESTATIONSERVICEERRORLIB_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ATTESTATIONSERVICEERRORLIB_ABI.clone(),
                ATTESTATIONSERVICEERRORLIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AttestationServiceErrorLib<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AttestationQueueAlreadyExists` with signature `AttestationQueueAlreadyExists()` and selector `0x94f62ae9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "AttestationQueueAlreadyExists",
        abi = "AttestationQueueAlreadyExists()"
    )]
    pub struct AttestationQueueAlreadyExists;
    ///Custom Error type `AttestationQueueDoesNotExist` with signature `AttestationQueueDoesNotExist()` and selector `0xbfb18a77`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "AttestationQueueDoesNotExist",
        abi = "AttestationQueueDoesNotExist()"
    )]
    pub struct AttestationQueueDoesNotExist;
    ///Custom Error type `FunctionDoesNotExist` with signature `FunctionDoesNotExist()` and selector `0xa9ad62f8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FunctionDoesNotExist", abi = "FunctionDoesNotExist()")]
    pub struct FunctionDoesNotExist;
    ///Custom Error type `InsufficientNodes` with signature `InsufficientNodes()` and selector `0xc79a6baa`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InsufficientNodes", abi = "InsufficientNodes()")]
    pub struct InsufficientNodes;
    ///Custom Error type `QuoteAlreadyExists` with signature `QuoteAlreadyExists()` and selector `0x01ac5a44`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "QuoteAlreadyExists", abi = "QuoteAlreadyExists()")]
    pub struct QuoteAlreadyExists;
    ///Custom Error type `QuoteDoesNotExist` with signature `QuoteDoesNotExist()` and selector `0xd483c43f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "QuoteDoesNotExist", abi = "QuoteDoesNotExist()")]
    pub struct QuoteDoesNotExist;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AttestationServiceErrorLibErrors {
        AttestationQueueAlreadyExists(AttestationQueueAlreadyExists),
        AttestationQueueDoesNotExist(AttestationQueueDoesNotExist),
        FunctionDoesNotExist(FunctionDoesNotExist),
        InsufficientNodes(InsufficientNodes),
        QuoteAlreadyExists(QuoteAlreadyExists),
        QuoteDoesNotExist(QuoteDoesNotExist),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AttestationServiceErrorLibErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <AttestationQueueAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AttestationQueueAlreadyExists(decoded));
            }
            if let Ok(decoded)
                = <AttestationQueueDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AttestationQueueDoesNotExist(decoded));
            }
            if let Ok(decoded)
                = <FunctionDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FunctionDoesNotExist(decoded));
            }
            if let Ok(decoded)
                = <InsufficientNodes as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InsufficientNodes(decoded));
            }
            if let Ok(decoded)
                = <QuoteAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QuoteAlreadyExists(decoded));
            }
            if let Ok(decoded)
                = <QuoteDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QuoteDoesNotExist(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AttestationServiceErrorLibErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AttestationQueueAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationQueueDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FunctionDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientNodes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AttestationServiceErrorLibErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AttestationQueueAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AttestationQueueDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FunctionDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientNodes as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <QuoteAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <QuoteDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AttestationServiceErrorLibErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttestationQueueAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationQueueDoesNotExist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FunctionDoesNotExist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientNodes(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuoteDoesNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for AttestationServiceErrorLibErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AttestationQueueAlreadyExists>
    for AttestationServiceErrorLibErrors {
        fn from(value: AttestationQueueAlreadyExists) -> Self {
            Self::AttestationQueueAlreadyExists(value)
        }
    }
    impl ::core::convert::From<AttestationQueueDoesNotExist>
    for AttestationServiceErrorLibErrors {
        fn from(value: AttestationQueueDoesNotExist) -> Self {
            Self::AttestationQueueDoesNotExist(value)
        }
    }
    impl ::core::convert::From<FunctionDoesNotExist>
    for AttestationServiceErrorLibErrors {
        fn from(value: FunctionDoesNotExist) -> Self {
            Self::FunctionDoesNotExist(value)
        }
    }
    impl ::core::convert::From<InsufficientNodes> for AttestationServiceErrorLibErrors {
        fn from(value: InsufficientNodes) -> Self {
            Self::InsufficientNodes(value)
        }
    }
    impl ::core::convert::From<QuoteAlreadyExists> for AttestationServiceErrorLibErrors {
        fn from(value: QuoteAlreadyExists) -> Self {
            Self::QuoteAlreadyExists(value)
        }
    }
    impl ::core::convert::From<QuoteDoesNotExist> for AttestationServiceErrorLibErrors {
        fn from(value: QuoteDoesNotExist) -> Self {
            Self::QuoteDoesNotExist(value)
        }
    }
}

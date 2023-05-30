pub use i_switchboard_attestation_service::*;
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
pub mod i_switchboard_attestation_service {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authority\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"accountAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AttestationQueueAccountInit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authority\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"accountAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FunctionAccountInit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"functionAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"funder\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FunctionFundEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authority\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"accountAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"QuoteAccountInit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"nodeAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"queue\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"QuoteGC\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"nodeAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"authority\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"QuoteHeartbeat\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"nodeAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"quoteAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"QuotePayoutEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"queueAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"verifiee\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"QuoteVerifyRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"quoteAuthority\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getQuoteEnclaveMeasurement\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authorityNode\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"permission\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasPermission\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authorityNode\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"permission\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"on\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPermission\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"quoteAuthority\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"validate\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static ISWITCHBOARDATTESTATIONSERVICE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct ISwitchboardAttestationService<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISwitchboardAttestationService<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISwitchboardAttestationService<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISwitchboardAttestationService<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISwitchboardAttestationService<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ISwitchboardAttestationService))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISwitchboardAttestationService<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISWITCHBOARDATTESTATIONSERVICE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getQuoteEnclaveMeasurement` (0x3e55bfd0) function
        pub fn get_quote_enclave_measurement(
            &self,
            quote_authority: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([62, 85, 191, 208], quote_authority)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasPermission` (0x8b01813d) function
        pub fn has_permission(
            &self,
            authority_node: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            permission: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([139, 1, 129, 61], (authority_node, recipient, permission))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPermission` (0xf551f042) function
        pub fn set_permission(
            &self,
            authority_node: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            permission: ::ethers::core::types::U256,
            on: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [245, 81, 240, 66],
                    (authority_node, recipient, permission, on),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validate` (0x207c64fb) function
        pub fn validate(
            &self,
            quote_authority: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([32, 124, 100, 251], quote_authority)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AttestationQueueAccountInit` event
        pub fn attestation_queue_account_init_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AttestationQueueAccountInitFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FunctionAccountInit` event
        pub fn function_account_init_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FunctionAccountInitFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FunctionFundEvent` event
        pub fn function_fund_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FunctionFundEventFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `QuoteAccountInit` event
        pub fn quote_account_init_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QuoteAccountInitFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `QuoteGC` event
        pub fn quote_gc_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, QuoteGCFilter> {
            self.0.event()
        }
        ///Gets the contract's `QuoteHeartbeat` event
        pub fn quote_heartbeat_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QuoteHeartbeatFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `QuotePayoutEvent` event
        pub fn quote_payout_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QuotePayoutEventFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `QuoteVerifyRequest` event
        pub fn quote_verify_request_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QuoteVerifyRequestFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ISwitchboardAttestationServiceEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ISwitchboardAttestationService<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AttestationQueueAccountInit",
        abi = "AttestationQueueAccountInit(address,address)"
    )]
    pub struct AttestationQueueAccountInitFilter {
        #[ethevent(indexed)]
        pub authority: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "FunctionAccountInit",
        abi = "FunctionAccountInit(address,address)"
    )]
    pub struct FunctionAccountInitFilter {
        #[ethevent(indexed)]
        pub authority: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "FunctionFundEvent",
        abi = "FunctionFundEvent(address,address,uint256)"
    )]
    pub struct FunctionFundEventFilter {
        #[ethevent(indexed)]
        pub function_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub funder: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "QuoteAccountInit", abi = "QuoteAccountInit(address,address)")]
    pub struct QuoteAccountInitFilter {
        #[ethevent(indexed)]
        pub authority: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "QuoteGC", abi = "QuoteGC(address,address)")]
    pub struct QuoteGCFilter {
        #[ethevent(indexed)]
        pub node_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub queue: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "QuoteHeartbeat", abi = "QuoteHeartbeat(address,address)")]
    pub struct QuoteHeartbeatFilter {
        #[ethevent(indexed)]
        pub node_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub authority: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "QuotePayoutEvent",
        abi = "QuotePayoutEvent(address,address,uint256)"
    )]
    pub struct QuotePayoutEventFilter {
        #[ethevent(indexed)]
        pub node_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "QuoteVerifyRequest",
        abi = "QuoteVerifyRequest(address,address,address)"
    )]
    pub struct QuoteVerifyRequestFilter {
        #[ethevent(indexed)]
        pub queue_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub verifier: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub verifiee: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISwitchboardAttestationServiceEvents {
        AttestationQueueAccountInitFilter(AttestationQueueAccountInitFilter),
        FunctionAccountInitFilter(FunctionAccountInitFilter),
        FunctionFundEventFilter(FunctionFundEventFilter),
        QuoteAccountInitFilter(QuoteAccountInitFilter),
        QuoteGCFilter(QuoteGCFilter),
        QuoteHeartbeatFilter(QuoteHeartbeatFilter),
        QuotePayoutEventFilter(QuotePayoutEventFilter),
        QuoteVerifyRequestFilter(QuoteVerifyRequestFilter),
    }
    impl ::ethers::contract::EthLogDecode for ISwitchboardAttestationServiceEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AttestationQueueAccountInitFilter::decode_log(log) {
                return Ok(
                    ISwitchboardAttestationServiceEvents::AttestationQueueAccountInitFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = FunctionAccountInitFilter::decode_log(log) {
                return Ok(
                    ISwitchboardAttestationServiceEvents::FunctionAccountInitFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = FunctionFundEventFilter::decode_log(log) {
                return Ok(
                    ISwitchboardAttestationServiceEvents::FunctionFundEventFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = QuoteAccountInitFilter::decode_log(log) {
                return Ok(
                    ISwitchboardAttestationServiceEvents::QuoteAccountInitFilter(decoded),
                );
            }
            if let Ok(decoded) = QuoteGCFilter::decode_log(log) {
                return Ok(ISwitchboardAttestationServiceEvents::QuoteGCFilter(decoded));
            }
            if let Ok(decoded) = QuoteHeartbeatFilter::decode_log(log) {
                return Ok(
                    ISwitchboardAttestationServiceEvents::QuoteHeartbeatFilter(decoded),
                );
            }
            if let Ok(decoded) = QuotePayoutEventFilter::decode_log(log) {
                return Ok(
                    ISwitchboardAttestationServiceEvents::QuotePayoutEventFilter(decoded),
                );
            }
            if let Ok(decoded) = QuoteVerifyRequestFilter::decode_log(log) {
                return Ok(
                    ISwitchboardAttestationServiceEvents::QuoteVerifyRequestFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ISwitchboardAttestationServiceEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttestationQueueAccountInitFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FunctionAccountInitFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FunctionFundEventFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuoteAccountInitFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuoteGCFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteHeartbeatFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuotePayoutEventFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuoteVerifyRequestFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AttestationQueueAccountInitFilter>
    for ISwitchboardAttestationServiceEvents {
        fn from(value: AttestationQueueAccountInitFilter) -> Self {
            Self::AttestationQueueAccountInitFilter(value)
        }
    }
    impl ::core::convert::From<FunctionAccountInitFilter>
    for ISwitchboardAttestationServiceEvents {
        fn from(value: FunctionAccountInitFilter) -> Self {
            Self::FunctionAccountInitFilter(value)
        }
    }
    impl ::core::convert::From<FunctionFundEventFilter>
    for ISwitchboardAttestationServiceEvents {
        fn from(value: FunctionFundEventFilter) -> Self {
            Self::FunctionFundEventFilter(value)
        }
    }
    impl ::core::convert::From<QuoteAccountInitFilter>
    for ISwitchboardAttestationServiceEvents {
        fn from(value: QuoteAccountInitFilter) -> Self {
            Self::QuoteAccountInitFilter(value)
        }
    }
    impl ::core::convert::From<QuoteGCFilter> for ISwitchboardAttestationServiceEvents {
        fn from(value: QuoteGCFilter) -> Self {
            Self::QuoteGCFilter(value)
        }
    }
    impl ::core::convert::From<QuoteHeartbeatFilter>
    for ISwitchboardAttestationServiceEvents {
        fn from(value: QuoteHeartbeatFilter) -> Self {
            Self::QuoteHeartbeatFilter(value)
        }
    }
    impl ::core::convert::From<QuotePayoutEventFilter>
    for ISwitchboardAttestationServiceEvents {
        fn from(value: QuotePayoutEventFilter) -> Self {
            Self::QuotePayoutEventFilter(value)
        }
    }
    impl ::core::convert::From<QuoteVerifyRequestFilter>
    for ISwitchboardAttestationServiceEvents {
        fn from(value: QuoteVerifyRequestFilter) -> Self {
            Self::QuoteVerifyRequestFilter(value)
        }
    }
    ///Container type for all input parameters for the `getQuoteEnclaveMeasurement` function with signature `getQuoteEnclaveMeasurement(address)` and selector `0x3e55bfd0`
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
    #[ethcall(
        name = "getQuoteEnclaveMeasurement",
        abi = "getQuoteEnclaveMeasurement(address)"
    )]
    pub struct GetQuoteEnclaveMeasurementCall {
        pub quote_authority: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasPermission` function with signature `hasPermission(address,address,uint256)` and selector `0x8b01813d`
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
    #[ethcall(name = "hasPermission", abi = "hasPermission(address,address,uint256)")]
    pub struct HasPermissionCall {
        pub authority_node: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub permission: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPermission` function with signature `setPermission(address,address,uint256,bool)` and selector `0xf551f042`
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
    #[ethcall(
        name = "setPermission",
        abi = "setPermission(address,address,uint256,bool)"
    )]
    pub struct SetPermissionCall {
        pub authority_node: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub permission: ::ethers::core::types::U256,
        pub on: bool,
    }
    ///Container type for all input parameters for the `validate` function with signature `validate(address)` and selector `0x207c64fb`
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
    #[ethcall(name = "validate", abi = "validate(address)")]
    pub struct ValidateCall {
        pub quote_authority: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISwitchboardAttestationServiceCalls {
        GetQuoteEnclaveMeasurement(GetQuoteEnclaveMeasurementCall),
        HasPermission(HasPermissionCall),
        SetPermission(SetPermissionCall),
        Validate(ValidateCall),
    }
    impl ::ethers::core::abi::AbiDecode for ISwitchboardAttestationServiceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetQuoteEnclaveMeasurementCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetQuoteEnclaveMeasurement(decoded));
            }
            if let Ok(decoded)
                = <HasPermissionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasPermission(decoded));
            }
            if let Ok(decoded)
                = <SetPermissionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPermission(decoded));
            }
            if let Ok(decoded)
                = <ValidateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Validate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISwitchboardAttestationServiceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetQuoteEnclaveMeasurement(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasPermission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPermission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Validate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ISwitchboardAttestationServiceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetQuoteEnclaveMeasurement(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasPermission(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPermission(element) => ::core::fmt::Display::fmt(element, f),
                Self::Validate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetQuoteEnclaveMeasurementCall>
    for ISwitchboardAttestationServiceCalls {
        fn from(value: GetQuoteEnclaveMeasurementCall) -> Self {
            Self::GetQuoteEnclaveMeasurement(value)
        }
    }
    impl ::core::convert::From<HasPermissionCall>
    for ISwitchboardAttestationServiceCalls {
        fn from(value: HasPermissionCall) -> Self {
            Self::HasPermission(value)
        }
    }
    impl ::core::convert::From<SetPermissionCall>
    for ISwitchboardAttestationServiceCalls {
        fn from(value: SetPermissionCall) -> Self {
            Self::SetPermission(value)
        }
    }
    impl ::core::convert::From<ValidateCall> for ISwitchboardAttestationServiceCalls {
        fn from(value: ValidateCall) -> Self {
            Self::Validate(value)
        }
    }
    ///Container type for all return fields from the `getQuoteEnclaveMeasurement` function with signature `getQuoteEnclaveMeasurement(address)` and selector `0x3e55bfd0`
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
    pub struct GetQuoteEnclaveMeasurementReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasPermission` function with signature `hasPermission(address,address,uint256)` and selector `0x8b01813d`
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
    pub struct HasPermissionReturn(pub bool);
    ///Container type for all return fields from the `validate` function with signature `validate(address)` and selector `0x207c64fb`
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
    pub struct ValidateReturn(pub ::ethers::core::types::Address);
}

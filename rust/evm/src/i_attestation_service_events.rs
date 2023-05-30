pub use i_attestation_service_events::*;
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
pub mod i_attestation_service_events {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authority\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"accountAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AttestationQueueAccountInit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authority\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"accountAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FunctionAccountInit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"functionAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"funder\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FunctionFundEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authority\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"accountAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"QuoteAccountInit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"nodeAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"queue\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"QuoteGC\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"nodeAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"authority\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"QuoteHeartbeat\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"nodeAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"quoteAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"QuotePayoutEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"queueAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"verifier\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"verifiee\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"QuoteVerifyRequest\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static IATTESTATIONSERVICEEVENTS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IAttestationServiceEvents<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IAttestationServiceEvents<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IAttestationServiceEvents<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IAttestationServiceEvents<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IAttestationServiceEvents<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IAttestationServiceEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IAttestationServiceEvents<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IATTESTATIONSERVICEEVENTS_ABI.clone(),
                    client,
                ),
            )
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
            IAttestationServiceEventsEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IAttestationServiceEvents<M> {
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
    pub enum IAttestationServiceEventsEvents {
        AttestationQueueAccountInitFilter(AttestationQueueAccountInitFilter),
        FunctionAccountInitFilter(FunctionAccountInitFilter),
        FunctionFundEventFilter(FunctionFundEventFilter),
        QuoteAccountInitFilter(QuoteAccountInitFilter),
        QuoteGCFilter(QuoteGCFilter),
        QuoteHeartbeatFilter(QuoteHeartbeatFilter),
        QuotePayoutEventFilter(QuotePayoutEventFilter),
        QuoteVerifyRequestFilter(QuoteVerifyRequestFilter),
    }
    impl ::ethers::contract::EthLogDecode for IAttestationServiceEventsEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AttestationQueueAccountInitFilter::decode_log(log) {
                return Ok(
                    IAttestationServiceEventsEvents::AttestationQueueAccountInitFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = FunctionAccountInitFilter::decode_log(log) {
                return Ok(
                    IAttestationServiceEventsEvents::FunctionAccountInitFilter(decoded),
                );
            }
            if let Ok(decoded) = FunctionFundEventFilter::decode_log(log) {
                return Ok(
                    IAttestationServiceEventsEvents::FunctionFundEventFilter(decoded),
                );
            }
            if let Ok(decoded) = QuoteAccountInitFilter::decode_log(log) {
                return Ok(
                    IAttestationServiceEventsEvents::QuoteAccountInitFilter(decoded),
                );
            }
            if let Ok(decoded) = QuoteGCFilter::decode_log(log) {
                return Ok(IAttestationServiceEventsEvents::QuoteGCFilter(decoded));
            }
            if let Ok(decoded) = QuoteHeartbeatFilter::decode_log(log) {
                return Ok(
                    IAttestationServiceEventsEvents::QuoteHeartbeatFilter(decoded),
                );
            }
            if let Ok(decoded) = QuotePayoutEventFilter::decode_log(log) {
                return Ok(
                    IAttestationServiceEventsEvents::QuotePayoutEventFilter(decoded),
                );
            }
            if let Ok(decoded) = QuoteVerifyRequestFilter::decode_log(log) {
                return Ok(
                    IAttestationServiceEventsEvents::QuoteVerifyRequestFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IAttestationServiceEventsEvents {
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
    for IAttestationServiceEventsEvents {
        fn from(value: AttestationQueueAccountInitFilter) -> Self {
            Self::AttestationQueueAccountInitFilter(value)
        }
    }
    impl ::core::convert::From<FunctionAccountInitFilter>
    for IAttestationServiceEventsEvents {
        fn from(value: FunctionAccountInitFilter) -> Self {
            Self::FunctionAccountInitFilter(value)
        }
    }
    impl ::core::convert::From<FunctionFundEventFilter>
    for IAttestationServiceEventsEvents {
        fn from(value: FunctionFundEventFilter) -> Self {
            Self::FunctionFundEventFilter(value)
        }
    }
    impl ::core::convert::From<QuoteAccountInitFilter>
    for IAttestationServiceEventsEvents {
        fn from(value: QuoteAccountInitFilter) -> Self {
            Self::QuoteAccountInitFilter(value)
        }
    }
    impl ::core::convert::From<QuoteGCFilter> for IAttestationServiceEventsEvents {
        fn from(value: QuoteGCFilter) -> Self {
            Self::QuoteGCFilter(value)
        }
    }
    impl ::core::convert::From<QuoteHeartbeatFilter>
    for IAttestationServiceEventsEvents {
        fn from(value: QuoteHeartbeatFilter) -> Self {
            Self::QuoteHeartbeatFilter(value)
        }
    }
    impl ::core::convert::From<QuotePayoutEventFilter>
    for IAttestationServiceEventsEvents {
        fn from(value: QuotePayoutEventFilter) -> Self {
            Self::QuotePayoutEventFilter(value)
        }
    }
    impl ::core::convert::From<QuoteVerifyRequestFilter>
    for IAttestationServiceEventsEvents {
        fn from(value: QuoteVerifyRequestFilter) -> Self {
            Self::QuoteVerifyRequestFilter(value)
        }
    }
}

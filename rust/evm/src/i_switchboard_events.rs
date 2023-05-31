pub use i_switchboard_events::*;
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
pub mod i_switchboard_events {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authority\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"accountAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AggregatorAccountInit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"aggregatorAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"funder\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AggregatorFundEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"aggregatorAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"intervalId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"balanceLeftForInterval\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AggregatorIntervalRefreshed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"aggregatorAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"varianceThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"minJobResults\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"forceReportPeriod\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AggregatorResponseSettingsUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"aggregatorAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AggregatorSaveResult\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"aggregatorAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AggregatorUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authority\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"accountAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OracleAccountInit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oracleAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"aggregatorAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OraclePayoutEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authority\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"accountAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OracleQueueAccountInit\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static ISWITCHBOARDEVENTS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct ISwitchboardEvents<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISwitchboardEvents<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISwitchboardEvents<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISwitchboardEvents<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISwitchboardEvents<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ISwitchboardEvents)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISwitchboardEvents<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISWITCHBOARDEVENTS_ABI.clone(),
                    client,
                ),
            )
        }
        ///Gets the contract's `AggregatorAccountInit` event
        pub fn aggregator_account_init_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AggregatorAccountInitFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AggregatorFundEvent` event
        pub fn aggregator_fund_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AggregatorFundEventFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AggregatorIntervalRefreshed` event
        pub fn aggregator_interval_refreshed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AggregatorIntervalRefreshedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AggregatorResponseSettingsUpdate` event
        pub fn aggregator_response_settings_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AggregatorResponseSettingsUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AggregatorSaveResult` event
        pub fn aggregator_save_result_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AggregatorSaveResultFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AggregatorUpdate` event
        pub fn aggregator_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AggregatorUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OracleAccountInit` event
        pub fn oracle_account_init_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OracleAccountInitFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OraclePayoutEvent` event
        pub fn oracle_payout_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OraclePayoutEventFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OracleQueueAccountInit` event
        pub fn oracle_queue_account_init_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OracleQueueAccountInitFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ISwitchboardEventsEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ISwitchboardEvents<M> {
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
        name = "AggregatorAccountInit",
        abi = "AggregatorAccountInit(address,address,uint256)"
    )]
    pub struct AggregatorAccountInitFilter {
        #[ethevent(indexed)]
        pub authority: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account_address: ::ethers::core::types::Address,
        pub timestamp: ::ethers::core::types::U256,
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
        name = "AggregatorFundEvent",
        abi = "AggregatorFundEvent(address,address,uint256)"
    )]
    pub struct AggregatorFundEventFilter {
        #[ethevent(indexed)]
        pub aggregator_address: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "AggregatorIntervalRefreshed",
        abi = "AggregatorIntervalRefreshed(address,uint256,uint256)"
    )]
    pub struct AggregatorIntervalRefreshedFilter {
        #[ethevent(indexed)]
        pub aggregator_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub interval_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub balance_left_for_interval: ::ethers::core::types::U256,
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
        name = "AggregatorResponseSettingsUpdate",
        abi = "AggregatorResponseSettingsUpdate(address,uint256,uint256,uint256)"
    )]
    pub struct AggregatorResponseSettingsUpdateFilter {
        #[ethevent(indexed)]
        pub aggregator_address: ::ethers::core::types::Address,
        pub variance_threshold: ::ethers::core::types::U256,
        pub min_job_results: ::ethers::core::types::U256,
        pub force_report_period: ::ethers::core::types::U256,
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
        name = "AggregatorSaveResult",
        abi = "AggregatorSaveResult(address,address,int256)"
    )]
    pub struct AggregatorSaveResultFilter {
        #[ethevent(indexed)]
        pub aggregator_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub oracle: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub value: ::ethers::core::types::I256,
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
        name = "AggregatorUpdate",
        abi = "AggregatorUpdate(address,int256,uint256)"
    )]
    pub struct AggregatorUpdateFilter {
        #[ethevent(indexed)]
        pub aggregator_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub value: ::ethers::core::types::I256,
        pub timestamp: ::ethers::core::types::U256,
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
    #[ethevent(name = "OracleAccountInit", abi = "OracleAccountInit(address,address)")]
    pub struct OracleAccountInitFilter {
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
        name = "OraclePayoutEvent",
        abi = "OraclePayoutEvent(address,address,uint256)"
    )]
    pub struct OraclePayoutEventFilter {
        #[ethevent(indexed)]
        pub oracle_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub aggregator_address: ::ethers::core::types::Address,
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
        name = "OracleQueueAccountInit",
        abi = "OracleQueueAccountInit(address,address)"
    )]
    pub struct OracleQueueAccountInitFilter {
        #[ethevent(indexed)]
        pub authority: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISwitchboardEventsEvents {
        AggregatorAccountInitFilter(AggregatorAccountInitFilter),
        AggregatorFundEventFilter(AggregatorFundEventFilter),
        AggregatorIntervalRefreshedFilter(AggregatorIntervalRefreshedFilter),
        AggregatorResponseSettingsUpdateFilter(AggregatorResponseSettingsUpdateFilter),
        AggregatorSaveResultFilter(AggregatorSaveResultFilter),
        AggregatorUpdateFilter(AggregatorUpdateFilter),
        OracleAccountInitFilter(OracleAccountInitFilter),
        OraclePayoutEventFilter(OraclePayoutEventFilter),
        OracleQueueAccountInitFilter(OracleQueueAccountInitFilter),
    }
    impl ::ethers::contract::EthLogDecode for ISwitchboardEventsEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AggregatorAccountInitFilter::decode_log(log) {
                return Ok(
                    ISwitchboardEventsEvents::AggregatorAccountInitFilter(decoded),
                );
            }
            if let Ok(decoded) = AggregatorFundEventFilter::decode_log(log) {
                return Ok(ISwitchboardEventsEvents::AggregatorFundEventFilter(decoded));
            }
            if let Ok(decoded) = AggregatorIntervalRefreshedFilter::decode_log(log) {
                return Ok(
                    ISwitchboardEventsEvents::AggregatorIntervalRefreshedFilter(decoded),
                );
            }
            if let Ok(decoded)
                = AggregatorResponseSettingsUpdateFilter::decode_log(log) {
                return Ok(
                    ISwitchboardEventsEvents::AggregatorResponseSettingsUpdateFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = AggregatorSaveResultFilter::decode_log(log) {
                return Ok(ISwitchboardEventsEvents::AggregatorSaveResultFilter(decoded));
            }
            if let Ok(decoded) = AggregatorUpdateFilter::decode_log(log) {
                return Ok(ISwitchboardEventsEvents::AggregatorUpdateFilter(decoded));
            }
            if let Ok(decoded) = OracleAccountInitFilter::decode_log(log) {
                return Ok(ISwitchboardEventsEvents::OracleAccountInitFilter(decoded));
            }
            if let Ok(decoded) = OraclePayoutEventFilter::decode_log(log) {
                return Ok(ISwitchboardEventsEvents::OraclePayoutEventFilter(decoded));
            }
            if let Ok(decoded) = OracleQueueAccountInitFilter::decode_log(log) {
                return Ok(
                    ISwitchboardEventsEvents::OracleQueueAccountInitFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ISwitchboardEventsEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AggregatorAccountInitFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AggregatorFundEventFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AggregatorIntervalRefreshedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AggregatorResponseSettingsUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AggregatorSaveResultFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AggregatorUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OracleAccountInitFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OraclePayoutEventFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OracleQueueAccountInitFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AggregatorAccountInitFilter>
    for ISwitchboardEventsEvents {
        fn from(value: AggregatorAccountInitFilter) -> Self {
            Self::AggregatorAccountInitFilter(value)
        }
    }
    impl ::core::convert::From<AggregatorFundEventFilter> for ISwitchboardEventsEvents {
        fn from(value: AggregatorFundEventFilter) -> Self {
            Self::AggregatorFundEventFilter(value)
        }
    }
    impl ::core::convert::From<AggregatorIntervalRefreshedFilter>
    for ISwitchboardEventsEvents {
        fn from(value: AggregatorIntervalRefreshedFilter) -> Self {
            Self::AggregatorIntervalRefreshedFilter(value)
        }
    }
    impl ::core::convert::From<AggregatorResponseSettingsUpdateFilter>
    for ISwitchboardEventsEvents {
        fn from(value: AggregatorResponseSettingsUpdateFilter) -> Self {
            Self::AggregatorResponseSettingsUpdateFilter(value)
        }
    }
    impl ::core::convert::From<AggregatorSaveResultFilter> for ISwitchboardEventsEvents {
        fn from(value: AggregatorSaveResultFilter) -> Self {
            Self::AggregatorSaveResultFilter(value)
        }
    }
    impl ::core::convert::From<AggregatorUpdateFilter> for ISwitchboardEventsEvents {
        fn from(value: AggregatorUpdateFilter) -> Self {
            Self::AggregatorUpdateFilter(value)
        }
    }
    impl ::core::convert::From<OracleAccountInitFilter> for ISwitchboardEventsEvents {
        fn from(value: OracleAccountInitFilter) -> Self {
            Self::OracleAccountInitFilter(value)
        }
    }
    impl ::core::convert::From<OraclePayoutEventFilter> for ISwitchboardEventsEvents {
        fn from(value: OraclePayoutEventFilter) -> Self {
            Self::OraclePayoutEventFilter(value)
        }
    }
    impl ::core::convert::From<OracleQueueAccountInitFilter>
    for ISwitchboardEventsEvents {
        fn from(value: OracleQueueAccountInitFilter) -> Self {
            Self::OracleQueueAccountInitFilter(value)
        }
    }
}

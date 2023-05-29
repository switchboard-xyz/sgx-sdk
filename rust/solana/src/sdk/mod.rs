use anchor_lang::prelude::*;
use solana_program::pubkey;

pub mod aggregator;
pub mod buffer_relayer;
pub mod crank;
pub mod decimal;
pub mod ecvrf;
pub mod error;
pub mod history_buffer;
pub mod job;
pub mod lease;
pub mod oracle;
pub mod permission;
pub mod queue;
pub mod quote;
pub mod sb_state;
pub mod sliding_window;
pub mod vrf;
pub mod vrf_lite;
pub mod vrf_pool;

pub use aggregator::*;
pub use buffer_relayer::*;
pub use crank::*;
pub use decimal::*;
pub use ecvrf::*;
pub use error::SwitchboardError;
pub use history_buffer::*;
pub use job::*;
pub use lease::*;
pub use oracle::*;
pub use permission::*;
pub use queue::*;
pub use quote::*;
pub use sb_state::*;
pub use sliding_window::*;
pub use vrf::*;
pub use vrf_lite::*;
pub use vrf_pool::*;

/// Seed used to derive the SbState PDA.
pub const STATE_SEED: &[u8] = b"STATE";
/// Seed used to derive the PermissionAccountData PDA.
pub const PERMISSION_SEED: &[u8] = b"PermissionAccountData";
/// Seed used to derive the LeaseAccountData PDA.
pub const LEASE_SEED: &[u8] = b"LeaseAccountData";
/// Seed used to derive the OracleAccountData PDA.
pub const ORACLE_SEED: &[u8] = b"OracleAccountData";
/// Discriminator used for Switchboard buffer accounts.
pub const BUFFER_DISCRIMINATOR: &[u8] = b"BUFFERxx";

/// Seed used to derive the SlidingWindow PDA.
const SLIDING_RESULT_SEED: &[u8] = b"SlidingResultAccountData";

/// Program id for the Switchboard oracle program
pub const SWITCHBOARD_PROGRAM_ID: Pubkey = pubkey!("SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f");

/// Program id for the Switchboard oracle program
pub const SWITCHBOARD_ATTESTATION_PROGRAM_ID: Pubkey =
    pubkey!("2No5FVKPAAYqytpkEoq93tVh33fo4p6DgAnm4S6oZHo7");

declare_id!(SWITCHBOARD_PROGRAM_ID);

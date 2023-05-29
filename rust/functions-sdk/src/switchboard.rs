use crate::config::Config;
use crate::{AnchorProgram, Keypair};
use std::sync::Arc;

use anchor_client::solana_sdk::signature::Signer;
use anchor_client::solana_sdk::signer::keypair::keypair_from_seed;
use anchor_client::Client;
use anchor_lang::solana_program::example_mocks::solana_sdk::signature::Keypair;
// use clokwerk::Interval;
use clokwerk::{AsyncScheduler, TimeUnits};
use sbac::sgx::Sgx;
use sbac::solana::*;
use solana_sdk::{pubkey, pubkey::Pubkey};

use switchboard_attestation_client as sbac;
const SWITCHBOARD_PROGRAM: Pubkey = pubkey!("SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f");
pub type Keypair = anchor_client::solana_sdk::signer::keypair::Keypair;
pub type AnchorClient = anchor_client::Client<Keypair>;
pub type AnchorProgram = anchor_client::Program<Keypair>;
pub struct SwitchboardFunctions {
    config: Config,
}
pub struct SwitchboardWorkerFunctionsArgs {
    program_id: Pubkey,
    queue: Pubkey,
    payer_path: String,
    cluster: Cluster,
}

trait Worker {
    pub fn new(args: SwitchboardWorkerFunctionsArgs) -> Self;
    fn init_quote_kp(client: AnchorClient, payer: Keypair) -> Keypair;
    pub fn create_and_run_function(self, client: AnchorClient, payer: Keypair, trigger: Trigger);
}
trait ServerlessFunctions {}

pub enum Trigger {
    /// milliseconds
    Interval(u64),
    Cron(String),
}

impl Worker for SwitchboardFunctions {
    fn new(args: SwitchboardWorkerFunctionsArgs) -> Self {
        let SwitchboardFunctionsArgs {
            program_id,
            payer_path,
            cluster,
            queue,
        } = args;
        Self {
            config: Config::new(program_id, queue, payer_path, cluster),
        }
    }
    fn init_quote_kp(client: AnchorClient, payer: Keypair) -> Keypair {
        let mut randomness = [0; 32];
        Sgx::read_rand(&mut randomness).unwrap();
        let quote_kp = Arc::new(keypair_from_seed(&randomness).unwrap());
        let quote = Sgx::gramine_generate_quote(&quote_kp.pubkey().to_bytes()).unwrap();
        let quote_init_ixs = QuoteInitSimple::build(
            &client,
            QuoteInitSimpleArgs {
                quote: quote_kp.pubkey(),
                verifier_queue: VERIFIER_QUEUE,
                authority: quote_kp.pubkey(),
                data: quote,
            },
            vec![&payer, &quote_kp],
        )
        .unwrap();
        let rpc = client.program(SWITCHBOARD_PROGRAM).rpc();
        let blockhash = rpc.get_latest_blockhash().unwrap();
        let mut sigs = Vec::new();
        for (i, ix) in quote_init_ixs.iter().enumerate() {
            println!("Trying quote init {}", i);
            let tx = ix_to_tx(&[ix.clone()], &[&payer, &quote_kp], blockhash);
            let sig = rpc.send_transaction(&tx).unwrap();
            println!("Quote init {}", sig);
            sigs.push(sig);
        }
        for sig in sigs {
            rpc.poll_for_signature_confirmation(&sig, 20).unwrap();
            println!("{} confirmed", sig);
        }
        quote_kp
    }
    fn create_and_run_function(self, trigger: Trigger) {
        let quote_signer = Self::init_quote_kp(client, payer);
        match trigger {
            Trigger::Interval(ms) => {
                let mut scheduler = AsyncScheduler::with_tz(chrono::Utc);

                let client = self.config.client;

                // execute every interval
                scheduler.every((ms / 1000).minutes()).run(move || {
                    // add code here
                });
                let switchboard_program: AnchorProgram = config.client.program(SWITCHBOARD_PROGRAM);
                // heartbeat every 60 seconds
                scheduler
                    .every(60.seconds())
                    .run(move || switchboard::run_heartbeat(switchboard_program.clone()));

                loop {
                    scheduler.run_pending();
                    thread::sleep(Duration::from_millis(10));
                }
            }
            Trigger::Cron(exp) => {
                let mut sched = JobScheduler::new();

                sched.add(Job::new(exp.parse().unwrap(), || {
                    // add code here
                }));

                loop {
                    sched.tick();

                    std::thread::sleep(Duration::from_millis(10));
                }
            }
        }
    }
}

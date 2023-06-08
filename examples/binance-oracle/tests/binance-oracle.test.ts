import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BinanceOracle } from "../target/types/binance_oracle";
import {
  AttestationPermissionAccount,
  AttestationQueueAccount,
  FunctionAccount,
  QuoteAccount,
  SwitchboardProgram,
  parseMrEnclave,
} from "@switchboard-xyz/solana.js";
import assert from "assert";

type BootstrappedAttestationQueue = {
  attestationQueueAccount: AttestationQueueAccount;
  verifier: {
    quoteAccount: QuoteAccount;
    permissionAccount: AttestationPermissionAccount;
    signer: anchor.web3.Keypair;
  };
};

const MRENCLAVE = parseMrEnclave(
  Buffer.from("3H98v4rOe5oTewVgg/9u2OoNM9fvPcZxRj2vya0R1p4=", "base64")
);

describe("binance-oracle", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BinanceOracle as Program<BinanceOracle>;
  let switchboard: BootstrappedAttestationQueue;
  let functionAccount: FunctionAccount;

  before(async () => {
    switchboard = await AttestationQueueAccount.bootstrapNewQueue(
      await SwitchboardProgram.fromProvider(
        program.provider as anchor.AnchorProvider
      )
    );

    [functionAccount] = await FunctionAccount.create(
      switchboard.attestationQueueAccount.program,
      {
        container: "gallynaut/binance-oracle",
        version: "1",
        schedule: "15 * * * * *",
        mrEnclave: MRENCLAVE,
        attestationQueue: switchboard.attestationQueueAccount,
      }
    );
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        state: anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("BINANCEORACLE")],
          program.programId
        )[0],
        authority: (program.provider as anchor.AnchorProvider).wallet.publicKey,
        attestationQueue: switchboard.attestationQueueAccount.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Pushes a BTC price", async () => {
    const tx = await program.methods.pushData({
      
    }).accounts().rpc();
  });
});

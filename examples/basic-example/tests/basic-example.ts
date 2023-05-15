import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicExample } from "../target/types/basic_example";
import { sleep } from "@switchboard-xyz/common";
import { assert } from "console";
import {
  QueueAccount,
  SwitchboardQuoteProgram,
  PROGRAM_ID,
  NodeAccount,
  QuoteAccount,
} from "@switchboard-xyz/sgx";
import fs from "fs";
import path from "path";
import { PublicKey } from "@solana/web3.js";

const SAMPLE_REPORTS = [
  {
    temperature: 100,
    windspeed: 1337,
    weathercode: new anchor.BN(0),
    timestamp: new anchor.BN(1),
  },
  {
    temperature: 100,
    windspeed: 1337,
    weathercode: new anchor.BN(0),
    timestamp: new anchor.BN(2),
  },
  {
    temperature: 100,
    windspeed: 1337,
    weathercode: new anchor.BN(0),
    timestamp: new anchor.BN(3),
  },
  {
    temperature: 100,
    windspeed: 1337,
    weathercode: new anchor.BN(0),
    timestamp: new anchor.BN(4),
  },
];

function getMrEnclave(): Buffer {
  const measurementTxtPath = path.join(
    __dirname,
    "..",
    "sgx-function",
    "measurement.txt"
  );
  if (!fs.existsSync(measurementTxtPath)) {
    throw new Error(
      `Failed to find measurement at ${path.relative(
        process.cwd(),
        measurementTxtPath
      )}`
    );
  }

  const measurement = fs.readFileSync(measurementTxtPath, "utf-8").trim();
  const buffer = Buffer.from(measurement, "base64");
  return buffer;
}

describe("basic-example", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BasicExample as Program<BasicExample>,
    stationSeed = anchor.utils.bytes.utf8.encode("WEATHERREPORT");

  let [stationPubkey, stationBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [stationSeed],
      program.programId
    );

  let switchboard: SwitchboardQuoteProgram;
  let verifierQueueAccount: QueueAccount;
  let serviceQueueAccount: QueueAccount;
  let mrEnclave: Buffer;
  let nodeAccount: NodeAccount;
  let quoteAccount: QuoteAccount;

  before(async () => {
    switchboard = await SwitchboardQuoteProgram.fromProvider(
      program.provider as anchor.AnchorProvider,
      PROGRAM_ID
    );

    [verifierQueueAccount] = await QueueAccount.create(switchboard, {
      authority: switchboard.walletPubkey,
      allowAuthorityOverrideAfter: 60,
      requireAuthorityHeartbeatPermission: true,
      requireUsagePermissions: false,
      maxQuoteVerificationAge: 604800,
      reward: 0,
      nodeTimeout: 180,
    });

    [serviceQueueAccount] = await QueueAccount.create(switchboard, {
      verifierQueue: verifierQueueAccount.publicKey,
      allowAuthorityOverrideAfter: 60, // do we need this?
      authority: switchboard.walletPubkey,
      requireAuthorityHeartbeatPermission: true,
      requireUsagePermissions: false,
      maxQuoteVerificationAge: 604800,
      reward: 0,
      nodeTimeout: 180,
    });

    mrEnclave = getMrEnclave();

    await verifierQueueAccount.addMrEnclave({ mrEnclave });

    [nodeAccount] = await NodeAccount.create(switchboard, {
      authority: switchboard.walletPubkey,
      queue: verifierQueueAccount.publicKey,
      queueAuthorityPubkey: switchboard.walletPubkey,
      enable: true,
    });

    [quoteAccount] = await nodeAccount.createQuote({
      data: mrEnclave,
    });

    // console.log(
    //   await program.provider.connection.getAccountInfo(quoteAccount.publicKey)
    // );

    // await nodeAccount.heartbeat({ quote: quoteAccount.publicKey });
  });

  it("Is initialized!", async () => {
    const station = await program.account.weatherStation.fetchNullable(
      stationPubkey
    );
    if (station) {
      console.log(`Station already initialized`);
      return;
    }

    const tx = await program.methods
      .initialize()
      .accounts({
        station: stationPubkey,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const stationState = await program.account.weatherStation.fetch(
      stationPubkey
    );
    console.log(JSON.stringify(stationState, jsonReplacers, 2));
  });

  it("Is saves a report!", async () => {
    const tx = await program.methods
      .report({ reports: SAMPLE_REPORTS })
      .accounts({
        station: stationPubkey,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const station = await program.account.weatherStation.fetch(stationPubkey);
    assert(station.reports[0].temperature === 1000);
    assert(station.lastUpdated.toNumber() > 0);

    const stationState = await program.account.weatherStation.fetch(
      stationPubkey
    );
    console.log(JSON.stringify(stationState, jsonReplacers, 2));
  });
});

function jsonReplacers(key, value) {
  if (anchor.BN.isBN(value)) {
    return value.toString(10);
  }
  if (value instanceof PublicKey) {
    return value.toBase58();
  }
  return value;
}

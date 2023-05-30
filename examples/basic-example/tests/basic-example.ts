import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicExample } from "../target/types/basic_example";
import { sleep } from "@switchboard-xyz/common";
import { assert } from "console";

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

  let mrEnclave: Buffer;

  before(async () => {});

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

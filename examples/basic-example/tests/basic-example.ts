import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicExample } from "../target/types/basic_example";
import { sleep } from "@switchboard-xyz/common";
import { assert } from "console";

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

describe("basic-example", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BasicExample as Program<BasicExample>,
    stationSeed = anchor.utils.bytes.utf8.encode("WEATHERREPORT");

  let stationPubkey: anchor.web3.PublicKey;

  before(async () => {
    [stationPubkey] = anchor.web3.PublicKey.findProgramAddressSync(
      [stationSeed],
      program.programId
    );
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

    console.log(await program.account.weatherStation.fetch(stationPubkey));
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
  });
});

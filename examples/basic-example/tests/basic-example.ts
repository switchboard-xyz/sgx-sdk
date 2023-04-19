import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicExample } from "../target/types/basic_example";

describe("basic-example", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BasicExample as Program<BasicExample>;

  it("Is initialized!", async () => {
    // Add your test here.
    // console.log(program.programId.toBase58());

    // const programAccountInfo = await program.provider.connection.getAccountInfo(
    //   program.programId
    // );
    // console.log(programAccountInfo);
    // console.log(`owner: ${programAccountInfo.owner.toBase58()}`);
    // console.log(`isExecutable: ${programAccountInfo.executable}`);

    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

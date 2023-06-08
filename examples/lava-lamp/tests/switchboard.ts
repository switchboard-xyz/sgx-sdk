import * as anchor from "@coral-xyz/anchor";
import {
  AttestationQueueAccount,
  QuoteAccount,
  SwitchboardProgram,
  parseMrEnclave,
  RawBuffer,
  FunctionAccount,
  TransactionObject,
} from "@switchboard-xyz/solana.js";

export interface SwitchboardEnvironment {
  attestationQueueAccount: AttestationQueueAccount;
  quoteVerifierKeypair: anchor.web3.Keypair;
  attestationQuoteVerifierAccount: QuoteAccount;
  functionKeypair: anchor.web3.Keypair;
  functionAccount: FunctionAccount;
  functionQuoteAccount: QuoteAccount;
}

export async function initializeSwitchboardEnvironment(
  provider: anchor.AnchorProvider,
  _mrEnclave: RawBuffer
): Promise<SwitchboardEnvironment> {
  const mrEnclave = parseMrEnclave(_mrEnclave);
  const switchboard = await SwitchboardProgram.fromProvider(provider);
  const payer = switchboard.wallet.payer;

  const quoteVerifierMrEnclave = parseMrEnclave(
    "This is the quote verifier MrEnclave"
  );

  const quoteVerifierKeypair = anchor.web3.Keypair.generate();

  const [attestationQueueAccount] = await AttestationQueueAccount.create(
    switchboard,
    {
      reward: 0,
      allowAuthorityOverrideAfter: 60, // should increase this
      maxQuoteVerificationAge: 604800,
      requireAuthorityHeartbeatPermission: false,
      requireUsagePermissions: false,
      nodeTimeout: 604800,
    }
  );

  await attestationQueueAccount.addMrEnclave({
    mrEnclave: new Uint8Array(quoteVerifierMrEnclave),
  });

  const [attestationQuoteVerifierAccount] =
    await attestationQueueAccount.createQuote({
      registryKey: new Uint8Array(Array(64).fill(1)),
      keypair: quoteVerifierKeypair,
      enable: true,
      queueAuthorityPubkey: switchboard.walletPubkey,
    });

  // join the queue so we can verify other quotes
  await attestationQuoteVerifierAccount.heartbeat({
    keypair: quoteVerifierKeypair,
  });

  // create temporary 2nd payer/quote to verify itself
  const payer2 = anchor.web3.Keypair.generate();
  await switchboard.signAndSend(
    new TransactionObject(
      payer.publicKey,
      [
        anchor.web3.SystemProgram.transfer({
          fromPubkey: payer.publicKey,
          toPubkey: payer2.publicKey,
          lamports: anchor.web3.LAMPORTS_PER_SOL,
        }),
      ],
      []
    )
  );

  // create quote #2
  const quoteKeypair2 = anchor.web3.Keypair.generate();
  const [attestationQuoteAccount2] = await attestationQueueAccount.createQuote({
    registryKey: new Uint8Array(Array(64).fill(1)),
    keypair: quoteKeypair2,
    enable: true,
    queueAuthorityPubkey: payer.publicKey,
    owner: payer2.publicKey,
  });

  // quote #1 verifies quote #2
  await attestationQuoteAccount2.verify({
    timestamp: new anchor.BN(Math.floor(Date.now() / 1000)),
    mrEnclave: new Uint8Array(quoteVerifierMrEnclave),
    verifierKeypair: quoteVerifierKeypair,
  });

  // join the queue so we can verify the overrridden quote
  await attestationQuoteAccount2.heartbeat({ keypair: quoteKeypair2 });

  // quote #2 verifies quote #1
  await attestationQuoteVerifierAccount.verify({
    timestamp: new anchor.BN(Math.floor(Date.now() / 1000)),
    mrEnclave: new Uint8Array(quoteVerifierMrEnclave),
    verifierKeypair: quoteKeypair2,
  });

  // create the function account
  const functionKeypair = anchor.web3.Keypair.generate();
  const [functionAccount] = await FunctionAccount.create(switchboard, {
    name: "FUNCTION_NAME",
    metadata: "FUNCTION_METADATA",
    schedule: "30 * * * * *", // every 30 seconds
    container: "containerId",
    version: "1.0.0",
    mrEnclave: mrEnclave,
    attestationQueue: attestationQueueAccount,
    keypair: functionKeypair,
  });

  const [functionQuoteAccount] = functionAccount.getQuoteAccount();

  await functionQuoteAccount.verify({
    timestamp: new anchor.BN(Math.floor(Date.now() / 1000)),
    mrEnclave: mrEnclave,
    verifierKeypair: quoteVerifierKeypair,
  });

  const [payerTokenWallet] = await switchboard.mint.getOrCreateWrappedUser(
    switchboard.walletPubkey,
    { fundUpTo: 0.25 }
  );

  await functionAccount.fund({
    fundAmount: 0.25,
    funderTokenWallet: payerTokenWallet,
  });

  return {
    attestationQueueAccount,
    quoteVerifierKeypair,
    attestationQuoteVerifierAccount,
    functionKeypair,
    functionAccount,
    functionQuoteAccount,
  };
}

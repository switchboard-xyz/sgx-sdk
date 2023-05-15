import 'mocha';
const assert = require('assert');
import { QueueAccount, NodeAccount, SwitchboardQuoteProgram } from '../src';
import { TestContext, setupTest } from './utils';
import { sleep } from '@switchboard-xyz/common';
import { LAMPORTS_PER_SOL } from '@solana/web3.js';

describe('Attestation Tests', () => {
  let ctx: TestContext;

  let program: SwitchboardQuoteProgram;

  const mrEnclave = Buffer.from('ABC123');

  before(async () => {
    ctx = await setupTest();

    program = ctx.program;

    console.log(program.programId.toBase58());
  });

  it('Attestation', async () => {
    console.log('Starting tests ...');
    console.log(program.walletPubkey.toBase58());
    console.log(
      (await program.connection.getBalance(program.walletPubkey)) /
        LAMPORTS_PER_SOL,
      'SOL'
    );
    const [verifierQueueAccount] = await QueueAccount.create(program, {
      authority: program.walletPubkey,
      allowAuthorityOverrideAfter: 60,
      requireAuthorityHeartbeatPermission: true,
      requireUsagePermissions: false,
      maxQuoteVerificationAge: 604800,
      reward: 0,
      nodeTimeout: 180,
    });

    console.log(`Created verifier queue ...`);

    const [queueAccount] = await QueueAccount.create(program, {
      verifierQueue: verifierQueueAccount.publicKey,
      allowAuthorityOverrideAfter: 60, // do we need this?
      authority: program.walletPubkey,
      requireAuthorityHeartbeatPermission: true,
      requireUsagePermissions: false,
      maxQuoteVerificationAge: 604800,
      reward: 0,
      nodeTimeout: 180,
    });

    console.log(`Created service queue ...`);

    await verifierQueueAccount.addMrEnclave({
      mrEnclave,
    });

    console.log(`Added MRENCLAVE ...`);

    const [vnodeAccount] = await NodeAccount.create(program, {
      authority: program.walletPubkey,
      queue: verifierQueueAccount.publicKey,
      queueAuthorityPubkey: (await verifierQueueAccount.loadData()).authority,
      enable: true,
    });

    console.log(`Created verifierQueue NodeAccount ...`);

    const [vquoteAccount] = await vnodeAccount.createQuote({
      data: mrEnclave,
    });

    console.log(`Created verifierQueue QuoteAccount ...`);

    await sleep(5000);

    await vnodeAccount.heartbeat({
      quote: vquoteAccount.publicKey,
    });

    console.log(`NodeAccount verifierQueue heartbeat ...`);

    const [nodeAccount] = await NodeAccount.create(program, {
      authority: program.walletPubkey,
      queue: queueAccount.publicKey,
      queueAuthorityPubkey: program.walletPubkey,
      enable: true,
    });

    console.log(`Created serviceQueue NodeAccount ...`);

    const [quoteAccount] = await nodeAccount.createQuote({
      data: mrEnclave,
    });

    console.log(`Created serviceQueue QuoteAccount`);

    await sleep(5000);

    await nodeAccount.heartbeat({ quote: quoteAccount.publicKey });

    console.log(`NodeAccount serviceQueue heartbeat ...`);
    // ===
  });
});

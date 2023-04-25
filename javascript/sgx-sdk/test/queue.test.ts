import 'mocha';
const assert = require('assert');
import {
  QueueAccount,
  PermissionAccount,
  NodeAccount,
  QuoteAccount,
  types,
  SwitchboardQuoteProgram,
} from '../src';
import { TestContext, setupTest } from './utils';
// import { SwitchboardQuoteVerifier } from '../target/types/switchboard_quote_verifier';

describe('SAS Tests', () => {
  let ctx: TestContext;

  let program: SwitchboardQuoteProgram;

  before(async () => {
    ctx = await setupTest();

    program = ctx.program;
  });

  it('SAS a sss', async () => {
    console.log('Starting tests ...');
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

    const tx2 = await verifierQueueAccount.addMrEnclave({
      mrEnclave: Buffer.from('123'),
    });

    console.log(`Added MRENCLAVE ...`);

    console.log(await verifierQueueAccount.loadData());
    // const tx3 = await queueAccount.removeMrEnclave({
    // mrEnclave: Buffer.from("123"),
    // });
    // await sendTx(program, tx3, [payerKeypair]);
    const [vnodeAccount, vtx] = await NodeAccount.create(program, {
      authority: program.walletPubkey,
      queue: verifierQueueAccount.publicKey,
    });

    console.log(`Created NodeAccount ...`);

    const [vpermissionAccount, vtx2] = await PermissionAccount.create(program, {
      authority: program.walletPubkey,
      granter: verifierQueueAccount.publicKey,
      grantee: vnodeAccount.publicKey,
    });

    console.log(`Created PermissionAccount ...`);

    const vtx3 = await vpermissionAccount.set({
      permission: new types.SwitchboardPermission.PermitNodeheartbeat(),
      enable: true,
    });

    console.log(`Enabled PermissionAccount ...`);

    const [vquoteAccount] = await QuoteAccount.create(program, {
      node: vnodeAccount.publicKey,
      data: Buffer.from(''),
    });

    console.log(`Created QuoteAccount ...`);

    const vtx4 = await vnodeAccount.heartbeat({
      quote: vquoteAccount.publicKey,
    });

    console.log(`NodeAccount heartbeat ...`);

    const [nodeAccount, tx4] = await NodeAccount.create(program, {
      authority: program.walletPubkey,
      queue: queueAccount.publicKey,
    });

    console.log(`Created NodeAccount #2 ...`);

    const [permissionAccount, tx5] = await PermissionAccount.create(program, {
      authority: program.walletPubkey,
      granter: queueAccount.publicKey,
      grantee: nodeAccount.publicKey,
    });

    console.log(`Created PermissionAccount #2`);

    const tx6 = await permissionAccount.set({
      permission: new types.SwitchboardPermission.PermitNodeheartbeat(),
      enable: true,
    });

    console.log(`Enabled PermissionAccount #2`);

    const [quoteAccount] = await QuoteAccount.create(program, {
      node: nodeAccount.publicKey,
      data: Buffer.from(''),
    });

    console.log(`Created QuoteAccount #2`);

    const tx7 = await nodeAccount.heartbeat({ quote: quoteAccount.publicKey });

    console.log(`NodeAccount #2 heartbeat ...`);
    // ===
  });
});

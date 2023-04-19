import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import {
  TransactionInstruction,
  PublicKey,
  AccountMeta,
} from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars

export interface NodeHeartbeatArgs {
  params: types.NodeHeartbeatParamsFields;
}

export interface NodeHeartbeatAccounts {
  node: PublicKey;
  authority: PublicKey;
  queue: PublicKey;
  queueAuthority: PublicKey;
  quote: PublicKey;
  gcNode: PublicKey;
  permission: PublicKey;
}

export const layout = borsh.struct([
  types.NodeHeartbeatParams.layout('params'),
]);

export function nodeHeartbeat(
  program: SwitchboardQuoteProgram,
  args: NodeHeartbeatArgs,
  accounts: NodeHeartbeatAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.node, isSigner: false, isWritable: true },
    { pubkey: accounts.authority, isSigner: false, isWritable: false },
    { pubkey: accounts.queue, isSigner: false, isWritable: true },
    { pubkey: accounts.queueAuthority, isSigner: false, isWritable: false },
    { pubkey: accounts.quote, isSigner: false, isWritable: false },
    { pubkey: accounts.gcNode, isSigner: false, isWritable: false },
    { pubkey: accounts.permission, isSigner: false, isWritable: false },
  ];
  const identifier = Buffer.from([186, 91, 115, 80, 234, 105, 202, 70]);
  const buffer = Buffer.alloc(1000);
  const len = layout.encode(
    {
      params: types.NodeHeartbeatParams.toEncodable(args.params),
    },
    buffer
  );
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len);
  const ix = new TransactionInstruction({
    keys,
    programId: program.programId,
    data,
  });
  return ix;
}

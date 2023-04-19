import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import {
  TransactionInstruction,
  PublicKey,
  AccountMeta,
} from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars

export interface NodeInitArgs {
  params: types.NodeInitParamsFields;
}

export interface NodeInitAccounts {
  node: PublicKey;
  authority: PublicKey;
  queue: PublicKey;
  payer: PublicKey;
  systemProgram: PublicKey;
}

export const layout = borsh.struct([types.NodeInitParams.layout('params')]);

export function nodeInit(
  program: SwitchboardQuoteProgram,
  args: NodeInitArgs,
  accounts: NodeInitAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.node, isSigner: true, isWritable: true },
    { pubkey: accounts.authority, isSigner: true, isWritable: false },
    { pubkey: accounts.queue, isSigner: false, isWritable: false },
    { pubkey: accounts.payer, isSigner: true, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ];
  const identifier = Buffer.from([50, 90, 191, 176, 2, 164, 173, 5]);
  const buffer = Buffer.alloc(1000);
  const len = layout.encode(
    {
      params: types.NodeInitParams.toEncodable(args.params),
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

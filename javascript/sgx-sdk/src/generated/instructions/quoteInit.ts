import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import {
  TransactionInstruction,
  PublicKey,
  AccountMeta,
} from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars

export interface QuoteInitArgs {
  params: types.QuoteInitParamsFields;
}

export interface QuoteInitAccounts {
  quote: PublicKey;
  queue: PublicKey;
  verifierQueue: PublicKey;
  node: PublicKey;
  authority: PublicKey;
  queueAuthority: PublicKey;
  payer: PublicKey;
  systemProgram: PublicKey;
}

export const layout = borsh.struct([types.QuoteInitParams.layout('params')]);

export function quoteInit(
  program: SwitchboardQuoteProgram,
  args: QuoteInitArgs,
  accounts: QuoteInitAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.quote, isSigner: true, isWritable: true },
    { pubkey: accounts.queue, isSigner: false, isWritable: true },
    { pubkey: accounts.verifierQueue, isSigner: false, isWritable: true },
    { pubkey: accounts.node, isSigner: false, isWritable: true },
    { pubkey: accounts.authority, isSigner: true, isWritable: false },
    { pubkey: accounts.queueAuthority, isSigner: false, isWritable: false },
    { pubkey: accounts.payer, isSigner: true, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ];
  const identifier = Buffer.from([124, 251, 28, 247, 136, 141, 198, 116]);
  const buffer = Buffer.alloc(1000);
  const len = layout.encode(
    {
      params: types.QuoteInitParams.toEncodable(args.params),
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

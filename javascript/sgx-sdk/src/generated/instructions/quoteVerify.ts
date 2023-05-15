import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import {
  TransactionInstruction,
  PublicKey,
  AccountMeta,
} from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars

export interface QuoteVerifyArgs {
  params: types.QuoteVerifyParamsFields;
}

export interface QuoteVerifyAccounts {
  quote: PublicKey;
  node: PublicKey;
  queue: PublicKey;
  verifierQueue: PublicKey;
  verifierNode: PublicKey;
  verifierAuthority: PublicKey;
}

export const layout = borsh.struct([types.QuoteVerifyParams.layout('params')]);

export function quoteVerify(
  program: SwitchboardQuoteProgram,
  args: QuoteVerifyArgs,
  accounts: QuoteVerifyAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.quote, isSigner: false, isWritable: true },
    { pubkey: accounts.node, isSigner: false, isWritable: false },
    { pubkey: accounts.queue, isSigner: false, isWritable: false },
    { pubkey: accounts.verifierQueue, isSigner: false, isWritable: false },
    { pubkey: accounts.verifierNode, isSigner: false, isWritable: false },
    { pubkey: accounts.verifierAuthority, isSigner: true, isWritable: false },
  ];
  const identifier = Buffer.from([158, 203, 69, 10, 212, 218, 45, 184]);
  const buffer = Buffer.alloc(1000);
  const len = layout.encode(
    {
      params: types.QuoteVerifyParams.toEncodable(args.params),
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

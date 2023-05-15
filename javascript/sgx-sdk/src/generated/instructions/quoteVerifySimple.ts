import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import {
  TransactionInstruction,
  PublicKey,
  AccountMeta,
} from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars

export interface QuoteVerifySimpleArgs {
  params: types.QuoteVerifySimpleParamsFields;
}

export interface QuoteVerifySimpleAccounts {
  quote: PublicKey;
  verifierQueue: PublicKey;
  verifierNode: PublicKey;
  verifierAuthority: PublicKey;
}

export const layout = borsh.struct([
  types.QuoteVerifySimpleParams.layout('params'),
]);

export function quoteVerifySimple(
  program: SwitchboardQuoteProgram,
  args: QuoteVerifySimpleArgs,
  accounts: QuoteVerifySimpleAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.quote, isSigner: false, isWritable: true },
    { pubkey: accounts.verifierQueue, isSigner: false, isWritable: false },
    { pubkey: accounts.verifierNode, isSigner: false, isWritable: false },
    { pubkey: accounts.verifierAuthority, isSigner: true, isWritable: false },
  ];
  const identifier = Buffer.from([238, 40, 61, 18, 136, 126, 114, 227]);
  const buffer = Buffer.alloc(1000);
  const len = layout.encode(
    {
      params: types.QuoteVerifySimpleParams.toEncodable(args.params),
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

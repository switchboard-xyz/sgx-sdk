import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import {
  TransactionInstruction,
  PublicKey,
  AccountMeta,
} from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars

export interface QueueAddMrSignerArgs {
  params: types.QueueAddMrEnclaveParamsFields;
}

export interface QueueAddMrSignerAccounts {
  queue: PublicKey;
  authority: PublicKey;
}

export const layout = borsh.struct([
  types.QueueAddMrEnclaveParams.layout('params'),
]);

export function queueAddMrSigner(
  program: SwitchboardQuoteProgram,
  args: QueueAddMrSignerArgs,
  accounts: QueueAddMrSignerAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.queue, isSigner: false, isWritable: true },
    { pubkey: accounts.authority, isSigner: true, isWritable: false },
  ];
  const identifier = Buffer.from([125, 221, 90, 120, 58, 252, 69, 99]);
  const buffer = Buffer.alloc(1000);
  const len = layout.encode(
    {
      params: types.QueueAddMrEnclaveParams.toEncodable(args.params),
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

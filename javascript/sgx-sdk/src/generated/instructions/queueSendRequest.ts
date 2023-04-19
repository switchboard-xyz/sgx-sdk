import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import {
  TransactionInstruction,
  PublicKey,
  AccountMeta,
} from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars

export interface QueueSendRequestArgs {
  params: types.QueueSendRequestParamsFields;
}

export interface QueueSendRequestAccounts {
  queue: PublicKey;
  permission: PublicKey;
  requester: PublicKey;
}

export const layout = borsh.struct([
  types.QueueSendRequestParams.layout('params'),
]);

export function queueSendRequest(
  program: SwitchboardQuoteProgram,
  args: QueueSendRequestArgs,
  accounts: QueueSendRequestAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.queue, isSigner: false, isWritable: false },
    { pubkey: accounts.permission, isSigner: false, isWritable: false },
    { pubkey: accounts.requester, isSigner: true, isWritable: false },
  ];
  const identifier = Buffer.from([95, 70, 31, 116, 120, 21, 126, 175]);
  const buffer = Buffer.alloc(1000);
  const len = layout.encode(
    {
      params: types.QueueSendRequestParams.toEncodable(args.params),
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

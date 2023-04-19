import {
  Keypair,
  PublicKey,
  SystemProgram,
  TransactionSignature,
} from '@solana/web3.js';
import * as errors from '../errors';
import * as types from '../generated';

import { SwitchboardQuoteProgram } from '../SwitchboardQuoteProgram';
import { TransactionObject } from '../TransactionObject';
import { Account } from './account';
import { PermissionAccount } from './permissionAccount';
import { QueueAccount } from './queueAccount';

/**
 *  Parameters for initializing an {@linkcode NodeAccount}
 */
export interface NodeAccountInitParams {
  keypair?: Keypair;
  authority?: PublicKey;
  queue: PublicKey;
}

/**
 *  Parameters for a {@linkcode NodeAccount} to heartbeat
 */
export interface NodeHeartbeatParams {
  quote: PublicKey;
  authority?: Keypair;
}

/**
 * Data: {@linkcode types.NodeAccountData}
 */
export class NodeAccount extends Account<types.NodeAccountData> {
  static accountName = 'NodeAccountData';

  public static size = 372;

  /**
   * Returns the size of an on-chain {@linkcode NodeAccount}.
   */
  public readonly size = this.program.account.NodeAccountData.size;

  /**
   * Retrieve and decode the {@linkcode types.NodeAccountData} stored in this account.
   */
  public async loadData(): Promise<types.NodeAccountData> {
    const data = await types.NodeAccountData.fetch(
      this.program,
      this.publicKey
    );
    if (data === null)
      throw new errors.AccountNotFoundError('Permissions', this.publicKey);
    return data;
  }

  public static createInstruction(
    program: SwitchboardQuoteProgram,
    payer: PublicKey,
    params: NodeAccountInitParams
  ): [NodeAccount, TransactionObject] {
    const keypair = params.keypair ?? Keypair.generate();
    const instruction = types.nodeInit(
      program,
      { params: {} },
      {
        node: keypair.publicKey,
        authority: params.authority ?? payer,
        queue: params.queue,
        payer: payer,
        systemProgram: SystemProgram.programId,
      }
    );

    const nodeAccount = new NodeAccount(program, keypair.publicKey);

    return [nodeAccount, new TransactionObject(payer, [instruction], [])];
  }

  public static async create(
    program: SwitchboardQuoteProgram,
    params: NodeAccountInitParams
  ): Promise<[NodeAccount, TransactionSignature]> {
    const [account, txnObject] = this.createInstruction(
      program,
      program.walletPubkey,
      params
    );
    const txSignature = await program.signAndSend(txnObject);
    return [account, txSignature];
  }

  public async heartbeatInstruction(
    payer: PublicKey,
    params: NodeHeartbeatParams
  ): Promise<TransactionObject> {
    const data = await this.loadData();
    if (
      params.authority &&
      !data.authority.equals(params.authority.publicKey)
    ) {
      throw new errors.IncorrectAuthority(
        data.authority,
        params.authority.publicKey
      );
    }

    const serviceQueueAccount = new QueueAccount(this.program, data.queue);
    const queue = await serviceQueueAccount.loadData();

    const [permissionAccount] = PermissionAccount.fromSeed(
      this.program,
      queue.authority,
      serviceQueueAccount.publicKey,
      this.publicKey
    );

    const txn = new TransactionObject(
      payer,
      [
        types.nodeHeartbeat(
          this.program,
          { params: {} },
          {
            node: this.publicKey,
            authority: data.authority,
            permission: permissionAccount.publicKey,
            queue: data.queue,
            queueAuthority: queue.authority,
            quote: params.quote,
            gcNode: queue.data[queue.gcIdx],
          }
        ),
      ],
      []
    );

    return txn;
  }

  public async heartbeat(
    params: NodeHeartbeatParams
  ): Promise<TransactionSignature> {
    const txnObject = await this.heartbeatInstruction(
      this.program.walletPubkey,
      params
    );
    const txnSignature = await this.program.signAndSend(txnObject);
    return txnSignature;
  }
}

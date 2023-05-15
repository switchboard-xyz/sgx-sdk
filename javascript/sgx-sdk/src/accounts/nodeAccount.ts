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
import { QuoteAccount, QuoteAccountInitParams } from './quoteAccount';

/**
 *  Parameters for initializing an {@linkcode NodeAccount}
 */
export interface NodeAccountInitParams {
  keypair?: Keypair;
  authority?: PublicKey;
  queue: PublicKey;
}

export type NodeInitWithPermissionsParams = NodeAccountInitParams & {
  enable?: boolean;
} & (
    | {
        queueAuthorityPubkey: PublicKey;
      }
    | { queueAuthorityKeypair: Keypair }
  );

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

  get size() {
    return 0;
  }

  // /**
  //  * Returns the size of an on-chain {@linkcode NodeAccount}.
  //  */
  // public readonly size = this.program.account.NodeAccountData.size;

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
    params: NodeInitWithPermissionsParams
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

    let queueAuthorityPubkey: PublicKey;
    if ('queueAuthorityKeypair' in params) {
      queueAuthorityPubkey = params.queueAuthorityKeypair!.publicKey;
    } else if ('queueAuthorityPubkey' in params) {
      queueAuthorityPubkey = params.queueAuthorityPubkey;
    } else {
      throw new Error(
        `Need to provide 'queueAuthorityPubkey' or 'queueAuthorityKeypair'`
      );
    }

    let nodeInit = new TransactionObject(payer, [instruction], [keypair]);

    const [permissionAccount, permissionInit] =
      PermissionAccount.createInstruction(program, payer, {
        granter: params.queue,
        grantee: nodeAccount.publicKey,
        authority: queueAuthorityPubkey,
      });

    nodeInit.combine(permissionInit);

    if (params.enable && queueAuthorityPubkey) {
      const permissionSetSigners: Array<Keypair> =
        'queueAuthorityKeypair' in params
          ? [params.queueAuthorityKeypair!]
          : [];

      const permissionSetIxn = types.permissionSet(
        program,
        {
          params: {
            permission: new types.SwitchboardPermission.PermitNodeheartbeat()
              .discriminator,
            enable: params.enable,
          },
        },
        {
          permission: permissionAccount.publicKey,
          authority: queueAuthorityPubkey,
          queue: params.queue,
          node: nodeAccount.publicKey,
        }
      );

      nodeInit = nodeInit.add(permissionSetIxn, permissionSetSigners);
    }

    return [nodeAccount, nodeInit];
  }

  public static async create(
    program: SwitchboardQuoteProgram,
    params: NodeInitWithPermissionsParams
  ): Promise<[NodeAccount, TransactionSignature]> {
    const [account, txnObject] = this.createInstruction(
      program,
      program.walletPubkey,
      params
    );
    const txSignature = await program.signAndSend(txnObject);
    return [account, txSignature];
  }

  public async createQuoteInstructions(
    payer: PublicKey,
    params: Omit<QuoteAccountInitParams, 'node'>
  ): Promise<[QuoteAccount, Array<TransactionObject>]> {
    const [quoteAccount, quoteInit] = await QuoteAccount.createInstruction(
      this.program,
      payer,
      {
        ...params,
        node: this.publicKey,
      }
    );
    return [quoteAccount, quoteInit];
  }

  public async createQuote(
    params: Omit<QuoteAccountInitParams, 'node'>
  ): Promise<[QuoteAccount, Array<TransactionSignature>]> {
    const [account, txnObject] = await this.createQuoteInstructions(
      this.program.walletPubkey,
      params
    );
    const txSignatures = await this.program.signAndSendAll(txnObject);
    return [account, txSignatures];
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

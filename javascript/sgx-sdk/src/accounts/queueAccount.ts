import {
  Commitment,
  Keypair,
  PublicKey,
  SystemProgram,
  TransactionSignature,
} from '@solana/web3.js';
import { BN } from '@switchboard-xyz/common';
import * as errors from '../errors';
import * as types from '../generated';
// import { queueAddMrSigner, queueRemoveMrSigner } from '../generated';
import { SwitchboardQuoteProgram } from '../SwitchboardQuoteProgram';
import { TransactionObject } from '../TransactionObject';
import { Account, OnAccountChangeCallback } from './account';

/**
 * Account type representing an oracle queue's configuration along with a buffer account holding a list of oracles that are actively heartbeating.
 *
 * A QueueAccount is responsible for allocating update requests to it's round robin queue of {@linkcode OracleAccount}'s.
 *
 * Data: {@linkcode types.ServiceQueueAccountData}
 *
 * Buffer: {@linkcode QueueDataBuffer}
 */
export class QueueAccount extends Account<types.ServiceQueueAccountData> {
  static accountName = 'ServiceQueueAccountData';

  get size() {
    return 0;
  }

  /**
   * Invoke a callback each time a QueueAccount's data has changed on-chain.
   * @param callback - the callback invoked when the queues state changes
   * @param commitment - optional, the desired transaction finality. defaults to 'confirmed'
   * @returns the websocket subscription id
   */
  onChange(
    callback: OnAccountChangeCallback<types.ServiceQueueAccountData>,
    commitment: Commitment = 'confirmed'
  ): number {
    return this.program.connection.onAccountChange(
      this.publicKey,
      accountInfo =>
        callback(types.ServiceQueueAccountData.decode(accountInfo.data)),
      commitment
    );
  }

  /**
   * Retrieve and decode the {@linkcode types.ServiceQueueAccountData} stored in this account.
   */
  public async loadData(): Promise<types.ServiceQueueAccountData> {
    const data = await types.ServiceQueueAccountData.fetch(
      this.program,
      this.publicKey
    );
    if (data === null)
      throw new errors.AccountNotFoundError('Queue', this.publicKey);
    return data;
  }

  /**
   * Creates a transaction object with oracleQueueInit instructions.
   *
   * @param program The SwitchboardQuoteProgram.
   *
   * @param payer - the publicKey of the account that will pay for the new accounts. Will also be used as the account authority if no other authority is provided.
   *
   * @param params oracle queue configuration parameters.
   *
   * @return Transaction signature and the newly created QueueAccount.
   */
  public static async createInstruction(
    program: SwitchboardQuoteProgram,
    payer: PublicKey,
    params: QueueInitParams
  ): Promise<[QueueAccount, TransactionObject]> {
    const keypair = params.keypair ?? Keypair.generate();
    const verifierQueue = params.verifierQueue ?? keypair.publicKey;

    const queueInitIxn = types.queueInit(
      program,
      {
        params: {
          allowAuthorityOverrideAfter: params.allowAuthorityOverrideAfter,
          requireAuthorityHeartbeatPermission:
            params.requireAuthorityHeartbeatPermission ?? false,
          requireUsagePermissions: params.requireUsagePermissions ?? false,
          maxQuoteVerificationAge: params.maxQuoteVerificationAge,
          reward: params.reward,
          nodeTimeout: params.nodeTimeout,
        },
      },
      {
        queue: keypair.publicKey,
        authority: params.authority ?? payer,
        verifierQueue: verifierQueue,
        payer: payer,
        systemProgram: SystemProgram.programId,
      }
    );

    const queueInitTxn = new TransactionObject(
      payer,
      [queueInitIxn],
      [keypair]
    );

    const queueAccount = new QueueAccount(program, keypair.publicKey);

    return [queueAccount, queueInitTxn];
  }

  /**
   * Creates an oracle queue on-chain and return the transaction signature and created account resource.
   *
   * @param program The SwitchboardQuoteProgram.
   *
   * @param params oracle queue configuration parameters.
   *
   * @return Transaction signature and the newly created QueueAccount.
   */
  public static async create(
    program: SwitchboardQuoteProgram,
    params: QueueInitParams
  ): Promise<[QueueAccount, string]> {
    const [account, txnObject] = await QueueAccount.createInstruction(
      program,
      program.walletPubkey,
      params
    );
    const txnSignature = await program.signAndSend(txnObject);
    return [account, txnSignature];
  }

  public async addMrEnclaveInstruction(
    payer: PublicKey,
    params: QueueModifyMrEnclaveParams
  ): Promise<TransactionObject> {
    const data = await this.loadData();

    const mrEnclave = new Uint8Array(32);
    mrEnclave.set(params.mrEnclave);

    return new TransactionObject(
      payer,
      [
        types.queueAddMrEnclave(
          this.program,
          { params: { mrEnclave: [...mrEnclave] } },
          {
            queue: this.publicKey,
            authority: data.authority,
          }
        ),
      ],
      params.authority ? [params.authority] : []
    );
  }

  public async addMrEnclave(
    params: QueueModifyMrEnclaveParams
  ): Promise<TransactionSignature> {
    const txnObject = await this.addMrEnclaveInstruction(
      this.program.walletPubkey,
      params
    );
    const txnSignature = await this.program.signAndSend(txnObject);
    return txnSignature;
  }

  public async removeMrEnclaveInstruction(
    payer: PublicKey,
    params: QueueModifyMrEnclaveParams
  ): Promise<TransactionObject> {
    const data = await this.loadData();

    return new TransactionObject(
      payer,
      [
        types.queueRemoveMrEnclave(
          this.program,
          { params: { mrEnclave: [...new Uint8Array(params.mrEnclave)] } },
          {
            queue: this.publicKey,
            authority: data.authority,
          }
        ),
      ],
      params.authority ? [params.authority] : []
    );
  }

  public async removeMrEnclave(
    params: QueueModifyMrEnclaveParams
  ): Promise<TransactionSignature> {
    const txnObject = await this.removeMrEnclaveInstruction(
      this.program.walletPubkey,
      params
    );
    const txnSignature = await this.program.signAndSend(txnObject);
    return txnSignature;
  }
}

export type QueueInitParams = {
  keypair?: Keypair;
  verifierQueue?: PublicKey;
  authority?: PublicKey;
  allowAuthorityOverrideAfter: number;
  requireAuthorityHeartbeatPermission?: boolean;
  requireUsagePermissions?: boolean;
  maxQuoteVerificationAge: number;
  reward: number;
  nodeTimeout: number;
};

export type QueueModifyMrEnclaveParams = {
  mrEnclave: Buffer;
  authority?: Keypair;
};

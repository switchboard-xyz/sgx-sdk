import {
  Keypair,
  PublicKey,
  SystemProgram,
  TransactionSignature,
} from '@solana/web3.js';
import * as errors from '../errors';
import * as types from '../generated';
import { quoteInit, quoteVerify } from '../generated';

import { SwitchboardQuoteProgram } from '../SwitchboardQuoteProgram';
import { TransactionObject } from '../TransactionObject';
import { Account } from './account';
import { NodeAccount } from './nodeAccount';
import { QueueAccount } from './queueAccount';
import { BN } from '@switchboard-xyz/common';

/**
 * Data: {@linkcode types.QuoteAccountData}
 */
export class QuoteAccount extends Account<types.QuoteAccountData> {
  static accountName = 'QuoteAccountData';

  get size() {
    return 0;
  }

  // public static size = 372;

  // /**
  //  * Returns the size of an on-chain {@linkcode QuoteAccount}.
  //  */
  // public readonly size = this.program.account.QuoteAccountData.size;

  /**
   * Retrieve and decode the {@linkcode types.QuoteAccountData} stored in this account.
   */
  public async loadData(): Promise<types.QuoteAccountData> {
    const data = await types.QuoteAccountData.fetch(
      this.program,
      this.publicKey
    );
    if (data === null)
      throw new errors.AccountNotFoundError('Permissions', this.publicKey);
    return data;
  }

  public static async createInstruction(
    program: SwitchboardQuoteProgram,
    payer: PublicKey,
    params: QuoteAccountInitParams
  ): Promise<[QuoteAccount, Array<TransactionObject>]> {
    const keypair = params.keypair ?? Keypair.generate();

    const nodeAccount = new NodeAccount(program, params.node);
    const node = await nodeAccount.loadData();

    const queueAccount = new QueueAccount(program, node.queue);
    const queue = await queueAccount.loadData();

    // console.log(
    //   `DATA (len): ${params.data.length}\nDATA: [${new Uint8Array(
    //     params.data
    //   )}]`
    // );

    const txns: Array<TransactionObject> = [];
    for (let i = 0; i < params.data.length; i += 512) {
      const tx = new TransactionObject(
        payer,
        [
          quoteInit(
            program,
            {
              params: {
                // TODO: double check its offset correct
                data: [
                  ...new Uint8Array(
                    params.data.slice(i, Math.min(i + 512, params.data.length))
                  ),
                ],
                totalLen: params.data.length,
                chunkStart: i,
                chunkEnd: Math.min(i + 512, params.data.length),
              },
            },
            {
              quote: keypair.publicKey,
              queue: queueAccount.publicKey,
              queueAuthority: queue.authority,
              verifierQueue: queue.verifierQueue,
              node: nodeAccount.publicKey,
              authority: node.authority,
              payer: payer,
              systemProgram: SystemProgram.programId,
            }
          ),
        ],
        [keypair]
      );

      txns.push(tx);
    }

    const quoteAccount = new QuoteAccount(program, keypair.publicKey);

    return [quoteAccount, TransactionObject.pack(txns)];
  }

  public static async create(
    program: SwitchboardQuoteProgram,
    params: QuoteAccountInitParams
  ): Promise<[QuoteAccount, Array<TransactionSignature>]> {
    const [account, txnObjects] = await this.createInstruction(
      program,
      program.walletPubkey,
      params
    );
    const txSignatures = await program.signAndSendAll(txnObjects);
    return [account, txSignatures];
  }

  public async verifyInstruction(
    payer: PublicKey,
    params: QuoteAccountVerifyParams
  ): Promise<TransactionObject> {
    const data = await this.loadData();

    const nodeAccount = new NodeAccount(this.program, data.node);
    const node = await nodeAccount.loadData();

    const queueAccount = new QueueAccount(this.program, data.queue);
    const queue = await queueAccount.loadData();

    const txn = new TransactionObject(
      payer,
      [
        quoteVerify(
          this.program,
          {
            params: {
              // queueIdx: params.queueIdx,
              timestamp: new BN(0),
            },
          },
          {
            quote: this.publicKey,
            queue: data.queue,
            // TODO: Update
            verifierQueue: PublicKey.default,
            // TODO: Update
            verifierNode: PublicKey.default,
            // TODO: Update
            node: PublicKey.default,
            verifierAuthority: queue.authority,
          }
        ),
      ],
      []
    );
    return txn;
  }

  public async verify(
    params: QuoteAccountVerifyParams
  ): Promise<TransactionSignature> {
    const txnObject = await this.verifyInstruction(
      this.program.walletPubkey,
      params
    );
    const txnSignature = await this.program.signAndSend(txnObject);
    return txnSignature;
  }
}

export type QuoteAccountInitParams = {
  keypair?: Keypair;
  node: PublicKey;
  data: Buffer;
};

export type QuoteAccountVerifyParams = {
  queueIdx: number;
};

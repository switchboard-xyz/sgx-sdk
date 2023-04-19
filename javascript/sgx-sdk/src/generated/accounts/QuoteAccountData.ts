import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import { PublicKey, Connection } from '@solana/web3.js';
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars

export interface QuoteAccountDataFields {
  node: PublicKey;
  queue: PublicKey;
  quoteBuffer: Array<number>;
  quoteLen: number;
  isReady: boolean;
  verificationStatus: types.VerificationStatusKind;
  verificationTimestamp: BN;
  ebuf: Array<number>;
}

export interface QuoteAccountDataJSON {
  node: string;
  queue: string;
  quoteBuffer: Array<number>;
  quoteLen: number;
  isReady: boolean;
  verificationStatus: types.VerificationStatusJSON;
  verificationTimestamp: string;
  ebuf: Array<number>;
}

export class QuoteAccountData {
  readonly node: PublicKey;
  readonly queue: PublicKey;
  readonly quoteBuffer: Array<number>;
  readonly quoteLen: number;
  readonly isReady: boolean;
  readonly verificationStatus: types.VerificationStatusKind;
  readonly verificationTimestamp: BN;
  readonly ebuf: Array<number>;

  static readonly discriminator = Buffer.from([
    205, 205, 167, 232, 0, 74, 44, 160,
  ]);

  static readonly layout = borsh.struct([
    borsh.publicKey('node'),
    borsh.publicKey('queue'),
    borsh.array(borsh.u8(), 8192, 'quoteBuffer'),
    borsh.u32('quoteLen'),
    borsh.bool('isReady'),
    types.VerificationStatus.layout('verificationStatus'),
    borsh.i64('verificationTimestamp'),
    borsh.array(borsh.u8(), 1024, 'ebuf'),
  ]);

  constructor(fields: QuoteAccountDataFields) {
    this.node = fields.node;
    this.queue = fields.queue;
    this.quoteBuffer = fields.quoteBuffer;
    this.quoteLen = fields.quoteLen;
    this.isReady = fields.isReady;
    this.verificationStatus = fields.verificationStatus;
    this.verificationTimestamp = fields.verificationTimestamp;
    this.ebuf = fields.ebuf;
  }

  static async fetch(
    program: SwitchboardQuoteProgram,
    address: PublicKey
  ): Promise<QuoteAccountData | null> {
    const info = await program.connection.getAccountInfo(address);

    if (info === null) {
      return null;
    }
    if (!info.owner.equals(program.programId)) {
      throw new Error("account doesn't belong to this program");
    }

    return this.decode(info.data);
  }

  static async fetchMultiple(
    program: SwitchboardQuoteProgram,
    addresses: PublicKey[]
  ): Promise<Array<QuoteAccountData | null>> {
    const infos = await program.connection.getMultipleAccountsInfo(addresses);

    return infos.map(info => {
      if (info === null) {
        return null;
      }
      if (!info.owner.equals(program.programId)) {
        throw new Error("account doesn't belong to this program");
      }

      return this.decode(info.data);
    });
  }

  static decode(data: Buffer): QuoteAccountData {
    if (!data.slice(0, 8).equals(QuoteAccountData.discriminator)) {
      throw new Error('invalid account discriminator');
    }

    const dec = QuoteAccountData.layout.decode(data.slice(8));

    return new QuoteAccountData({
      node: dec.node,
      queue: dec.queue,
      quoteBuffer: dec.quoteBuffer,
      quoteLen: dec.quoteLen,
      isReady: dec.isReady,
      verificationStatus: types.VerificationStatus.fromDecoded(
        dec.verificationStatus
      ),
      verificationTimestamp: dec.verificationTimestamp,
      ebuf: dec.ebuf,
    });
  }

  toJSON(): QuoteAccountDataJSON {
    return {
      node: this.node.toString(),
      queue: this.queue.toString(),
      quoteBuffer: this.quoteBuffer,
      quoteLen: this.quoteLen,
      isReady: this.isReady,
      verificationStatus: this.verificationStatus.toJSON(),
      verificationTimestamp: this.verificationTimestamp.toString(),
      ebuf: this.ebuf,
    };
  }

  static fromJSON(obj: QuoteAccountDataJSON): QuoteAccountData {
    return new QuoteAccountData({
      node: new PublicKey(obj.node),
      queue: new PublicKey(obj.queue),
      quoteBuffer: obj.quoteBuffer,
      quoteLen: obj.quoteLen,
      isReady: obj.isReady,
      verificationStatus: types.VerificationStatus.fromJSON(
        obj.verificationStatus
      ),
      verificationTimestamp: new BN(obj.verificationTimestamp),
      ebuf: obj.ebuf,
    });
  }
}

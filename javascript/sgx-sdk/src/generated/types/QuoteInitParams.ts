import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import { PublicKey } from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh';

export interface QuoteInitParamsFields {
  data: Array<number>;
  totalLen: number;
  chunkStart: number;
  chunkEnd: number;
}

export interface QuoteInitParamsJSON {
  data: Array<number>;
  totalLen: number;
  chunkStart: number;
  chunkEnd: number;
}

export class QuoteInitParams {
  readonly data: Array<number>;
  readonly totalLen: number;
  readonly chunkStart: number;
  readonly chunkEnd: number;

  constructor(fields: QuoteInitParamsFields) {
    this.data = fields.data;
    this.totalLen = fields.totalLen;
    this.chunkStart = fields.chunkStart;
    this.chunkEnd = fields.chunkEnd;
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.array(borsh.u8(), 512, 'data'),
        borsh.u32('totalLen'),
        borsh.u32('chunkStart'),
        borsh.u32('chunkEnd'),
      ],
      property
    );
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new QuoteInitParams({
      data: obj.data,
      totalLen: obj.totalLen,
      chunkStart: obj.chunkStart,
      chunkEnd: obj.chunkEnd,
    });
  }

  static toEncodable(fields: QuoteInitParamsFields) {
    return {
      data: fields.data,
      totalLen: fields.totalLen,
      chunkStart: fields.chunkStart,
      chunkEnd: fields.chunkEnd,
    };
  }

  toJSON(): QuoteInitParamsJSON {
    return {
      data: this.data,
      totalLen: this.totalLen,
      chunkStart: this.chunkStart,
      chunkEnd: this.chunkEnd,
    };
  }

  static fromJSON(obj: QuoteInitParamsJSON): QuoteInitParams {
    return new QuoteInitParams({
      data: obj.data,
      totalLen: obj.totalLen,
      chunkStart: obj.chunkStart,
      chunkEnd: obj.chunkEnd,
    });
  }

  toEncodable() {
    return QuoteInitParams.toEncodable(this);
  }
}

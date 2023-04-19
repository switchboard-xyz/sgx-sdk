import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import { PublicKey } from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh';

export interface QuoteVerifyParamsFields {
  queueIdx: number;
}

export interface QuoteVerifyParamsJSON {
  queueIdx: number;
}

export class QuoteVerifyParams {
  readonly queueIdx: number;

  constructor(fields: QuoteVerifyParamsFields) {
    this.queueIdx = fields.queueIdx;
  }

  static layout(property?: string) {
    return borsh.struct([borsh.u32('queueIdx')], property);
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new QuoteVerifyParams({
      queueIdx: obj.queueIdx,
    });
  }

  static toEncodable(fields: QuoteVerifyParamsFields) {
    return {
      queueIdx: fields.queueIdx,
    };
  }

  toJSON(): QuoteVerifyParamsJSON {
    return {
      queueIdx: this.queueIdx,
    };
  }

  static fromJSON(obj: QuoteVerifyParamsJSON): QuoteVerifyParams {
    return new QuoteVerifyParams({
      queueIdx: obj.queueIdx,
    });
  }

  toEncodable() {
    return QuoteVerifyParams.toEncodable(this);
  }
}

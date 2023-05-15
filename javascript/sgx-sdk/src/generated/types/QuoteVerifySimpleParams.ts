import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import { PublicKey } from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh';

export interface QuoteVerifySimpleParamsFields {
  timestamp: BN;
}

export interface QuoteVerifySimpleParamsJSON {
  timestamp: string;
}

export class QuoteVerifySimpleParams {
  readonly timestamp: BN;

  constructor(fields: QuoteVerifySimpleParamsFields) {
    this.timestamp = fields.timestamp;
  }

  static layout(property?: string) {
    return borsh.struct([borsh.i64('timestamp')], property);
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new QuoteVerifySimpleParams({
      timestamp: obj.timestamp,
    });
  }

  static toEncodable(fields: QuoteVerifySimpleParamsFields) {
    return {
      timestamp: fields.timestamp,
    };
  }

  toJSON(): QuoteVerifySimpleParamsJSON {
    return {
      timestamp: this.timestamp.toString(),
    };
  }

  static fromJSON(obj: QuoteVerifySimpleParamsJSON): QuoteVerifySimpleParams {
    return new QuoteVerifySimpleParams({
      timestamp: new BN(obj.timestamp),
    });
  }

  toEncodable() {
    return QuoteVerifySimpleParams.toEncodable(this);
  }
}

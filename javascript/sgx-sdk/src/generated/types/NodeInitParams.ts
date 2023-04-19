import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import { PublicKey } from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh';

export interface NodeInitParamsFields {}

export interface NodeInitParamsJSON {}

export class NodeInitParams {
  constructor(fields: NodeInitParamsFields) {}

  static layout(property?: string) {
    return borsh.struct([], property);
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new NodeInitParams({});
  }

  static toEncodable(fields: NodeInitParamsFields) {
    return {};
  }

  toJSON(): NodeInitParamsJSON {
    return {};
  }

  static fromJSON(obj: NodeInitParamsJSON): NodeInitParams {
    return new NodeInitParams({});
  }

  toEncodable() {
    return NodeInitParams.toEncodable(this);
  }
}

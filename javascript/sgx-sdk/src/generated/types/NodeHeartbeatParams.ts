import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import { PublicKey } from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh';

export interface NodeHeartbeatParamsFields {}

export interface NodeHeartbeatParamsJSON {}

export class NodeHeartbeatParams {
  constructor(fields: NodeHeartbeatParamsFields) {}

  static layout(property?: string) {
    return borsh.struct([], property);
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new NodeHeartbeatParams({});
  }

  static toEncodable(fields: NodeHeartbeatParamsFields) {
    return {};
  }

  toJSON(): NodeHeartbeatParamsJSON {
    return {};
  }

  static fromJSON(obj: NodeHeartbeatParamsJSON): NodeHeartbeatParams {
    return new NodeHeartbeatParams({});
  }

  toEncodable() {
    return NodeHeartbeatParams.toEncodable(this);
  }
}

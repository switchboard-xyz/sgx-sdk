import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import { PublicKey } from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh';

export interface QueueInitParamsFields {
  allowAuthorityOverrideAfter: BN;
  requireAuthorityHeartbeatPermission: boolean;
  requireUsagePermissions: boolean;
  maxQuoteVerificationAge: BN;
  reward: number;
  nodeTimeout: BN;
}

export interface QueueInitParamsJSON {
  allowAuthorityOverrideAfter: string;
  requireAuthorityHeartbeatPermission: boolean;
  requireUsagePermissions: boolean;
  maxQuoteVerificationAge: string;
  reward: number;
  nodeTimeout: string;
}

export class QueueInitParams {
  readonly allowAuthorityOverrideAfter: BN;
  readonly requireAuthorityHeartbeatPermission: boolean;
  readonly requireUsagePermissions: boolean;
  readonly maxQuoteVerificationAge: BN;
  readonly reward: number;
  readonly nodeTimeout: BN;

  constructor(fields: QueueInitParamsFields) {
    this.allowAuthorityOverrideAfter = fields.allowAuthorityOverrideAfter;
    this.requireAuthorityHeartbeatPermission =
      fields.requireAuthorityHeartbeatPermission;
    this.requireUsagePermissions = fields.requireUsagePermissions;
    this.maxQuoteVerificationAge = fields.maxQuoteVerificationAge;
    this.reward = fields.reward;
    this.nodeTimeout = fields.nodeTimeout;
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.i64('allowAuthorityOverrideAfter'),
        borsh.bool('requireAuthorityHeartbeatPermission'),
        borsh.bool('requireUsagePermissions'),
        borsh.i64('maxQuoteVerificationAge'),
        borsh.u32('reward'),
        borsh.i64('nodeTimeout'),
      ],
      property
    );
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new QueueInitParams({
      allowAuthorityOverrideAfter: obj.allowAuthorityOverrideAfter,
      requireAuthorityHeartbeatPermission:
        obj.requireAuthorityHeartbeatPermission,
      requireUsagePermissions: obj.requireUsagePermissions,
      maxQuoteVerificationAge: obj.maxQuoteVerificationAge,
      reward: obj.reward,
      nodeTimeout: obj.nodeTimeout,
    });
  }

  static toEncodable(fields: QueueInitParamsFields) {
    return {
      allowAuthorityOverrideAfter: fields.allowAuthorityOverrideAfter,
      requireAuthorityHeartbeatPermission:
        fields.requireAuthorityHeartbeatPermission,
      requireUsagePermissions: fields.requireUsagePermissions,
      maxQuoteVerificationAge: fields.maxQuoteVerificationAge,
      reward: fields.reward,
      nodeTimeout: fields.nodeTimeout,
    };
  }

  toJSON(): QueueInitParamsJSON {
    return {
      allowAuthorityOverrideAfter: this.allowAuthorityOverrideAfter.toString(),
      requireAuthorityHeartbeatPermission:
        this.requireAuthorityHeartbeatPermission,
      requireUsagePermissions: this.requireUsagePermissions,
      maxQuoteVerificationAge: this.maxQuoteVerificationAge.toString(),
      reward: this.reward,
      nodeTimeout: this.nodeTimeout.toString(),
    };
  }

  static fromJSON(obj: QueueInitParamsJSON): QueueInitParams {
    return new QueueInitParams({
      allowAuthorityOverrideAfter: new BN(obj.allowAuthorityOverrideAfter),
      requireAuthorityHeartbeatPermission:
        obj.requireAuthorityHeartbeatPermission,
      requireUsagePermissions: obj.requireUsagePermissions,
      maxQuoteVerificationAge: new BN(obj.maxQuoteVerificationAge),
      reward: obj.reward,
      nodeTimeout: new BN(obj.nodeTimeout),
    });
  }

  toEncodable() {
    return QueueInitParams.toEncodable(this);
  }
}

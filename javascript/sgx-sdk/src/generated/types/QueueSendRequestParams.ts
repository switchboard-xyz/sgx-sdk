import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import { PublicKey } from '@solana/web3.js'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh';

export interface QueueSendRequestParamsFields {
  targetNode: PublicKey;
  x25519EncryptedData: Uint8Array;
  decryptHash: Array<number>;
}

export interface QueueSendRequestParamsJSON {
  targetNode: string;
  x25519EncryptedData: Array<number>;
  decryptHash: Array<number>;
}

export class QueueSendRequestParams {
  readonly targetNode: PublicKey;
  readonly x25519EncryptedData: Uint8Array;
  readonly decryptHash: Array<number>;

  constructor(fields: QueueSendRequestParamsFields) {
    this.targetNode = fields.targetNode;
    this.x25519EncryptedData = fields.x25519EncryptedData;
    this.decryptHash = fields.decryptHash;
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey('targetNode'),
        borsh.vecU8('x25519EncryptedData'),
        borsh.array(borsh.u8(), 32, 'decryptHash'),
      ],
      property
    );
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new QueueSendRequestParams({
      targetNode: obj.targetNode,
      x25519EncryptedData: new Uint8Array(
        obj.x25519EncryptedData.buffer,
        obj.x25519EncryptedData.byteOffset,
        obj.x25519EncryptedData.length
      ),
      decryptHash: obj.decryptHash,
    });
  }

  static toEncodable(fields: QueueSendRequestParamsFields) {
    return {
      targetNode: fields.targetNode,
      x25519EncryptedData: Buffer.from(
        fields.x25519EncryptedData.buffer,
        fields.x25519EncryptedData.byteOffset,
        fields.x25519EncryptedData.length
      ),
      decryptHash: fields.decryptHash,
    };
  }

  toJSON(): QueueSendRequestParamsJSON {
    return {
      targetNode: this.targetNode.toString(),
      x25519EncryptedData: Array.from(this.x25519EncryptedData.values()),
      decryptHash: this.decryptHash,
    };
  }

  static fromJSON(obj: QueueSendRequestParamsJSON): QueueSendRequestParams {
    return new QueueSendRequestParams({
      targetNode: new PublicKey(obj.targetNode),
      x25519EncryptedData: Uint8Array.from(obj.x25519EncryptedData),
      decryptHash: obj.decryptHash,
    });
  }

  toEncodable() {
    return QueueSendRequestParams.toEncodable(this);
  }
}

import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
import { PublicKey, Connection } from '@solana/web3.js';
import { BN } from '@switchboard-xyz/common'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from '@coral-xyz/borsh'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from '../types'; // eslint-disable-line @typescript-eslint/no-unused-vars

export interface NodeAccountDataFields {
  authority: PublicKey;
  queue: PublicKey;
  lastHeartbeat: BN;
  numInUse: number;
  verifiedAt: BN;
  validUntil: BN;
  ebuf: Array<number>;
}

export interface NodeAccountDataJSON {
  authority: string;
  queue: string;
  lastHeartbeat: string;
  numInUse: number;
  verifiedAt: string;
  validUntil: string;
  ebuf: Array<number>;
}

export class NodeAccountData {
  readonly authority: PublicKey;
  readonly queue: PublicKey;
  readonly lastHeartbeat: BN;
  readonly numInUse: number;
  readonly verifiedAt: BN;
  readonly validUntil: BN;
  readonly ebuf: Array<number>;

  static readonly discriminator = Buffer.from([
    121, 138, 18, 240, 215, 193, 3, 182,
  ]);

  static readonly layout = borsh.struct([
    borsh.publicKey('authority'),
    borsh.publicKey('queue'),
    borsh.i64('lastHeartbeat'),
    borsh.u32('numInUse'),
    borsh.i64('verifiedAt'),
    borsh.i64('validUntil'),
    borsh.array(borsh.u8(), 256, 'ebuf'),
  ]);

  constructor(fields: NodeAccountDataFields) {
    this.authority = fields.authority;
    this.queue = fields.queue;
    this.lastHeartbeat = fields.lastHeartbeat;
    this.numInUse = fields.numInUse;
    this.verifiedAt = fields.verifiedAt;
    this.validUntil = fields.validUntil;
    this.ebuf = fields.ebuf;
  }

  static async fetch(
    program: SwitchboardQuoteProgram,
    address: PublicKey
  ): Promise<NodeAccountData | null> {
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
  ): Promise<Array<NodeAccountData | null>> {
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

  static decode(data: Buffer): NodeAccountData {
    if (!data.slice(0, 8).equals(NodeAccountData.discriminator)) {
      throw new Error('invalid account discriminator');
    }

    const dec = NodeAccountData.layout.decode(data.slice(8));

    return new NodeAccountData({
      authority: dec.authority,
      queue: dec.queue,
      lastHeartbeat: dec.lastHeartbeat,
      numInUse: dec.numInUse,
      verifiedAt: dec.verifiedAt,
      validUntil: dec.validUntil,
      ebuf: dec.ebuf,
    });
  }

  toJSON(): NodeAccountDataJSON {
    return {
      authority: this.authority.toString(),
      queue: this.queue.toString(),
      lastHeartbeat: this.lastHeartbeat.toString(),
      numInUse: this.numInUse,
      verifiedAt: this.verifiedAt.toString(),
      validUntil: this.validUntil.toString(),
      ebuf: this.ebuf,
    };
  }

  static fromJSON(obj: NodeAccountDataJSON): NodeAccountData {
    return new NodeAccountData({
      authority: new PublicKey(obj.authority),
      queue: new PublicKey(obj.queue),
      lastHeartbeat: new BN(obj.lastHeartbeat),
      numInUse: obj.numInUse,
      verifiedAt: new BN(obj.verifiedAt),
      validUntil: new BN(obj.validUntil),
      ebuf: obj.ebuf,
    });
  }
}

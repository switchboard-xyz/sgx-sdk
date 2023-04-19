import * as anchor from '@coral-xyz/anchor';
import {
  Keypair,
  PublicKey,
  SystemProgram,
  TransactionSignature,
} from '@solana/web3.js';
import * as errors from '../errors';
import * as types from '../generated';

import { SwitchboardQuoteProgram } from '../SwitchboardQuoteProgram';
import { TransactionObject } from '../TransactionObject';
import { Account } from './account';

/**
 *  Parameters for initializing an {@linkcode PermissionAccount}
 */
export interface PermissionAccountInitParams {
  granter: PublicKey;
  grantee: PublicKey;
  authority: PublicKey;
}

export interface PermissionSetParams {
  /** Whether to enable PERMIT_ORACLE_HEARTBEAT permissions. **Note:** Requires a provided queueAuthority keypair or payer to be the assigned queue authority. */
  enable: boolean;
  /** Keypair used to enable heartbeat permissions if payer is not the queue authority. */
  queueAuthority?: Keypair;
}

/**
 * Account type dictating the level of permissions between a granter and a grantee.
 *
 * A {@linkcode QueueAccount} acts as the granter where the queue authority assigns or revokes a grantee's {@linkcode types.SwitchboardPermission}. A grantee can be one of the following: {@linkcode AggregatorAccount}, {@linkcode BufferRelayerAccount}, or {@linkcode VrfAccount}.
 *
 * Data: {@linkcode types.PermissionAccountData}
 */
export class PermissionAccount extends Account<types.PermissionAccountData> {
  static accountName = 'PermissionAccountData';

  public static size = 372;

  /**
   * Returns the size of an on-chain {@linkcode PermissionAccount}.
   */
  public readonly size = this.program.account.permissionAccountData.size;

  /**
   * Retrieve and decode the {@linkcode types.PermissionAccountData} stored in this account.
   */
  public async loadData(): Promise<types.PermissionAccountData> {
    const data = await types.PermissionAccountData.fetch(
      this.program,
      this.publicKey
    );
    if (data === null)
      throw new errors.AccountNotFoundError('Permissions', this.publicKey);
    return data;
  }

  /** Load an existing PermissionAccount with its current on-chain state */
  public static async load(
    program: SwitchboardQuoteProgram,
    authority: PublicKey | string,
    granter: PublicKey | string,
    grantee: PublicKey | string
  ): Promise<[PermissionAccount, types.PermissionAccountData, number]> {
    const [account, bump] = PermissionAccount.fromSeed(
      program,
      typeof authority === 'string' ? new PublicKey(authority) : authority,
      typeof granter === 'string' ? new PublicKey(granter) : granter,
      typeof grantee === 'string' ? new PublicKey(grantee) : grantee
    );
    const state = await account.loadData();
    return [account, state, bump];
  }

  /**
   * Loads a PermissionAccount from the expected PDA seed format.
   * @param program The Switchboard program for the current connection.
   * @param authority The authority pubkey to be incorporated into the account seed.
   * @param granter The granter pubkey to be incorporated into the account seed.
   * @param grantee The grantee pubkey to be incorporated into the account seed.
   * @return PermissionAccount and PDA bump.
   */
  public static fromSeed(
    program: SwitchboardQuoteProgram,
    authority: PublicKey,
    granter: PublicKey,
    grantee: PublicKey
  ): [PermissionAccount, number] {
    const [publicKey, bump] = anchor.utils.publicKey.findProgramAddressSync(
      [
        Buffer.from('PermissionAccountData'),
        authority.toBytes(),
        granter.toBytes(),
        grantee.toBytes(),
      ],
      program.programId
    );
    return [new PermissionAccount(program, publicKey), bump];
  }

  public static createInstruction(
    program: SwitchboardQuoteProgram,
    payer: PublicKey,
    params: PermissionAccountInitParams
  ): [PermissionAccount, TransactionObject] {
    const [account] = PermissionAccount.fromSeed(
      program,
      params.authority,
      params.granter,
      params.grantee
    );
    const instruction = types.permissionInit(
      program,
      { params: {} },
      {
        permission: account.publicKey,
        queue: params.granter,
        node: params.grantee,
        systemProgram: SystemProgram.programId,
        authority: params.authority,
        payer: payer,
      }
    );
    return [account, new TransactionObject(payer, [instruction], [])];
  }

  public static async create(
    program: SwitchboardQuoteProgram,
    params: PermissionAccountInitParams
  ): Promise<[PermissionAccount, TransactionSignature]> {
    const [account, txnObject] = this.createInstruction(
      program,
      program.walletPubkey,
      params
    );
    const txSignature = await program.signAndSend(txnObject);
    return [account, txSignature];
  }

  /**
   * Check if a specific permission is enabled on this permission account
   */
  public async isPermissionEnabled(permission): Promise<boolean> {
    const permissions = (await this.loadData()).permissions;
    return (permissions & (permission as number)) !== 0;
  }

  /**
   * Produces the instruction to set the permission in the PermissionAccount
   */
  public async setInstruction(
    payer: PublicKey,
    params: PermissionSetParams & {
      /** The {@linkcode types.SwitchboardPermission} to set for the grantee. */
      permission: types.SwitchboardPermissionKind;
    }
  ): Promise<TransactionObject> {
    const data = await this.loadData();
    return new TransactionObject(
      payer,
      [
        types.permissionSet(
          this.program,
          {
            params: {
              permission: params.permission.discriminator,
              enable: params.enable,
            },
          },
          {
            permission: this.publicKey,
            authority: params.queueAuthority
              ? params.queueAuthority.publicKey
              : payer,
            queue: data.granter,
            node: data.grantee,
          }
        ),
      ],
      params.queueAuthority ? [params.queueAuthority] : []
    );
  }

  /**
   * Sets the permission in the PermissionAccount
   */
  public async set(
    params: PermissionSetParams & {
      /** The {@linkcode types.SwitchboardPermission} to set for the grantee. */
      permission: types.SwitchboardPermissionKind;
    }
  ): Promise<string> {
    const setTxn = await this.setInstruction(this.program.walletPubkey, params);
    const txnSignature = await this.program.signAndSend(setTxn);
    return txnSignature;
  }
}

import * as anchor from '@coral-xyz/anchor';
import { SwitchboardQuoteProgram } from '../SwitchboardQuoteProgram';

export abstract class Account<T> {
  public readonly publicKey: anchor.web3.PublicKey;

  /**
   * Account constructor
   * @param program SwitchboardQuoteProgram
   * @param publicKey PublicKey of the on-chain resource
   */
  public constructor(
    public readonly program: SwitchboardQuoteProgram,
    publicKey: anchor.web3.PublicKey | string
  ) {
    this.publicKey =
      typeof publicKey === 'string'
        ? new anchor.web3.PublicKey(publicKey)
        : publicKey;
  }

  /**
   * @return on-chain account size.
   */
  public abstract get size(): number;

  /**
   * Retrieve and decode the data in this account.
   */
  public abstract loadData(): Promise<T>;
}

/** Callback to pass deserialized account data when updated on-chain */
export type OnAccountChangeCallback<T> = (accountData: T) => void;

import * as anchor from '@coral-xyz/anchor';
import {
  Cluster,
  ConfirmOptions,
  Connection,
  Keypair,
  PublicKey,
  SendOptions,
  Transaction,
  TransactionSignature,
} from '@solana/web3.js';
import { MAINNET_GENESIS_HASH, DEVNET_GENESIS_HASH } from './const';
import * as errors from './errors';
import { TransactionObject, TransactionOptions } from './TransactionObject';

export type SendTransactionOptions = (ConfirmOptions | SendOptions) & {
  skipConfrimation?: boolean;
};
export const DEFAULT_SEND_TRANSACTION_OPTIONS: SendTransactionOptions = {
  skipPreflight: false,
  maxRetries: 10,
  skipConfrimation: false,
};

/**
 *  A generated keypair that is assigned as the _payerKeypair_ when in read-only mode.
 */
export const READ_ONLY_KEYPAIR = Keypair.generate();

/**
 * Wrapper class for the Switchboard anchor Program.
 *
 * Basic usage example:
 *
 * ```ts
 * import { Connection } from "@solana/web3.js";
 * import { SwitchboardQuoteProgram, TransactionObject } from '@switchboard-xyz/solana.js';
 *
 * const program = await SwitchboardQuoteProgram.load(
 *    "mainnet-beta",
 *    new Connection("https://api.mainnet-beta.solana.com"),
 *    payerKeypair
 * );
 *
 * const txn = new TransactionObject(program.walletPubkey, [], []);
 * const txnSignature = await program.signAndSend(txn);
 * ```
 */
export class SwitchboardQuoteProgram {
  private static readonly _readOnlyKeypair = READ_ONLY_KEYPAIR;

  private readonly _program: anchor.Program;

  /** The solana cluster to load the Switchboard program for. */
  readonly cluster: Cluster | 'localnet';

  /**
   * Constructor.
   */
  constructor(program: anchor.Program, cluster: Cluster | 'localnet') {
    this._program = program;
    this.cluster = cluster;
  }

  static async loadAnchorProgram(
    cluster: Cluster | 'localnet',
    connection: Connection,
    payerKeypair: Keypair,
    programId: PublicKey
  ): Promise<anchor.Program> {
    const pid = programId;
    const provider = new anchor.AnchorProvider(
      connection,
      // If no keypair is provided, default to dummy keypair
      new AnchorWallet(
        payerKeypair ?? SwitchboardQuoteProgram._readOnlyKeypair
      ),
      { commitment: 'confirmed' }
    );
    const anchorIdl = await anchor.Program.fetchIdl(pid, provider);
    if (!anchorIdl) {
      throw new Error(`Failed to find IDL for ${pid.toBase58()}`);
    }
    const program = new anchor.Program(anchorIdl, pid, provider);

    return program;
  }

  /**
   * Create and initialize a {@linkcode SwitchboardQuoteProgram} connection object.
   *
   * @param cluster - the solana cluster to load the Switchboard program for.
   *
   * @param connection - the Solana connection object used to connect to an RPC node.
   *
   * @param payerKeypair - optional, payer keypair used to pay for on-chain transactions.
   *
   * @param programId - optional, override the cluster's default programId.
   *
   * @return the {@linkcode SwitchboardQuoteProgram} used to create and interact with Switchboard accounts.
   *
   * Basic usage example:
   *
   * ```ts
   * import { Connection } from "@solana/web3.js";
   * import { SwitchboardQuoteProgram, TransactionObject } from '@switchboard-xyz/solana.js';
   *
   * const program = await SwitchboardQuoteProgram.load(
   *    "mainnet-beta",
   *    new Connection("https://api.mainnet-beta.solana.com"),
   *    payerKeypair
   * );
   *
   * const txn = new TransactionObject(program.walletPubkey, [], []);
   * const txnSignature = await program.signAndSend(txn);
   * ```
   */
  static load = async (
    cluster: Cluster | 'localnet',
    connection: Connection,
    payerKeypair: Keypair,
    programId: PublicKey
  ): Promise<SwitchboardQuoteProgram> => {
    const program = await SwitchboardQuoteProgram.loadAnchorProgram(
      cluster,
      connection,
      payerKeypair,
      programId
    );
    return new SwitchboardQuoteProgram(program, cluster);
  };

  /**
   * Create and initialize a {@linkcode SwitchboardQuoteProgram} connection object.
   *
   * @param provider - the anchor provider containing the rpc and wallet connection.
   *
   * @return the {@linkcode SwitchboardQuoteProgram} used to create and interact with Switchboard accounts.
   *
   * Basic usage example:
   *
   * ```ts
   * import * as anchor from "@coral-xyz/anchor";
   * import { Connection } from "@solana/web3.js";
   * import { AnchorWallet, SwitchboardQuoteProgram, TransactionObject } from '@switchboard-xyz/solana.js';
   *
   * const connection = new Connection("https://api.mainnet-beta.solana.com");
   * const provider = new anchor.AnchorProvider(
      connection,
      new AnchorWallet(payerKeypair ?? SwitchboardQuoteProgram._readOnlyKeypair),
      { commitment: 'confirmed' }
    );
   * const program = await SwitchboardQuoteProgram.fromProvider(provider);
   *
   * const txn = new TransactionObject(program.walletPubkey, [], []);
   * const txnSignature = await program.signAndSend(txn);
   * ```
   */
  static fromProvider = async (
    provider: anchor.AnchorProvider,
    programId: PublicKey
  ): Promise<SwitchboardQuoteProgram> => {
    const payer = (provider.wallet as AnchorWallet).payer;
    const program = await SwitchboardQuoteProgram.fromConnection(
      provider.connection,
      payer,
      programId
    );
    return program;
  };

  /**
   * Create and initialize a {@linkcode SwitchboardQuoteProgram} connection object.
   *
   * @param provider - the anchor provider containing the rpc and wallet connection.
   *
   * @return the {@linkcode SwitchboardQuoteProgram} used to create and interact with Switchboard accounts.
   *
   * Basic usage example:
   *
   * ```ts
   * import * as anchor from "@coral-xyz/anchor";
   * import { Connection } from "@solana/web3.js";
   * import { AnchorWallet, SwitchboardQuoteProgram, TransactionObject } from '@switchboard-xyz/solana.js';
   *
   * const connection = new Connection("https://api.mainnet-beta.solana.com");
   * const program = await SwitchboardQuoteProgram.fromConnection(connection);
   * ```
   */
  static fromConnection = async (
    connection: Connection,
    payer: Keypair,
    programId: PublicKey
  ): Promise<SwitchboardQuoteProgram> => {
    const genesisHash = await connection.getGenesisHash();
    const cluster =
      genesisHash === MAINNET_GENESIS_HASH
        ? 'mainnet-beta'
        : genesisHash === DEVNET_GENESIS_HASH
        ? 'devnet'
        : 'localnet';

    const programAccountInfo = await connection.getAccountInfo(programId);
    if (programAccountInfo === null) {
      throw new Error(
        `Failed to load Switchboard at ${programId}, try manually providing a programId`
      );
    }

    const program = await SwitchboardQuoteProgram.load(
      cluster,
      connection,
      payer,
      programId
    );
    return program;
  };

  /**
   * The Switchboard Program ID for the currently connected cluster.
   */
  public get programId(): PublicKey {
    return this._program.programId;
  }
  /**
   * The Switchboard Program IDL.
   */
  public get idl(): anchor.Idl {
    return this._program.idl;
  }
  /**
   * The Switchboard Program ID for the currently connected cluster.
   */
  public get coder(): anchor.BorshAccountsCoder {
    return new anchor.BorshAccountsCoder(this._program.idl);
  }
  /**
   * The anchor Provider used by this program to connect with Solana cluster.
   */
  public get provider(): anchor.AnchorProvider {
    return this._program.provider as anchor.AnchorProvider;
  }
  /**
   * The Connection used by this program to connect with Solana cluster.
   */
  public get connection(): Connection {
    return this._program.provider.connection;
  }
  /**
   * The Connection used by this program to connect with Solana cluster.
   */
  public get wallet(): AnchorWallet {
    return this.provider.wallet as AnchorWallet;
  }

  public get walletPubkey(): PublicKey {
    return this.wallet.payer.publicKey;
  }
  /**
   * Some actions exposed by this SDK require that a payer Keypair has been
   * provided to {@linkcode SwitchboardQuoteProgram} in order to send transactions.
   */
  public get isReadOnly(): boolean {
    return (
      this.provider.publicKey.toBase58() ===
      SwitchboardQuoteProgram._readOnlyKeypair.publicKey.toBase58()
    );
  }

  /** Verify a payer keypair was supplied. */
  public verifyPayer(): void {
    if (this.isReadOnly) {
      throw new errors.SwitchboardProgramReadOnlyError();
    }
  }

  /**
   * Verify a fresh keypair was provided.
   *
   * **NOTE:** Creating new accounts without this check will prevent the ability to remove any existing funds. */
  public async verifyNewKeypair(keypair: Keypair): Promise<void> {
    const accountInfo = await this.connection.getAccountInfo(keypair.publicKey);
    if (accountInfo) {
      throw new errors.ExistingKeypair();
    }
  }

  public get account(): anchor.AccountNamespace {
    return this._program.account;
  }

  public async signAndSendAll(
    txns: Array<TransactionObject>,
    opts: SendTransactionOptions = DEFAULT_SEND_TRANSACTION_OPTIONS,
    txnOptions?: TransactionOptions,
    delay = 0
  ): Promise<Array<TransactionSignature>> {
    const txnSignatures = await TransactionObject.signAndSendAll(
      this.provider,
      txns,
      opts,
      txnOptions,
      delay
    );
    return txnSignatures;
  }

  public async signAndSend(
    txn: TransactionObject,
    opts: SendTransactionOptions = DEFAULT_SEND_TRANSACTION_OPTIONS,
    txnOptions?: TransactionOptions
  ): Promise<TransactionSignature> {
    const txnSignature = await txn.signAndSend(this.provider, opts, txnOptions);
    return txnSignature;
  }
}

export class AnchorWallet implements anchor.Wallet {
  constructor(readonly payer: Keypair) {
    this.payer = payer;
  }
  get publicKey(): PublicKey {
    return this.payer.publicKey;
  }
  private sign = (txn: Transaction): Transaction => {
    txn.partialSign(this.payer);
    return txn;
  };
  async signTransaction(txn: Transaction) {
    return this.sign(txn);
  }
  async signAllTransactions(txns: Transaction[]) {
    return txns.map(this.sign);
  }
}

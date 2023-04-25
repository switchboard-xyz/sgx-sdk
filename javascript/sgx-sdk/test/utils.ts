import {
  clusterApiUrl,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from '@solana/web3.js';

import assert from 'assert';
import dotenv from 'dotenv';
import fs from 'fs';
import os from 'os';
import path from 'path';
import { SwitchboardQuoteProgram, PROGRAM_ID } from '../src';
dotenv.config();

type SolanaCluster = 'localnet' | 'devnet' | 'mainnet-beta';

export const sleep = (ms: number): Promise<any> =>
  new Promise(s => setTimeout(s, ms));

export const DEFAULT_KEYPAIR_PATH = path.join(
  os.homedir(),
  '.config/solana/id.json'
);

export interface TestContext {
  cluster: SolanaCluster;
  program: SwitchboardQuoteProgram;
  payer: Keypair;
  toUrl: (signature: string) => string;
  round: (num: number, decimalPlaces: number) => number;
}

export function isLocalnet(): boolean {
  if (process.env.SOLANA_LOCALNET) {
    switch (process.env.SOLANA_LOCALNET) {
      case '1':
      case 'true':
      case 'localnet': {
        return true;
      }
    }
  }
  return false;
}

export function getCluster(): SolanaCluster {
  if (process.env.SOLANA_CLUSTER) {
    const cluster = String(process.env.SOLANA_CLUSTER);
    if (
      cluster === 'localnet' ||
      cluster === 'devnet' ||
      cluster === 'mainnet-beta'
    ) {
      return cluster;
    } else {
      throw new Error(
        `SOLANA_CLUSTER must be localnet, devnet, or mainnet-beta`
      );
    }
  }

  if (isLocalnet()) {
    return 'localnet';
  }

  return 'devnet';
}

export function getProgramId(cluster: SolanaCluster): PublicKey {
  if (process.env.SWITCHBOARD_QUOTE_VERIFIER_PROGRAM_ID) {
    return new PublicKey(process.env.SWITCHBOARD_QUOTE_VERIFIER_PROGRAM_ID);
  }

  return PROGRAM_ID;
}

export function getRpcUrl(cluster: SolanaCluster): string {
  if (isLocalnet()) {
    return 'http://0.0.0.0:8899';
  }
  if (process.env.SOLANA_RPC_URL) {
    return String(process.env.SOLANA_RPC_URL);
  }

  if (cluster === 'localnet') {
    return 'http://0.0.0.0:8899';
  }

  return clusterApiUrl(cluster);
}

export async function setupTest(): Promise<TestContext> {
  const cluster = getCluster();
  const payer: Keypair = fs.existsSync(DEFAULT_KEYPAIR_PATH)
    ? Keypair.fromSecretKey(
        new Uint8Array(
          JSON.parse(fs.readFileSync(DEFAULT_KEYPAIR_PATH, 'utf8'))
        )
      )
    : Keypair.generate();

  const programId = getProgramId(cluster);

  const program = await SwitchboardQuoteProgram.load(
    cluster,
    new Connection(getRpcUrl(cluster), { commitment: 'confirmed' }),
    payer,
    programId
  );

  // request airdrop if low on funds
  const payerBalance = await program.connection.getBalance(payer.publicKey);
  if (payerBalance === 0) {
    const airdropTxn = await program.connection.requestAirdrop(
      payer.publicKey,
      1 * LAMPORTS_PER_SOL
    );
    console.log(`Airdrop requested: ${airdropTxn}`);
    await program.connection.confirmTransaction(airdropTxn);
  }

  return {
    cluster,
    program,
    payer,
    toUrl: signature =>
      cluster === 'localnet'
        ? `https://explorer.solana.com/tx/${signature}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`
        : `https://explorer.solana.com/tx/${signature}${
            cluster === 'devnet' ? '?cluster=devnet' : ''
          }`,
    round: (num: number, decimalPlaces = 2): number => {
      assert(decimalPlaces > 0 && decimalPlaces < 16);
      const base = Math.pow(10, decimalPlaces);
      return Math.round((num + Number.EPSILON) * base) / base;
    },
  };
}

import detect from 'detect-port';
import { execSync, spawn } from 'child_process';
import fs from 'fs';
import os from 'os';
import path from 'path';
import _ from 'lodash';
import { sleep } from '@switchboard-xyz/common';
import { Keypair } from '@solana/web3.js';

const accountsToCopy = [
  'SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f', // Switchboard PID
  'Fi8vncGpNKbq62gPo56G4toCehWNy77GgqGkTaAF5Lkk', // Switchboard IDL
  'CyZuD7RPDcrqCGbNvLCyqk6Py9cEZTKmNKujfPi3ynDd', // Switchboard SbState
  '7hkp1xfPBcD2t1vZMoWWQPzipHVcXeLAAaiGXdPSfDie', // Switchboard tokenVault
  // 'Hxfwq7cxss4Ef9iDvaLb617dhageGyNWbDLLrg2sdQgT', // Switchboard VerifierQueuer PID
  // '5Kug4HWcdcMv5TNaHtV4x6gpmtPvigdrHH5BHjEMnLGT', // Switchboard VerifierQueue IDL
  'GMPkWUe4KdTUqD6jTvAV6PJTcA9rT5LCWZhjFuHQocGb',
  'Be1e7zt1a1FSTZ6hPLRGYWw5WoMsrxyRs71gZ5juoxUs',
];

(async () => {
  const shouldReset = process.argv.includes('--reset');
  const shouldKill = process.argv.includes('--kill');

  if (shouldKill) {
    killProcessUsingPort(8899);
    return;
  }

  if (shouldReset) {
    killProcessUsingPort(8899);
  } else {
    const isValidatorRunning = (await detect(8899)) !== 8899;
    if (isValidatorRunning) {
      console.log(
        `Solana RPC Validator port (8899) in use, skipping solana-test-validator`
      );
      return;
    }
  }

  fs.mkdirSync('.anchor/test-ledger', { recursive: true });

  const payerPubkey = Keypair.fromSecretKey(
    new Uint8Array(
      JSON.parse(
        fs.readFileSync(
          `${path.join(os.homedir(), '.config', 'solana', 'id.json')}`,
          'utf-8'
        )
      )
    )
  ).publicKey.toBase58();

  // start validator
  spawn(
    `solana-test-validator`,
    [
      '-q',
      '-r',
      '--ledger',
      '.anchor/test-ledger',
      '--mint',
      `${payerPubkey}`,
      '--bind-address',
      '0.0.0.0',
      '--rpc-port',
      '8899',
      '--url',
      'https://api.devnet.solana.com',
      ..._.flatten(accountsToCopy.map(a => ['--clone', a])),
    ],
    { stdio: 'inherit' }
  ).unref();

  await awaitValidatorReady();

  return;
})();

async function awaitValidatorReady() {
  let numRetries = 30;
  while (numRetries) {
    try {
      const response = await fetch(`http://0.0.0.0:${8899}`, {
        method: 'POST',
        headers: {
          Accept: 'application/json',
          'Content-Type': 'application/json',
        },
        body: '{"jsonrpc":"2.0","id":1, "method":"getBlockHeight"}',
      });

      if (response.ok) {
        const responseJson = await response.json();
        if (
          'result' in responseJson &&
          Number.parseInt(responseJson.result) > 0
        ) {
          return;
        }
      }
    } catch (error) {
      // console.error(error);
    }

    --numRetries;
    await sleep(1000);
  }

  throw new Error(`Failed to start solana validator`);
}

function killProcessUsingPort(port: number): void {
  try {
    execSync(`fuser -s -k ${port}/tcp`, { encoding: 'utf-8' });
    // const pid = parseInt(stdout.trim(), 10);

    // if (isNaN(pid)) {
    //   console.error(`Unable to parse PID from output: ${stdout}`);
    //   return;
    // }

    // process.kill(pid);
  } catch (stderr) {
    console.error('Error getting process ID:', stderr);
    return;
  }
}

import { PublicKey } from '@solana/web3.js';
import * as sb from './src';
import * as anchor from '@coral-xyz/anchor';

/** Get the program data address for a given programId
 * @param programId the programId for a given on-chain program
 * @return the publicKey of the address holding the upgradeable program buffer
 */
export const getProgramDataAddress = (programId: PublicKey): PublicKey => {
  return anchor.utils.publicKey.findProgramAddressSync(
    [programId.toBytes()],
    new PublicKey('BPFLoaderUpgradeab1e11111111111111111111111')
  )[0];
};

(async () => {
  const programId = new PublicKey(
    'Hxfwq7cxss4Ef9iDvaLb617dhageGyNWbDLLrg2sdQgT'
  );
  const base = (await PublicKey.findProgramAddress([], programId))[0];
  const idlAddress = PublicKey.createWithSeed(base, 'anchor:idl', programId);
  console.log((await idlAddress).toBase58());

  console.log(getProgramDataAddress(programId).toBase58());
})();

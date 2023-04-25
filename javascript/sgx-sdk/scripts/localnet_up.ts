import { PublicKey } from '@solana/web3.js';
import * as sb from '../src';

(async () => {
  const programId = new PublicKey(
    'Hxfwq7cxss4Ef9iDvaLb617dhageGyNWbDLLrg2sdQgT'
  );
  const base = (await PublicKey.findProgramAddress([], programId))[0];
  const idlAddress = PublicKey.createWithSeed(base, 'anchor:idl', programId);
  console.log((await idlAddress).toBase58());
})();

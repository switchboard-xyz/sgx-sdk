const shell = require('shelljs');
const path = require('path');
const fs = require('fs');
const { execSync } = require('child_process');

const projectRoot = __dirname;
// const shx = path.join(projectRoot, 'node_modules', '.bin', 'shx');

// Super hacky. Some files need to be reset to the previous git state and will be manually managed
// const ignoreFiles = [
//   './src/generated/types/SwitchboardPermission.ts', // we manually added NONE enumeration
//   './src/generated/types/SwitchboardDecimal.ts', // added toBig methods
//   './src/generated/types/Lanes.ts', // anchor-client-gen struggles with dual exports
//   './src/generated/types/index.ts', // TODO: Need a better way to handle this. anchor-client-gen adds multiple, broken exports (for VRF builder)
//   './src/generated/errors/index.ts', // need to revert the program ID check
// ];

/**
 * Fetch a list of filepaths for a given directory and desired file extension
 * @param [dirPath] Filesystem path to a directory to search.
 * @param [arrayOfFiles] An array of existing file paths for recursive calls
 * @param [extensions] Optional, an array of desired extensions with the leading separator '.'
 * @throws {String}
 * @returns {string[]}
 */
const getAllFiles = (dirPath, arrayOfFiles, extensions) => {
  const files = fs.readdirSync(dirPath, 'utf8');

  arrayOfFiles = arrayOfFiles || [];

  files.forEach(file => {
    if (fs.statSync(dirPath + '/' + file).isDirectory()) {
      arrayOfFiles = getAllFiles(
        dirPath + '/' + file,
        arrayOfFiles,
        extensions
      );
    } else {
      const ext = path.extname(file);
      if (extensions && Array.isArray(extensions) && extensions.includes(ext)) {
        arrayOfFiles.push(path.join(dirPath, '/', file));
      } else {
        arrayOfFiles.push(path.join(dirPath, '/', file));
      }
      // if (!(extensions === undefined) || extensions.includes(ext)) {
      //   arrayOfFiles.push(path.join(dirPath, '/', file));
      // }
    }
  });

  return arrayOfFiles;
};

async function main() {
  shell.cd(projectRoot);

  if (!shell.which('anchor')) {
    shell.echo(
      "Sorry, this script requires 'anchor' to be installed in your $PATH"
    );
    shell.exit(1);
  }

  // TODO: Update this program ID
  execSync(
    'rm -rf ./src/generated && npx anchor-client-gen --program-id Hxfwq7cxss4Ef9iDvaLb617dhageGyNWbDLLrg2sdQgT /Users/gally/dev/switchboard/switchboard-core/switchboard_v2/target/idl/switchboard_quote_verifier.json ./src/generated'
  );
  fs.writeFileSync(
    './src/generated/index.ts',
    [
      "export * from './accounts';",
      "export * from './errors';",
      "export * from './instructions';",
      "export * from './types';",
    ].join('\n')
  );

  // loop through directory and run regex replaces
  for await (const file of [
    ...getAllFiles('./src/generated/accounts'),
    ...getAllFiles('./src/generated/errors'),
    ...getAllFiles('./src/generated/instructions'),
    ...getAllFiles('./src/generated/types'),
  ]) {
    if (file.includes('index.ts')) {
      continue;
    }
    const fileString = fs.readFileSync(file, 'utf-8');
    fs.writeFileSync(
      file,
      `import { SwitchboardQuoteProgram } from "../../SwitchboardQuoteProgram"\n${fileString}`
    );

    console.log(file);
    // replace BN import
    execSync(
      `sed -i '' 's/import BN from \\"bn.js\\"/import { BN } from \\"@switchboard-xyz\\/common\\"/g' ${file}`
    );
    // replace borsh import
    execSync(`sed -i '' 's/@project-serum/@coral-xyz/g' ${file}`);
    // remove PROGRAM_ID import, we will use SwitchboardQuoteProgram instead
    execSync(
      `sed -i '' 's/import { PROGRAM_ID } from "..\\/programId"/ /g' ${file}`
    );
    // replace PROGRAM_ID with program.programId
    execSync(`sed -i '' 's/PROGRAM_ID/program.programId/g' ${file}`);
    // replace Connection with SwitchboardQuoteProgram
    execSync(
      `sed -i '' 's/c: Connection,/program: SwitchboardQuoteProgram,/g' ${file}`
    );
    // replace c.getAccountInfo with the SwitchboardQuoteProgram connection
    execSync(
      `sed -i '' 's/c.getAccountInfo/program.connection.getAccountInfo/g' ${file}`
    );
    // replace c.getMultipleAccountsInfo with the SwitchboardQuoteProgram connection
    execSync(
      `sed -i '' 's/c.getMultipleAccountsInfo/program.connection.getMultipleAccountsInfo/g' ${file}`
    );

    // add program as first arguement to instructions
    if (file.includes('/instructions/')) {
      execSync(
        `sed -i '' 's/args:/program: SwitchboardQuoteProgram, args:/g' ${file}`
      );
    }
  }

  execSync('npx prettier ./src/generated --write');

  // reset files
  // for (const file of ignoreFiles) {
  //   execSync(`git restore ${file}`);
  // }
}

main()
  .then(() => {
    // console.log("Executed successfully");
  })
  .catch(err => {
    console.error(err);
  });

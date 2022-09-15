// import * as anchor, { Program } from "@project-serum/anchor";
// import { Myepicproject } from "../target/types/myepicproject";

// describe("myepicproject", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.Myepicproject as Program<Myepicproject>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });

// // * FROM https://buildspace.so/p/build-solana-web3-app/lessons/write-first-solana-program
// import * as anchor from '@project-serum/anchor';

// const main = async() => {
//   console.log("🚀 Starting test...")

//   anchor.setProvider(anchor.AnchorProvider.env());
//   const program = anchor.workspace.Myepicproject;
//   const tx = await program.rpc.startStuffOff();

//   console.log("📝 Your transaction signature", tx);
// }

// const runMain = async () => {
//   try {
//     await main();
//     process.exit(0);
//   } catch (error) {
//     console.error(error);
//     process.exit(1);
//   }
// };

// runMain();

// // * FROM https://buildspace.so/p/build-solana-web3-app/lessons/store-basic-data-on-contract
// import * as anchor from '@project-serum/anchor';

// // Need the system program, will talk about this soon.
// const { SystemProgram } = anchor.web3;

// const main = async() => {
//   console.log("🚀 Starting test...")

//   // Create and set the provider. We set it before but we needed to update it, so that it can communicate with our frontend!
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.Myepicproject;
	
//   // Create an account keypair for our program to use.
//   const baseAccount = anchor.web3.Keypair.generate();

//   // Call start_stuff_off, pass it the params it needs!
//   let tx = await program.rpc.startStuffOff({
//     accounts: {
//       baseAccount: baseAccount.publicKey,
//       user: provider.wallet.publicKey,
//       systemProgram: SystemProgram.programId,
//     },
//     signers: [baseAccount],
//   });

//   console.log("📝 Your transaction signature", tx);

//   // Fetch data from the account.
//   let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
//   console.log('👀 GIF Count', account.totalGifs.toString())

//   // Call add_gif!
//   await program.rpc.addGif({
//     accounts: {
//       baseAccount: baseAccount.publicKey,
//     },
//   });
  
//   // Get the account again to see what changed.
//   account = await program.account.baseAccount.fetch(baseAccount.publicKey);
//   console.log('👀 GIF Count', account.totalGifs.toString())
// }

// const runMain = async () => {
//   try {
//     await main();
//     process.exit(0);
//   } catch (error) {
//     console.error(error);
//     process.exit(1);
//   }
// };

// runMain();

const TEST_GIFS = [
  'https://media.giphy.com/media/bPCwGUF2sKjyE/giphy.gif',
  'https://media.giphy.com/media/4no7ul3pa571e/giphy.gif',
  'https://media.giphy.com/media/5Zesu5VPNGJlm/giphy.gif',
  'https://media.giphy.com/media/VbnUQpnihPSIgIXuZv/giphy.gif',
  'https://media.giphy.com/media/JIX9t2j0ZTN9S/giphy.gif',
  'https://media.giphy.com/media/ktcUyw6mBlMVa/giphy.gif',
];

// * FROM https://buildspace.so/p/build-solana-web3-app/lessons/store-structs-program
import * as anchor from '@project-serum/anchor';
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // console.log('provider keys', JSON.stringify(Object.getOwnPropertyNames(provider)));
  // return;

  const program = anchor.workspace.Myepicproject;
  const baseAccount = anchor.web3.Keypair.generate();
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // You'll need to now pass a GIF link to the function! You'll also need to pass in the user submitting the GIF!
  await program.rpc.addGif(TEST_GIFS[0], {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // You'll need to now pass a GIF link to the function! You'll also need to pass in the user submitting the GIF!
  await program.rpc.addGif(TEST_GIFS[1], {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // Access gif_list on the account!
  console.log('👀 GIF List', account.gifList)

  // Vote on GIF #0
  await program.rpc.upvote(new anchor.BN(0), {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  // Vote on GIF #0
  await program.rpc.upvote(new anchor.BN(0), {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  // Vote on GIF #1
  await program.rpc.upvote(new anchor.BN(1), {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    }
  });

  // Vote on GIF #0
  await program.rpc.upvote(new anchor.BN(0), {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    }
  });
  
  // Down-Vote on GIF #0
  await program.rpc.downvote(new anchor.BN(0), {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // Access gif_list on the account!
  console.log('👀 GIF List', account.gifList)
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();

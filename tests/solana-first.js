const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("🚀 Starting test...");

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaFirst;

  // Create an account keypair for our program to use.
  const baseAccount = anchor.web3.Keypair.generate();

  // Call start_stuff_off, pass it the params it needs!
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  // Fetch data from the account.
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString());

  await program.rpc.addGif('https://media.giphy.com/media/c2W9nPICZahHUyePnH/giphy-downsized-large.gif', {
    accounts: {
      baseAccount: baseAccount.publicKey,
    },
  });

  // Get the account again to see what changed.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString());

  // Access gif_list on the account!
  console.log('👀 GIF List', account.gifList);


  // Upvote a gif
  await program.rpc.upvoteGif(account.gifList[0].gifLink, {
    accounts: { baseAccount: baseAccount.publicKey }
  });
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('Upvoted gif', account.gifList[0]);
};

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

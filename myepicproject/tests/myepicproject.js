const anchor = require('@project-serum/anchor');

// Precisa do programa do sistema, falaremos sobre isso em breve.
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Iniciando testes...")

  // Crie e defina o provedor. Nós o configuramos antes, mas precisávamos atualizá-lo, para que ele pudesse se comunicar com nosso frontend!
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Myepicproject;
	
  // Crie um par de chaves de conta para nosso programa usar.
  const baseAccount = anchor.web3.Keypair.generate();

  // Chame start_stuff_off, passe os parâmetros necessários!
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Sua assinatura de transação", tx);

  // Obtem dados da conta.
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count:', account.totalGifs.toString())

	
  // Você precisará agora passar um link do GIF para a função! Você também precisará passar o usuário que está enviando o GIF!
  await program.rpc.addGif("https://media.giphy.com/media/QC7rGl1AEoGOXGWHUC/giphy-downsized-large.gif", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  // Chama a conta
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("👀 GIF Count", account.totalGifs.toString());

  // Acessa o gif_list na conta
  console.log("👀 GIF List", account.gifList);
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
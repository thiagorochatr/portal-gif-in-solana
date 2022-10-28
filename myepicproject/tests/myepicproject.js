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

	
  // Chama add_gif!
  await program.rpc.addGif({
    accounts: {
      baseAccount: baseAccount.publicKey,
    },
  });
  
  // Obtem a conta novamente e veja o que mudou.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count:', account.totalGifs.toString())
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
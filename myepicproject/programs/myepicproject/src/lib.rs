use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
        // Obtem a referência da conta
        let base_account = &mut ctx.accounts.base_account;
        // Inicializa o total_gifs.
        base_account.total_gifs = 0;
        Ok(())
    }
    
    pub fn add_gif(ctx: Context<AddGif>) -> Result <()> {
        // Obtem a referencia para a conta e incrementa total_gifs.
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs += 1;
        Ok(())
  }
}

// Anexa algumas variaveis ao contexto do StartStuffOff.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Especifica que dados queremos no Contexto AddGif
// Obtendo um controle sobre o fluxo das coisas 😊?
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// Fala para a Solana o que queremos guardar nessa conta.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}

use anchor_lang::prelude::*;

declare_id!("6GrvCvjDiSopTEsurpV3CF8tWR8TwuonCHM2H5pYvzEp");

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
    
  // A função agora aceita um parâmetro gif_link do usuário. Também referenciamos o usuário do Contexto
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	// Constroi o struct.
    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
    };

	// Adiciona ele ao array gif_list.
    base_account.gif_list.push(item);
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

// Adicione o signatário que chama o método AddGif ao struct para que possamos salvá-lo
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Crie uma estrutura personalizada para trabalharmos.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
	// Anexe um vetor do tipo ItemStruct à conta.
    pub gif_list: Vec<ItemStruct>,
}

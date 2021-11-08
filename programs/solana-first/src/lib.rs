use anchor_lang::prelude::*;

declare_id!("4uL5mteMGjtuAdNvUP8mQrh5Z4pXE72krJjF9S9M2n2F");

#[program]
pub mod solana_first {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    // Get a reference to the account
    let base_account = &mut ctx.accounts.base_account;

    // Build the struct.
    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *base_account.to_account_info().key,
      votes: 0,
    };

    // Add it to the gif_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;

    Ok(())
  }

  /*   pub fn upvote_gif(ctx: Context<UpvoteGif>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;

    let pos = base_account
      .gif_list
      .iter()
      .position(|gif| gif.gif_link == gif_link);

    if pos.is_some() {
      base_account.gif_list[pos.unwrap()].votes += 1;
    }

    Ok(())
  } */
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

// Specify what data you want in the AddGif Context.
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)] // get mutable access to account (can made changes to it)
  pub base_account: Account<'info, BaseAccount>,
}

// Specify what data you want in the UpvoteGif Context.
/* #[derive(Accounts)]
pub struct UpvoteGif<'info> {
  #[account(mut)] // get mutable access to account (can made changes to it)
  pub base_account: Account<'info, BaseAccount>,
} */

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
  pub gif_link: String,
  pub user_address: Pubkey,
  pub votes: u64,
}

// Tell Solana what we want to store on this account
#[account]
pub struct BaseAccount {
  pub total_gifs: u64,
  // Attach a Vector of type ItemStruct to the account.
  pub gif_list: Vec<ItemStruct>,
}

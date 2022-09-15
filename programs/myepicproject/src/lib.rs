// * FROM https://buildspace.so/p/build-solana-web3-app/lessons/finishing-touches-web-app-and-program
use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
declare_id!("3TyU5TFCdqNdvt6yAtDpaQsX8xsJtjqEePTn3EL2tdnh"); // DevNet key

#[program]
pub mod myepicproject {
  use anchor_lang::solana_program::{program::invoke, entrypoint::ProgramResult};

use super::*;

  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn clear_gifs(ctx: Context<BaseAccountOnly>) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    // let user = &mut ctx.accounts.user;
    base_account.total_gifs = 0;
    base_account.gif_list.clear();
    Ok(())
  }

  // The function now accepts a gif_link param from the user. We also reference the user from the Context
  pub fn add_gif(ctx: Context<BaseAccountAndUser>, gif_link: String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	  // Build the struct.
    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
      votes: 0,
    };
		
	  // Add it to the gif_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }

  pub fn upvote(ctx: Context<BaseAccountOnly>, index: u64) -> Result <()> {
  // pub fn upvote(ctx: Context<BaseAccountUserAndSystem>, index: u64) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    base_account.gif_list[index as usize].votes += 1;
    // if(base_account.gif_list[index as usize].votes == 5) {
    //   sendBonus(
    //     ctx,
    //     base_account.gif_list[index as usize].user_address,
    //     10_000_000
    //   );
    // }
    Ok(())
  }

  // pub fn sendBonus(ctx: Context<BaseAccountUserAndSystem>, to: Pubkey,amount: u64) -> Result <()> {
  //   // let instruction = anchor_lang::solana_program::system_instruction::transfer(
  //   //   // &ctx.accounts.from.key,
  //   //   &ctx.accounts.base_account.key(),
  //   //   // &ctx.accounts.to.key,
  //   //   &to,
  //   //   amount);
  //   // invoke(
  //   //   &instruction, 
  //   //   &[
  //   //     ctx.accounts.base_account.to_account_info().clone(),
  //   //     AccountInfo::from(to),
  //   //     ctx.accounts.system_program.to_account_info().clone(),
  //   // ]);
  //   ctx.accounts.system_program.???
  //   Ok(())
  // }

  pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> ProgramResult {
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.from.key(),
        &ctx.accounts.to.key(),
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.to.to_account_info(),
        ],
    )
  }

  pub fn downvote(ctx: Context<BaseAccountOnly>, index: u64) -> Result <()> {
    (&mut ctx.accounts.base_account)
      .gif_list[index as usize].votes -= 1;
    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct BaseAccountUserAndSystem<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct BaseAccountAndUser<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct BaseAccountOnly<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

// #[derive(Accounts)]
// pub struct Transfer<'info> {
//     //#[account(mut, signer)]
//     #[account(mut)]
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     pub from: AccountInfo<'info>,       
//     // /// CHECK: This is not dangerous because we don't read or write from this account
//     // #[account(mut)]
//     // pub to: AccountInfo<'info>,        
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     pub system_program: AccountInfo<'info>,
// }

#[derive(Accounts)]
pub struct SendSol<'info> {
    //#[account(mut, signer)]
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub from: AccountInfo<'info>,       
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,        
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes: u64,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
	  // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}

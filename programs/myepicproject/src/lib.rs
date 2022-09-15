// use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[program]
// pub mod myepicproject {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

// // * FROM https://buildspace.so/p/build-solana-web3-app/lessons/write-first-solana-program
// use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[program]
// pub mod myepicproject {
//   use super::*;
//   pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
//     Ok(())
//   }
// }

// #[derive(Accounts)]
// pub struct StartStuffOff {}

// // * FROM https://buildspace.so/p/build-solana-web3-app/lessons/store-basic-data-on-contract
// use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[program]
// pub mod myepicproject {
//   use super::*;
//   pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
//     // Get a reference to the account.
//     let base_account = &mut ctx.accounts.base_account;
//     // Initialize total_gifs.
//     base_account.total_gifs = 0;
//     Ok(())
//   }

//   pub fn add_gif(ctx: Context<AddGif>) -> Result <()> {
//     // Get a reference to the account and increment total_gifs.
//     let base_account = &mut ctx.accounts.base_account;
//     base_account.total_gifs += 1;
//     Ok(())
//   }
// }

// // Specify what data you want in the AddGif Context.
// #[derive(Accounts)]
// pub struct AddGif<'info> {
//   #[account(mut)]
//   pub base_account: Account<'info, BaseAccount>,
// }

// // Attach certain variables to the StartStuffOff context.
// #[derive(Accounts)]
// pub struct StartStuffOff<'info> {
//     #[account(init, payer = user, space = 9000)]
//     pub base_account: Account<'info, BaseAccount>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program <'info, System>,
// }

// // Tell Solana what we want to store on this account.
// #[account]
// pub struct BaseAccount {
//     pub total_gifs: u64,
// }

// // * FROM https://buildspace.so/p/build-solana-web3-app/lessons/store-structs-program
// use anchor_lang::prelude::*;

// // declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
// // * FROM https://buildspace.so/p/build-solana-web3-app/lessons/deploy-program-to-devnet
// declare_id!("3TyU5TFCdqNdvt6yAtDpaQsX8xsJtjqEePTn3EL2tdnh"); // DevNet key

// #[program]
// pub mod myepicproject {
//   use super::*;
//   pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
//     let base_account = &mut ctx.accounts.base_account;
//     base_account.total_gifs = 0;
//     Ok(())
//   }

//   // The function now accepts a gif_link param from the user. We also reference the user from the Context
//   pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
//     let base_account = &mut ctx.accounts.base_account;
//     let user = &mut ctx.accounts.user;

// 	// Build the struct.
//     let item = ItemStruct {
//       gif_link: gif_link.to_string(),
//       user_address: *user.to_account_info().key,
//     };
		
// 	// Add it to the gif_list vector.
//     base_account.gif_list.push(item);
//     base_account.total_gifs += 1;
//     Ok(())
//   }
// }

// #[derive(Accounts)]
// pub struct StartStuffOff<'info> {
//   #[account(init, payer = user, space = 9000)]
//   pub base_account: Account<'info, BaseAccount>,
//   #[account(mut)]
//   pub user: Signer<'info>,
//   pub system_program: Program <'info, System>,
// }

// // Add the signer who calls the AddGif method to the struct so that we can save it
// #[derive(Accounts)]
// pub struct AddGif<'info> {
//   #[account(mut)]
//   pub base_account: Account<'info, BaseAccount>,
//   #[account(mut)]
//   pub user: Signer<'info>,
// }

// // Create a custom struct for us to work with.
// #[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
// pub struct ItemStruct {
//     pub gif_link: String,
//     pub user_address: Pubkey,
// }

// #[account]
// pub struct BaseAccount {
//     pub total_gifs: u64,
// 	// Attach a Vector of type ItemStruct to the account.
//     pub gif_list: Vec<ItemStruct>,
// }

// * FROM https://buildspace.so/p/build-solana-web3-app/lessons/finishing-touches-web-app-and-program
use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
declare_id!("3TyU5TFCdqNdvt6yAtDpaQsX8xsJtjqEePTn3EL2tdnh"); // DevNet key

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    Ok(())
  }

  // The function now accepts a gif_link param from the user. We also reference the user from the Context
  pub fn add_gif(ctx: Context<BaseAccountAndUser>, gif_link: String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	  // Build the struct.
    let item = ItemStruct {
      votes: 0,
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
    };
		
	  // Add it to the gif_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }

  pub fn upvote(ctx: Context<BaseAccountOnly>, index: u64) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    base_account.gif_list[index as usize].votes += 1;
    Ok(())
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

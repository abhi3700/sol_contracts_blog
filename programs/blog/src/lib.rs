use anchor_lang::prelude::*;
use std::str::from_utf8;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod blog {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let blog_account = &mut ctx.accounts.blog_account;
        blog_account.authority = *ctx.accounts.blogger_auth.key;
        Ok(())
    }

    pub fn make_post(
        ctx: Context<MakePost>,
        new_post: Vec<u8>
    ) -> ProgramResult {
        let post = from_utf8(&new_post).map_err(|err| {
            msg!("Invalid UTF-8, from byte {}", err.valid_up_to());
            ProgramError::InvalidInstructionData
        })?;
        msg!(post);
        let blog_account = &mut ctx.accounts.blog_account;
        blog_account.latest_post = new_post;

        Ok(())
    }
}



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,       // Hey, Anchor please create new PDA with given requirements. Also, allocates the program_id to be the owner of the created PDA
        payer = blogger_auth,       // payer for the created PDA
        space = 8   // discriminator
        + 32        // 32 bytes for authority of post
        + 566       // 566 bytes long for blog post
    )]
    pub blog_account: Account<'info, BlogAccount>,
    pub blogger_auth: Signer<'info>,
    pub system_program: Program<'info, System>  // allocates the owner of the program as the system program BPFUpgradeable111....
}

#[derive(Accounts)]
pub struct MakePost<'info> {
    #[account(mut, has_one = authority)]
    pub blog_account: Account<'info, BlogAccount>,
    pub authority: Signer<'info>
}

#[account]
pub struct BlogAccount {
    pub latest_post: Vec<u8>,
    pub authority: Pubkey,
}

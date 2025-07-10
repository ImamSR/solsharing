use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

declare_id!("7Rc9yjhQ9RrGFLe5WbZwQ3kqqE7bzAYDsk4tJsXXKre8");

#[program]
pub mod solsharing {
    use super::*;

    pub fn upload_paper(ctx: Context<UploadPaper>, title: String, ipfs_hash: String) -> Result<()> {
        let title_hash = hash(title.as_bytes());
        let (expected_pda, _bump) = Pubkey::find_program_address(
            &[
                b"paper",
                ctx.accounts.author.key.as_ref(),
                &title_hash.to_bytes()
            ],
            ctx.program_id,
        );

        require_keys_eq!(
            ctx.accounts.paper.key(),
            expected_pda,
            CustomError::InvalidPda
        );

        let paper = &mut ctx.accounts.paper;
        paper.author = ctx.accounts.author.key();
        paper.title = title;
        paper.ipfs_hash = ipfs_hash;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UploadPaper<'info> {
    #[account(
        init,
        payer = author,
        space = Paper::LEN,
        seeds = [
            b"paper",
            author.key().as_ref(),
            &title_hash_seed.key.to_bytes()
        ],
        bump
    )]
    pub paper: Account<'info, Paper>,

    /// CHECK: This dummy account is used for passing the hash seed
    /// The PDA is verified manually in the handler
    pub title_hash_seed: AccountInfo<'info>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Paper {
    pub author: Pubkey,
    pub title: String,
    pub ipfs_hash: String,
}

impl Paper {
    // 8 discriminator + 32 author pubkey + 4+200 title + 4+50 ipfs
    pub const LEN: usize = 8 + 32 + (4 + 200) + (4 + 50);
}

#[error_code]
pub enum CustomError {
    #[msg("The provided PDA does not match the expected address.")]
    InvalidPda,
}

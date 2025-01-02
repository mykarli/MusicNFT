
use anchor_lang::prelude::*;

declare_id!("YOUR_PROGRAM_ID_HERE");

#[program]
pub mod music_nft {
    use super::*;

    pub fn create_nft(ctx: Context<CreateNFT>, title: String, artist: String, metadata: String) -> Result<()> {
        let nft = &mut ctx.accounts.nft;
        nft.title = title;
        nft.artist = artist;
        nft.metadata = metadata;
        nft.owner = *ctx.accounts.user.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateNFT<'info> {
    #[account(init, payer = user, space = 8 + 128)]
    pub nft: Account<'info, NFT>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NFT {
    pub title: String,
    pub artist: String,
    pub metadata: String,
    pub owner: Pubkey,
}

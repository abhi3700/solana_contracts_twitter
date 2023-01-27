use anchor_lang::prelude::*;

declare_id!("8DXC6GMDneGhysZHX8uVZDXPuJBUGXHcJcymYov1ePhN");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

// define Tweet account
#[account]
pub struct Tweet {
    // storing author such that it can be used to verify the tweet's owner is the caller of the program
    // assertion that `solana_twitter_program.caller() == author`
    pub author: Pubkey,  // pubkey of the author
    pub timestamp: i64,  // post timestamp
    pub topic: String,   // hashtag
    pub content: String, // post
}

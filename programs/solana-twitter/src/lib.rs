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

// 1. Define the structure of the Tweet account.
#[account]
pub struct Tweet {
    // storing author such that it can be used to verify the tweet's owner is the caller of the program
    // assertion that `solana_twitter_program.caller() == author`
    pub author: Pubkey,  // pubkey of the author
    pub timestamp: i64,  // post timestamp
    pub topic: String,   // hashtag
    pub content: String, // post
}

// 2. Add some useful constants for sizing propeties.
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.

// 3. Add a constant on the Tweet account that provides its total size.
impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // Content.
}

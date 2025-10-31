use anchor_lang::prelude::*;
use anchor_spl::token::Token;

declare_id!("9eCWaZoZuNafUYqptxGWW8o87BieZkfLfm6C3PTsFuLZ");

#[program]
pub mod prediction_market {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, market_id: u32) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub token_program: Program<'info, Token>,

    pub market_pda: Account<'info, Market>,
}

#[account]
pub struct Market{
    pub authority: Pubkey,
    pub market_id: u32,
    pub bump: u8,
    pub is_settled: bool,
    #[max_len(50)]
    pub metadata_uri: String,
}
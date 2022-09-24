use anchor_lang::prelude::*;

declare_id!("6UYEfgjgxdeMAsznFYEwtgR25UjDtd6wPvXpBQLZtv8N");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

use anchor_lang::prelude::*;

mod state;
mod instructions;
mod error;
declare_id!("GydRG8fB6xgtKCNW1AzrTqUTmmb7CaHWV8ZaCM9fh89H");

#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

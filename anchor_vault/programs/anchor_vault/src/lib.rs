use anchor_lang::prelude::*;

declare_id!("vF7jhRztdWk47aQkYwHJa3Gfo4FwLyUXngYhy2PdAEs");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

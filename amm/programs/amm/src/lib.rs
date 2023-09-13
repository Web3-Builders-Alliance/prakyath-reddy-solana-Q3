use anchor_lang::prelude::*;
use state;

declare_id!("3XaekuKBM4f4Tdaa4xMEpwZDf6aHinZCFDdv1MGKAPYE");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

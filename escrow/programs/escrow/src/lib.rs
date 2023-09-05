use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount, Transfer},
};

declare_id!("E5K3rtGD7nEvSqHTxeNfEChvcrd3xiKUkKNY9zQWam1K");

mod constants;
use constants::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        deposit_amount: u64,
        offer_amount: u64,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.maker = *ctx.accounts.maker.key;
        escrow.maker_token = ctx.accounts.maker_token.key();
        escrow.taker_token = ctx.accounts.taker_token.key();
        escrow.offer_amount = offer_amount;
        escrow.seed = seed;
        escrow.auth_bump = *ctx.bumps.get("auth").unwrap();
        escrow.vault_bump = *ctx.bumps.get("vault").unwrap();
        escrow.escrow_bump = *ctx.bumps.get("escrow").unwrap();

        let cpi_accounts = Transfer {
            from: ctx.accounts.maker_ata.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        token::transfer(cpi_ctx, deposit_amount)?;

        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        uimplemented!()
    }
}

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = maker_token,
        associated_token::authority = maker,
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    pub maker_token: Box<Account<'info, Mint>>,
    pub taker_token: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"auth", escrow.key().as_ref()],
        bump,
    )]
    /// CHECK: This is safe, because it doesn't have exist
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = maker,
        seeds = [b"vault", escrow.key().as_ref()],
        bump,
        token::mint = taker_token,
        token::authority = auth,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = Escrow::LEN,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Take<'info> {
    /// CHECK: no need to check this.
    pub maker: UncheckedAccount<'info>,
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_token,
        associated_token::authority = maker,
    )]
    pub maker_recieve_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = maker_token,
        associated_token::authority = maker,
    )]
    pub taker_ata: Account<'info, TokenAccount>,>
    pub maker_token: Box<Account<'info, Mint>>,
    pub taker_token: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"auth", escrow.key().as_ref()],
        bump = escrow.auth_bump,
    )]
    /// CHECK: This is safe, because it doesn't have exist
    pub auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", escrow.key().as_ref()],
        bump = escrow.vault_bump,
        token::mint = taker_token,
        token::authority = auth,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        has_one = maker,
        has_one = taker_token,
        has_one = maker_token,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn deposit_to_maker(&self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.taker_ata.to_account_info(),
            to: self.maker_recieve_ata.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        token::transfer(cpi_ctx, self.escrow.offer_amount)
    }

    pub fn empty_vault_to_taker(&self) -> Result<()> {
        let signer_seeds = &[
            b"auth"[..],
            &[self.escrow.auth_bump],
        ];
        let binding = [&signer_seeds[..]];
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.taker_recieve_ata.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, &binding);

        token::transfer(cpi_ctx, self.vault.amount)
    }
}

#[account]
pub struct Escrow {
    pub maker: Pubkey,
    pub maker_token: Pubkey,
    pub taker_token: Pubkey,
    pub offer_amount: u64,
    pub seed: u64,
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub escrow_bump: u8,
}

impl Escrow {
    const LEN: usize = (PUBKEY_L * 3) + (U64_L * 2) + (U8_L * 3);
}

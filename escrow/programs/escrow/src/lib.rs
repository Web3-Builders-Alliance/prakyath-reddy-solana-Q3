use anchor_lang::prelude::*;
use anchor_spl::token::close_account as spl_close_account;
use anchor_spl::token::transfer as spl_transfer;
use anchor_spl::token::CloseAccount as SplCloseAccount;
use anchor_spl::token::Transfer as SplTransfer;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("E5K3rtGD7nEvSqHTxeNfEChvcrd3xiKUkKNY9zQWam1K");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        offer_amount: u64,
        deposit_amount: u64,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.maker = ctx.accounts.maker.key();
        escrow.maker_token = ctx.accounts.maker_token.key();
        escrow.taker_token = ctx.accounts.taker_token.key();
        escrow.seed = seed;
        escrow.offer_amount = offer_amount;
        escrow.auth_bump = *ctx.bumps.get("auth").unwrap();
        escrow.vault_bump = *ctx.bumps.get("vault").unwrap();
        escrow.escrow_bump = *ctx.bumps.get("escrow").unwrap();

        let ctx_accounts = SplTransfer {
            from: ctx.accounts.maker_ata.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };
        let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), ctx_accounts);
        spl_transfer(cpi, deposit_amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit_to_maker()?;
        ctx.accounts.empty_vault_to_taker()?;
        ctx.accounts.close_vault()
    }

    pub fn update(ctx: Context<Update>, offer_amount: u64) -> Result<()> {
        ctx.accounts.update(offer_amount)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.empty_vault()?;
        ctx.accounts.close_vault()
    }
}

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = maker_token,
        associated_token::authority = maker
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    pub maker_token: Box<Account<'info, Mint>>,
    pub taker_token: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"auth", escrow.key().as_ref()],
        bump
    )]
    /// CHECK: this is safe
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer= maker,
        seeds=[b"vault", escrow.key().as_ref()],
        token::mint= maker_token,
        token::authority= auth,
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        init,
        payer= maker,
        space= Escrow::LEN,
        seeds= [b"escrow", maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Take<'info> {
    /// CHECK: this is safe
    pub maker: UncheckedAccount<'info>,
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(
        init_if_needed,
        payer= taker,
        associated_token::mint = taker_token,
        associated_token::authority = taker
    )]
    pub taker_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer= taker,
        associated_token::mint = taker_token,
        associated_token::authority = maker
    )]
    pub maker_recieve_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer= taker,
        associated_token::mint = maker_token,
        associated_token::authority = taker
    )]
    pub taker_recieve_ata: Account<'info, TokenAccount>,
    pub maker_token: Box<Account<'info, Mint>>,
    pub taker_token: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"auth", escrow.key().as_ref()],
        bump= escrow.auth_bump,
    )]
    /// CHECK: this is safe
    pub auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", escrow.key().as_ref()],
        token::mint= maker_token,
        token::authority= auth,
        bump= escrow.vault_bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        has_one = maker,
        has_one = taker_token,
        has_one = maker_token,
        seeds= [b"escrow", maker.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump=escrow.escrow_bump,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub new_taker_token: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds= [b"escrow", maker.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump=escrow.escrow_bump,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = maker_token,
        associated_token::authority = maker
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    pub maker_token: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"auth", escrow.key().as_ref()],
        bump = escrow.auth_bump
    )]
    /// CHECK: this is safe
    pub auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds=[b"vault", escrow.key().as_ref()],
        token::mint= maker_token,
        token::authority= auth,
        bump = escrow.vault_bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        seeds= [b"escrow", maker.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.escrow_bump,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    pub fn empty_vault(&self) -> Result<()> {
        let ctx_accounts = SplTransfer {
            from: self.vault.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.auth.to_account_info(),
        };
        let seeds: &[&[u8]; 2] = &[b"auth", &[self.escrow.auth_bump]];
        let pda_signer: &[&[&[u8]]; 1] = &[&seeds[..]];
        let cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            ctx_accounts,
            pda_signer,
        );
        spl_transfer(cpi, self.vault.amount)
    }

    pub fn close_vault(&self) -> Result<()> {
        let ctx_accounts = SplCloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.auth.to_account_info(),
        };
        let seeds: &[&[u8]; 2] = &[b"auth", &[self.escrow.auth_bump]];
        let pda_signer: &[&[&[u8]]; 1] = &[&seeds[..]];
        let cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            ctx_accounts,
            pda_signer,
        );
        spl_close_account(cpi)
    }
}

impl<'info> Update<'info> {
    pub fn update(&mut self, offer_amount: u64) -> Result<()> {
        self.escrow.taker_token = self.new_taker_token.key();
        self.escrow.offer_amount = offer_amount;
        Ok(())
    }
}

impl<'info> Take<'info> {
    pub fn deposit_to_maker(&self) -> Result<()> {
        let ctx_accounts = SplTransfer {
            from: self.taker_ata.to_account_info(),
            to: self.maker_recieve_ata.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let cpi = CpiContext::new(self.token_program.to_account_info(), ctx_accounts);
        spl_transfer(cpi, self.escrow.offer_amount)
    }

    pub fn empty_vault_to_taker(&self) -> Result<()> {
        let ctx_accounts = SplTransfer {
            from: self.vault.to_account_info(),
            to: self.taker_recieve_ata.to_account_info(),
            authority: self.auth.to_account_info(),
        };
        let seeds: &[&[u8]; 2] = &[b"auth", &[self.escrow.auth_bump]];
        let pda_signer: &[&[&[u8]]; 1] = &[&seeds[..]];
        let cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            ctx_accounts,
            pda_signer,
        );
        spl_transfer(cpi, self.vault.amount)
    }

    pub fn close_vault(&self) -> Result<()> {
        let ctx_accounts = SplCloseAccount {
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.auth.to_account_info(),
        };
        let seeds: &[&[u8]; 2] = &[b"auth", &[self.escrow.auth_bump]];
        let pda_signer: &[&[&[u8]]; 1] = &[&seeds[..]];
        let cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            ctx_accounts,
            pda_signer,
        );
        spl_close_account(cpi)
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
    const LEN: usize = 32 * 3 + 8 * 2 + 1 * 3;
}

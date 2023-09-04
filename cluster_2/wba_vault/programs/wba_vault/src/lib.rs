use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::{
    close_account as spl_close_account, transfer as spl_transfer, CloseAccount as SplCloseAccount,
    Mint, Token, TokenAccount, Transfer as SplTransfer,
};
// Transfer is for the function, transfer is the actual execution of the function

declare_id!("wbauEhzu1CGBTsbzW2VpFfKrsStuNDi7YMw3Uj5WBvf");

#[program]
pub mod wba_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.auth_bump = *ctx.bumps.get("auth").unwrap();
        ctx.accounts.state.vault_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.state.state_bump = *ctx.bumps.get("state").unwrap();
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let cpi = CpiContext::new(ctx.accounts.system_program.to_account_info(), accounts);

        transfer(cpi, amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let accounts = Transfer {
            // this is system program transfer
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
        };

        let seeds = &[
            b"vault",
            ctx.accounts.state.to_account_info().key.as_ref(),
            &[ctx.accounts.state.vault_bump],
        ];

        let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer(cpi, amount)
    }

    pub fn deposit_spl(ctx: Context<DepositSPL>, amount: u64) -> Result<()> {
        let accounts: Transfer<'_> = SPLTransfer {
            // this is spl token transfer
            from: ctx.accounts.owner_ata.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };

        let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts);

        spl_transfer(cpi, amount)
    }

    pub fn withdraw_spl(ctx: Context<WithdrawSPL>, amount: u64) -> Result<()> {
        let accounts: Transfer<'_> = SPLTransfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner_ata.to_account_info(),
            authority: ctx.accounts.auth.to_account_info(),
        };
        let seeds = &[
            b"auth",
            ctx.accounts.state.to_account_info().key.as_ref(),
            &[ctx.accounts.state.auth_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        spl_transfer(cpi, amount)
    }

    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        match ctx.accounts.vault.try_lamports() {
            Ok(amount) => {
                let accounts = Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.owner.to_account_info(),
                };

                let seeds = &[
                    b"vault",
                    ctx.accounts.state.to_account_info().key.as_ref(),
                    &[ctx.accounts.state.vault_bump],
                ];

                let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];

                let cpi = CpiContext::new_with_signer(
                    ctx.accounts.system_program.to_account_info(),
                    accounts,
                    signer_seeds,
                );

                transfer(cpi, amount)?;
            }
            Err(_) => (),
        }

        let seeds = &[
            b"auth",
            ctx.accounts.state.to_account_info().key.as_ref(),
            &[ctx.accounts.state.auth_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let close_account_spl = CloseAccount {
            account: ctx.accounts.spl_vaults.to_account_info(),
            destination: ctx.accounts.owner.to_account_info(),
            authority: ctx.accounts.auth.to_account_info(),
        };

        if ctx.accounts.spl_vault.amount > 0 {
            let accounts = SPLTransfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.owner_ata.to_account_info(),
                authority: ctx.accounts.auth.to_account_info(),
            };

            let cpi = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                accounts,
                signer_seeds,
            );

            spl_transfer(cpi, ctx.accounts.spl_vault.amount.clone())?;
        }

        let cpi = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            close_account_spl,
            signer_seeds,
        );

        spl_close_account(cpi)
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    // here we are going to send some lamports to a PDA
    // since we have to send lamports, we need to use a system program
    // also we want to keep tract of that bump seed of that PDA
    // so we are going to create an unchecked account now
    /// CHECK: it's safe
    #[account(mut)]
    owner: AccountInfo<'info>,
    /// CHECK: it's safe
    #[account(
        seeds=[b"auth", state.key().as_ref()], // replaced signer with state so we can inherit the public key of the state
        bump,
    )]
    /// CHECK: it's safe
    auth: UncheckedAccount<'info>,
    // when we create an SPL token, when we are moving it around, we are going
    // to need an auth account, someone who can sign for it. It can be a public-private
    // keypair, or it can be a PDA. We are going to use a PDA here.
    #[account(
        seeds=[b"vault", state.key().as_ref()],
        bump,
    )]
    vault: SystemAccount<'info>,
    // vault is where we are going to store the sol
    system_program: Program<'info, System>,
    #[account(
        init,
        payer = owner,
        space = VaultState::LEN, // 8 bytes for each u64
        seeds=[b"state", owner.key.as_ref()],
        bump,
    )]
    state: Account<'info, VaultState>,
    // what it's saying is, make sure this account is owned by the program
    // and make VaultState is what is contains
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        seeds=[b"vault", state.key().as_ref()],
        bump = state.vault_bump,
    )]
    vault: SystemAccount<'info>,
    #[account(
        seeds=[b"state", owner.key.as_ref()],
        bump = state.state_bump,
    )]
    state: Account<'info, VaultState>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSPL<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    owner_ata: AccountInfo<'info, TokenAccount>,
    mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        seeds = [b"spl_vault", state.key().as_ref()],
        token::mint = mint,
        token::authority = auth,
        bump
    )]
    vault: Account<'info, TokenAccount>,
    #[account(
        seeds=[b"auth", state.key().as_ref()], // replaced signer with state so we can inherit the public key of the state
        bump = state.auth_bump,
    )]
    /// CHECK: it's safe
    auth: UncheckedAccount<'info>,
    #[account(
        seeds=[b"state", owner.key.as_ref()],
        bump = state.state_bump,
    )]
    state: Account<'info, VaultState>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SPlWithdraw<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    owner_ata: AccountInfo<'info, TokenAccount>,
    mint: AccountInfo<'info, Mint>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = auth,
        seeds = [b"spl_vault", state.key().as_ref()],
        bump
    )]
    vault: AccountInfo<'info, TokenAccount>,
    #[account(
        seeds=[b"auth", state.key().as_ref()], // replaced signer with state so we can inherit the public key of the state
        bump = state.auth_bump,
    )]
    /// CHECK: it's safe
    auth: UncheckedAccount<'info>,
    #[account(
        seeds=[b"state", owner.key.as_ref()],
        bump = state.state_bump,
    )]
    state: Account<'info, VaultState>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    owner_ata: AccountInfo<'info, TokenAccount>,
    mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"spl_vault", state.key().as_ref()],
        token::mint = mint,
        token::authority = auth,
        bump
    )]
    spl_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"vault", state.key().as_ref()],
        bump = state.vault_bump,
    )]
    vault: SystemAccount<'info>,
    #[account(
        seeds=[b"auth", state.key().as_ref()], // replaced signer with state so we can inherit the public key of the state
        bump = state.auth_bump,
    )]
    /// CHECK: it's safe
    auth: UncheckedAccount<'info>,
    #[account(
        mut,
        close = owner,
        seeds=[b"state", owner.key.as_ref()],
        bump = state.state_bump,
    )]
    state: Account<'info, VaultState>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

// we need a data account to store the bumps
#[account]
pub struct VaultState {
    auth_bump: u8,
    vault_bump: u8,
    state_bump: u8,
}

impl VaultState {
    const LEN: usize = 8 + 3 * 1;
}

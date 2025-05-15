use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

declare_id!("tokenExchangeProgram11111111111111111111111111");

#[program]
pub mod token_exchange {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        fee_numerator: u64,
        fee_denominator: u64,
    ) -> Result<()> {
        let exchange_state = &mut ctx.accounts.exchange_state;
        exchange_state.authority = ctx.accounts.authority.key();
        exchange_state.fee_numerator = fee_numerator;
        exchange_state.fee_denominator = fee_denominator;
        exchange_state.total_fee_collected = 0;
        Ok(())
    }

    pub fn swap(
        ctx: Context<Swap>,
        amount_in: u64,
    ) -> Result<()> {
        // Calculate anti-snipe fee (10% to 2% based on liquidity)
        let liquidity = ctx.accounts.pool_token_account.amount;
        let fee_rate = if liquidity < 1000000 {
            10 // 10% fee for low liquidity
        } else if liquidity < 10000000 {
            5 // 5% fee for medium liquidity
        } else {
            2 // 2% fee for high liquidity
        };

        let fee_amount = amount_in.checked_mul(fee_rate).unwrap() / 100;
        let amount_in_after_fee = amount_in.checked_sub(fee_amount).unwrap();

        // Transfer tokens from user to pool
        token::transfer(
            ctx.accounts.into_transfer_to_pool_context(),
            amount_in,
        )?;

        // Calculate output amount based on bonding curve
        let amount_out = calculate_output_amount(amount_in_after_fee, liquidity);

        // Transfer tokens from pool to user
        token::transfer(
            ctx.accounts.into_transfer_from_pool_context(),
            amount_out,
        )?;

        // Burn fee tokens
        token::burn(
            ctx.accounts.into_burn_context(),
            fee_amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8 + 8)]
    pub exchange_state: Account<'info, ExchangeState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub exchange_state: Account<'info, ExchangeState>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub user: Signer<'info>,
}

#[account]
pub struct ExchangeState {
    pub authority: Pubkey,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
    pub total_fee_collected: u64,
}

impl<'info> Swap<'info> {
    fn into_transfer_to_pool_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_account.to_account_info(),
                to: self.pool_token_account.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    fn into_transfer_from_pool_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.pool_token_account.to_account_info(),
                to: self.user_token_account.to_account_info(),
                authority: self.exchange_state.to_account_info(),
            },
        )
    }

    fn into_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.pool_token_account.mint.to_account_info(),
                from: self.user_token_account.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }
}

// Helper function to calculate output amount based on bonding curve
fn calculate_output_amount(amount_in: u64, liquidity: u64) -> u64 {
    // Simple linear bonding curve for demonstration
    // In production, use more sophisticated curves
    let base_rate = 98; // 98% base exchange rate
    amount_in.checked_mul(base_rate).unwrap() / 100
}
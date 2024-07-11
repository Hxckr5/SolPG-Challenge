use anchor_lang::prelude::*;

declare_id!("8MaTQ4rf5QpqnDRcaYwJhW4LKRniaJUoEBFaaUkqin2R");

#[program]
pub mod moneybox {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let moneybox = &mut ctx.accounts.moneybox;
        moneybox.owner = *ctx.accounts.user.key;
        moneybox.balance = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let moneybox = &mut ctx.accounts.moneybox;
        let user = &ctx.accounts.user;

        **user.try_borrow_mut_lamports()? -= amount;
        **moneybox.to_account_info().try_borrow_mut_lamports()? += amount;

        moneybox.balance += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let moneybox = &mut ctx.accounts.moneybox;
        let user = &ctx.accounts.user;

        if moneybox.balance < amount {
            return Err(MyError::InsufficientFunds.into());
        }

        **moneybox.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.try_borrow_mut_lamports()? += amount;

        moneybox.balance -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub moneybox: Account<'info, Moneybox>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub moneybox: Account<'info, Moneybox>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub moneybox: Account<'info, Moneybox>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Moneybox {
    pub owner: Pubkey,
    pub balance: u64,
}

#[error_code]
pub enum MyError {
    #[msg("Insufficient funds in the moneybox.")]
    InsufficientFunds,
}


//Made by Hxckr5 for SuperteamUK
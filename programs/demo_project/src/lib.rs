use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;
declare_id!("9CdHEqVd7ZM5Hk34MPCPNG6NPA5QiaxFV9awc57DfSQH");

#[program]
pub mod demo_project {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut _ctx.accounts.base_account;
        base_account.count = 0;
        
        Ok(())
    }
    pub fn increment(_ctx: Context<Increment>) -> ProgramResult {
        let base_account = &mut _ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>
}



#[account]
pub struct BaseAccount {
    pub count: u64,
}

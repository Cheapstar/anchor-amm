use anchor_lang::prelude::*;



#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user:Signer<'info>,

    

}




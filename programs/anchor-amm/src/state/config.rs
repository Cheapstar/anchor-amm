use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Config {
    pub seeds:u64,
    pub authority:Option<Pubkey>,
    pub fee:u16,
    // Ye token isiliye nahi hai kyuki we are using associated token accounts,
    //  so saath ke saath we can find it
    pub mint_x:Pubkey,
    pub mint_y:Pubkey,

    // Q. As per me mint_lp should also come in this , but we will see ?
    // Ans. we don't need it as is derivable and easily verifiable so extra space not needed
    // pub mint_lp:Pubkey,
    pub locked:bool,

    // Storing Bumps is a good practice
    pub config_bump:u8,
    pub lp_bump:u8

}
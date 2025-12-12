use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{TokenAccount,TokenInterface,Mint}};

use crate::state::Config;


#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Initialize<'info> {
    // Since ye act karega as a initializer for many things
    #[account(mut)]
    pub initializer:Signer<'info>,
    pub mint_x:InterfaceAccount<'info,Mint>,                 // tokens in liquidity pool
    pub mint_y:InterfaceAccount<'info,Mint>,

    #[account(
        init,
        payer = initializer,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config",seeds.to_le_bytes().as_ref()],
        bump
    )]
    pub config:Account<'info,Config>,


    /// Ye wo accounts hai jo hold karenge tokens
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x : InterfaceAccount<'info,TokenAccount>,
        #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y : InterfaceAccount<'info,TokenAccount>,

    #[account(
        init,
        seeds = [b"lp",config.key().as_ref()],
        payer = initializer,
        mint::authority = config,
        mint::decimals = 6,
        bump
    )]
    pub mint_lp:InterfaceAccount<'info,Mint>,            // Jo Token will be given to the liquidity provider
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>
}



impl<'info> Initialize<'info> {
    fn init(&mut self,fee:u16,seeds:u64,bumps:InitializeBumps){
        // Initialize Bumps Basically Stores Bump of PDAs in the context
        
        self.config.set_inner(
            Config { 
                seeds, 
                authority: Some(self.initializer.key()), 
                fee, 
                mint_x: self.mint_x.key(),
                mint_y: self.mint_y.key(), 
                locked: false, 
                config_bump: bumps.config, 
                lp_bump: bumps.mint_lp 
            });
    }
}
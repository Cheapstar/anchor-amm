use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::{associated_token::AssociatedToken, token_2022::MintToChecked, token_interface::{Mint, MintTo, TokenAccount, TokenInterface, TransferChecked, mint_to_checked, transfer_checked}};

use crate::{error::AmmError, state::Config};



#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user:Signer<'info>,

    pub mint_x:InterfaceAccount<'info,Mint>,
    pub mint_y:InterfaceAccount<'info,Mint>,

    #[account(
        mut,  // cause we are changing the total supply of the account , Nice!
        mint::authority = config,
        mint::decimals = 6,
        seeds = [b"lp",config.key().as_ref()],
        bump
    )]
    pub mint_lp:InterfaceAccount<'info,Mint>,
    // ye i think is needed for verification
    #[account(
        mut,
        seeds=[b"config",config.seeds.to_le_bytes().as_ref()],
        bump,
        has_one=mint_x,
        has_one=mint_y,
        constraint = config.locked == false @ AmmError::MarketLocked
    )]
    pub config:Account<'info,Config>,                

    // program's associated accounts 
    #[account(
        mut,
        associated_token::mint = config.mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: InterfaceAccount<'info,TokenAccount>, 
    #[account(
        mut,
        associated_token::mint = config.mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: InterfaceAccount<'info,TokenAccount>, 

    // user associated account
    #[account(
        mut,
        associated_token::mint = config.mint_x,
        associated_token::authority = user,
    )]
    pub deposit_x:InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::mint = config.mint_y,
        associated_token::authority = user,
    )]
    pub deposit_y:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init_if_needed,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
        payer = user
    )]
    pub user_lp:InterfaceAccount<'info,TokenAccount>,

    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>
}


impl<'info> Deposit<'info> {
    pub fn deposit_tokens(&mut self,is_x:bool,amount:u64)->ProgramResult{
        let (from,to,mint) = match is_x {
            true => (self.deposit_x.to_account_info(),self.vault_x.to_account_info(),&self.mint_x),
            false => (self.deposit_y.to_account_info(),self.vault_y.to_account_info(),&self.mint_y),
        };

         

        let cpi = CpiContext::new(
            self.token_program.to_account_info(), 
            TransferChecked {
                from,
                mint:mint.to_account_info(),
                to,
                authority:self.user.to_account_info()
            },
        );

        transfer_checked(cpi,amount,mint.decimals);
        Ok(())
    }

    pub fn mint_lp_tokens(&mut self,amount:u64)->ProgramResult {
        // seeds m bump hona hi chahiye
        let seeds = &[
                        &b"config"[..],
                        &self.config.seeds.to_le_bytes(),
                        &[self.config.config_bump]
                    ];
        let signer_seeds = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            MintToChecked {
                mint:self.mint_lp.to_account_info(),
                to:self.user_lp.to_account_info(),
                authority:self.config.to_account_info()
            }, signer_seeds);


        mint_to_checked(cpi, amount, self.mint_lp.decimals);
        Ok(())
    }
}


use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::*,
        token::*,
    },

    crate::collections::Event,
};


#[derive(Accounts)]
#[instruction(cantidad: u64)]
pub struct WithdrawFunds<'info> {

    #[account(mut)]
    pub organizador: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [
            Event::SEED_EVENTO.as_ref(), 
            organizador.key().as_ref(), 
        ],
        bump = evento.bump_evento, 
    )]
    pub evento: Box<Account<'info, Event>>, 

    pub moneda_pago: Box<Account<'info, Mint>>, 

    #[account(
        init_if_needed, 
        payer = organizador,
        associated_token::mint = moneda_pago, 
        associated_token::authority = organizador, 
    )]
    pub ata_moneda_pago: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            Event::SEED_TESORERIA.as_ref(), 
            evento.key().as_ref(), 
        ],
        bump = evento.bump_tesoreria,
        constraint = tesoreria.amount >= cantidad 
    )]
    pub tesoreria: Box<Account<'info, TokenAccount>>, 
}

pub fn handle(ctx: Context<WithdrawFunds>, cantidad: u64) -> Result<()> {
    
    let seeds = [    
        Event::SEED_EVENTO.as_bytes(), 
        ctx.accounts.evento.organizador.as_ref(), 
        &[ctx.accounts.evento.bump_evento], 
    ];
    
    let signer = &[&seeds[..]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.tesoreria.to_account_info(), 
                to: ctx.accounts.ata_moneda_pago.to_account_info(), 
                authority: ctx.accounts.evento.to_account_info(), 
            },
            signer, 
        ),
        cantidad,
    )?;

    Ok(())
}

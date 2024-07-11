use {
    crate::collections::Event,
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::*,
        token::*,
    },
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct WithdrawFunds<'info> {
    #[account(
        mut,
        seeds = [
            Event::SEED_EVENTO.as_ref(), 
            evento.organizador.as_ref(), 
        ],
        bump = evento.bump_evento, 
    )]
    pub evento: Box<Account<'info, Event>>, 

    pub moneda_aceptada: Box<Account<'info, Mint>>, 

    #[account(
        init_if_needed, 
        payer = organizador,
        associated_token::mint = moneda_aceptada, 
        associated_token::authority = organizador, 
    )]
    
    pub ata_organizador_moneda_aceptada: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            Event::SEED_TESORERIA.as_ref(), 
            evento.key().as_ref(), 
        ],
        bump = evento.bump_tesoreria, /
        constraint = tesoreria.amount >= amount 
    )]
    pub tesoreria: Box<Account<'info, TokenAccount>>, 

    #[account(mut)]
    pub organizador: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handle(
    ctx: Context<RetirarFondos>,
    amount: u64,
) -> Result<()> {
    let seeds = [
        
        Event::SEED_EVENTO.as_bytes(), 
        ctx.accounts.evento.organizador.as_ref(), 
        &[ctx.accounts.evento.bump_evento], 
    ];
    let signer = &[&seeds[..]];

    transferir(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transferir {
                from: ctx.accounts.tesoreria.to_account_info(), 
                to: ctx.accounts.ata_organizador_moneda_aceptada.to_account_info(), 
                authority: ctx.accounts.evento.to_account_info(), 
            },
            signer, 
        ),
        amount,
    )?;
    Ok(())
}

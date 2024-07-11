use {
    anchor_lang::prelude::*, 
    anchor_spl::{
        token::*,
        associated_token::*,
    },

    crate::collections::Event,
};

#[derive(Accounts)]
pub struct Sponsor<'info> {
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
            evento.organizador.key().as_ref(),
        ],
        bump = evento.bump_evento,
    )]
    pub evento: Box<Account<'info, Event>>,

    #[account(
        mut,
        seeds = [
            Event::SEED_FLOW_TOKEN.as_ref(),
            evento.key().as_ref()
        ],
        bump = evento.bump_flow_token,
    )]
    pub flow_token: Box<Account<'info, Mint>>,

    // ATA (Associated Token Account) del sponsor con la moneda de pago
    // Cuenta de la wallet del sponsor que contiene la moneda de pago que va a pagar
    #[account(
        mut,
        constraint = ata_moneda_pago.mint == evento.moneda_pago,
        constraint = ata_moneda_pago.amount > 0
    )]
    pub ata_moneda_pago: Box<Account<'info, TokenAccount>>,

    // ATA para el sponsor donde se hara el mint del token del evento
    #[account(
        init_if_needed,
        payer = organizador, 
        associated_token::mint = flow_token, // event mint
        associated_token::authority = organizador,
    )]
    pub ata_flow_token: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            Event::SEED_TESORERIA.as_ref(),
            evento.key().as_ref()
        ],
        bump = evento.bump_tesoreria,
    )]
    pub tesoreria: Box<Account<'info, TokenAccount>>,
}

pub fn handler(
    ctx: Context<Sponsor>,
    cantidad: u64,
  ) -> Result<()> {

    let seeds = [
        Event::SEED_EVENTO.as_bytes(),
        ctx.accounts.evento.organizador.as_ref(),
        &[ctx.accounts.evento.bump_evento],
    ];
    
    let signer = &[&seeds[..]];
    
    // Cobrar
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.ata_moneda_pago.to_account_info(),
                to: ctx.accounts.tesoreria.to_account_info(),
                authority: ctx.accounts.organizador.to_account_info(),
            },
        ),
        cantidad,
    )?;
    
    // Enviar token
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.flow_token.to_account_info(),
                to: ctx.accounts.ata_flow_token.to_account_info(),
                authority: ctx.accounts.evento.to_account_info(),
            },
            signer,
        ),
        cantidad,
    )?;
    
    ctx.accounts.evento.sponsors += cantidad;

    Ok(())
  }

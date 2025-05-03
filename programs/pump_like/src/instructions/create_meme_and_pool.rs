use crate::constants::meme::{DECIMALS, POOL_SEED, POOL_CONFIG_SEED, MINT_SEED, TOTAL_SUPPLY};
use crate::state::*;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_V3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata as Metaplex,
    },
    token::{
        mint_to, set_authority, spl_token::instruction::AuthorityType,  Mint, MintTo, SetAuthority,
        Token, TokenAccount},
};

use std::mem::size_of;

#[derive(Accounts)]
#[instruction(
    params: InitTokenParams
)]
pub struct CreateMemeAndPool<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = PoolState::LEN,
        seeds = [POOL_SEED, mint.key().as_ref()],
        bump,
    )]
    pub pool_state: AccountLoader<'info, PoolState >,

    #[account(
        init,
        payer = payer,
        seeds = [MINT_SEED, mint.key().as_ref()],
        bump,
        mint::decimals = DECIMALS,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        seeds = [POOL_CONFIG_SEED, mint.key().as_ref()],
        bump,
        space = size_of::<MintConfig>()+8,
    )]
    pub mint_config: Account<'info, MintConfig>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = pool_state,
    )]
    pub meme_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_config: AccountLoader<'info, PoolConfig>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metaplex>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenParams {
    pub name: String,
    pub symbol: String,
    pub url: String,
    pub mint_seed: String,
}

pub fn create_meme_and_pool(
    ctx: Context<CreateMemeAndPool>,
    init_token_params: InitTokenParams,
) -> Result<()> {
    ctx.accounts.mint_config.seed = init_token_params.mint_seed.clone();

    // ------ Create Token Mint ------ //
    let seeds = &[
        MINT_SEED,
        init_token_params.mint_seed.as_bytes(),
        &[ctx.bumps.mint],
    ];
    let signer = &[&seeds[..]];
    let token_data: DataV2 = DataV2 {
        name: init_token_params.name,
        symbol: init_token_params.symbol,
        uri: init_token_params.url,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };
    let metadata_ctx = Cpicontext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            payer: ctx.accounts.payer.to_account_info(),
            update_authority: ctx.accounts.mint.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
            mint_authority: ctx.accounts.mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        signer,
    );
    create_metadata_accounts_V3(
        metadata_ctx,
        token_data,
        fasle,
        true,
        None,
    )?;

    msg!("Token created created");

    // ------ Initialize Pool ------ //
    let mut pool_state = ctx.accounts.pool_state.load_init()?;
    let bump = ctx.bumps.pool_state;
    pool_state.initialize(
        bump,
        ctx.accounts.mint.key(),
        ctx.accounts.meme_vault.key(),
        ctx.accounts.pool_config.to_account_info().clone().key(), 
    )?;
    
    let rent_amount = ctx.accounts.pool_state.get_lamports();
    pool_state.set_rent_amount(rent_amount)?;

    msg!("Initialized pool state successfully");

    // ------ Mint MemeToken to Pool ------ //
    mint_to(
        Cpicontext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.meme_vault.to_account_info(),
                authority: ctx.accounts.mint.to_account_info(),
            },
            signer,
        ),
        TOTAL_SUPPLY
    )?;
    msg!("Minted meme token to pool");

    // ------ Remove mint authority ------ //
    let seeds = &[
        MINT_SEED,
        ctx.accounts.mint_config.seed.as_ref(),
        &[ctx.bumps.mint],
    ];
    let signer = &[&seeds[..]];

    let cpi_ctx = Cpicontext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        SetAuthority {
            current_authority: ctx.accounts.mint.to_account_info(),
            account_or_mint: ctx.accounts.mint.to_account_info(),
        },
        signer,
    );

    set_authority(
        cpi_ctx,
        AuthorityType::MintTokens,
        None
    )?;

    msg!("Removed mint authority");

    Ok(())
}
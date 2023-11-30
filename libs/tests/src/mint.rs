use anchor_lang::{
    prelude::{Pubkey, Rent, Result},
    AccountDeserialize,
};
use anchor_spl::token::{
    spl_token::{self, instruction::initialize_mint2},
    Mint,
};
use anyhow::anyhow;
use solana_program_test::ProgramTestContext;
use solana_sdk::{
    hash::Hash,
    signer::{keypair::Keypair, Signer},
    system_instruction::create_account,
    transaction::Transaction,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct MintFixture {
    pub ctx: Rc<RefCell<ProgramTestContext>>,
    pub mint_pubkey: Pubkey,
    pub mint: Mint,
}

impl MintFixture {
    pub async fn new(
        ctx: Rc<RefCell<ProgramTestContext>>,
        payer_keypair: &Keypair,
        mint_keypair: Option<&Keypair>,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
        decimals: u8,
    ) -> anyhow::Result<MintFixture> {
        let ctx_ref = Rc::clone(&ctx);

        let new_mint_keypair = Keypair::new();
        let mint_keypair = mint_keypair.unwrap_or(&new_mint_keypair);

        let mint = {
            let mut ctx = ctx.borrow_mut();

            let rent = ctx.banks_client.get_rent().await?;

            let init_account_ix = create_account(
                &payer_keypair.pubkey(),
                &mint_keypair.pubkey(),
                rent.minimum_balance(Mint::LEN),
                Mint::LEN as u64,
                &spl_token::id(),
            );

            let init_mint_ix = initialize_mint2(
                &spl_token::id(),
                &mint_keypair.pubkey(),
                mint_authority,
                freeze_authority,
                decimals,
            )?;

            let tx = Transaction::new_signed_with_payer(
                &[init_account_ix, init_mint_ix],
                Some(&payer_keypair.pubkey()),
                &[payer_keypair, mint_keypair],
                ctx.last_blockhash,
            );

            ctx.banks_client.process_transaction(tx).await?;

            let mint_account = ctx
                .banks_client
                .get_account(mint_keypair.pubkey())
                .await?
                .ok_or(anyhow!("Cannot find Mint account"))?;

            Mint::try_deserialize(&mut mint_account.data.as_slice())?
        };

        Ok(MintFixture {
            ctx: ctx_ref,
            mint_pubkey: mint_keypair.pubkey(),
            mint,
        })
    }
}

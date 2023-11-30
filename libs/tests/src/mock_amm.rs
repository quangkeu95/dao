use anchor_lang::prelude::*;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

pub struct CreateMockAmmData<'a> {
    pub moc_amm_program_id: &'a Pubkey,
}

pub fn create_mock_amm(data: CreateMockAmmData) {
    let base_keypair = Keypair::new();

    let (mock_amm, _mock_amm_bump) = Pubkey::find_program_address(
        &[b"moc_amm", base_keypair.pubkey().as_ref()],
        data.moc_amm_program_id,
    );

    // anchor_spl::token::initialize_mint2()
}

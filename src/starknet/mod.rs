use std::env;

pub use starknet::accounts::Account;
use starknet::{
	accounts::SingleOwnerAccount,
	core::{chain_id, types::FieldElement},
	providers::SequencerGatewayProvider,
	signers::{LocalWallet, SigningKey},
};

use crate::domain::*;

mod contracts;
use contracts::{ContributionContract, ProfileContract, RegistryContract};

mod model;
pub use model::*;

mod indexing;

pub fn make_account_from_env() -> impl Account {
	let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
	let account_address = env::var("ACCOUNT_ADDRESS").expect("ACCOUNT_ADDRESS must be set");
	make_account(&private_key, &account_address)
}

fn make_account(
	private_key: &str,
	account_address: &str,
) -> SingleOwnerAccount<SequencerGatewayProvider, LocalWallet> {
	let signer = LocalWallet::from(SigningKey::from_secret_scalar(
		FieldElement::from_hex_be(private_key).unwrap(),
	));

	// TODO: make chain_id configurable
	SingleOwnerAccount::new(
		sequencer(),
		signer,
		FieldElement::from_hex_be(account_address).unwrap(),
		chain_id::TESTNET,
	)
}

pub fn sequencer() -> SequencerGatewayProvider {
	SequencerGatewayProvider::starknet_alpha_goerli()
}

pub struct API<'a, A>
where
	A: Account + Sync,
{
	registry: RegistryContract,
	contributions: ContributionContract<'a, A>,
	profile: ProfileContract,
}

impl<'a, A> API<'a, A>
where
	A: Account + Sync,
{
	pub fn new(account: &'a A) -> Self {
		Self {
			registry: RegistryContract::default(),
			contributions: ContributionContract::new(account),
			profile: ProfileContract::default(),
		}
	}

	pub async fn execute_actions(&self, actions: &[Action]) -> Result<String> {
		self.contributions.execute_actions(actions, true).await
	}

	pub async fn get_user_information(
		&self,
		contributor_id: &ContributorId,
	) -> Option<Contributor> {
		let account = self.profile.get_account(contributor_id).await?;
		self.registry.get_user_information(account).await
	}
}

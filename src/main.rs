mod balances;
mod proof_of_existence;
mod support;
mod system;

use crate::support::Dispatch;

mod types {
	use crate::support;

	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;

	pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = support::Header<BlockNumber>;
	pub type Block = support::Block<Header, Extrinsic>;

	pub type Content = String;
}

// pub enum RuntimeCall {
// 	Balances(balances::Call<Runtime>),
// 	ProofOfExistence(proof_of_existence::Call<Runtime>),
// }

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

fn main() {
	let mut runtime = Runtime::new();
	runtime.balances.set_balance(&"alice".to_string(), 100);

	let block_1 = types::Block {
		header: types::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: "alice".to_string(),
				call: RuntimeCall::balances(balances::Call::transfer {
					to: "bob".to_string(),
					amount: 30,
				}),
			},
			support::Extrinsic {
				caller: "alice".to_string(),
				call: RuntimeCall::balances(balances::Call::transfer {
					to: "charlie".to_string(),
					amount: 20,
				}),
			},
		],
	};

	runtime.execute_block(block_1).expect("invalid block");

	let block_2 = support::Block {
		header: types::Header { block_number: 2 },
		extrinsics: vec![support::Extrinsic {
			caller: "alice".to_string(),
			call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
				claim: "I did something amazing".to_string(),
			}),
		}],
	};

	runtime.execute_block(block_2).expect("invalid block");

	let block_3 = types::Block {
		header: types::Header { block_number: 3 },
		extrinsics: vec![types::Extrinsic {
			caller: "bob".to_string(),
			call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
				claim: "I did something amazing".to_string(),
			}),
		}],
	};

	runtime.execute_block(block_3).expect("invalid block");

	println!("{:#?}", runtime);
}

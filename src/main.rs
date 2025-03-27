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

pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
	ProofOfExistence(proof_of_existence::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_exisence: proof_of_existence::Pallet<Self>,
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

impl Runtime {
	fn new() -> Self {
		Self {
			system: system::Pallet::<Self>::new(),
			balances: balances::Pallet::<Self>::new(),
			proof_of_exisence: proof_of_existence::Pallet::<Self>::new(),
		}
	}

	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();
		let current_block_number = self.system.block_number();
		if current_block_number != block.header.block_number {
			return Err("Unexpected block number");
		}

		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}

		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)?;
			},
			RuntimeCall::ProofOfExistence(call) => {
				self.proof_of_exisence.dispatch(caller, call)?;
			},
		}

		Ok(())
	}
}

fn main() {
	let mut runtime = Runtime::new();
	runtime.balances.set_balance(&"alice".to_string(), 100);

	let block_1 = types::Block {
		header: types::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: "alice".to_string(),
				call: RuntimeCall::Balances(balances::Call::Transfer {
					to: "bob".to_string(),
					amount: 30,
				}),
			},
			support::Extrinsic {
				caller: "alice".to_string(),
				call: RuntimeCall::Balances(balances::Call::Transfer {
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
			call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::Create {
				caller: "alice".to_string(),
				claim: "I did something amazing".to_string(),
			}),
		}],
	};

	runtime.execute_block(block_2).expect("invalid block");

	let block_3 = types::Block {
		header: types::Header { block_number: 3 },
		extrinsics: vec![types::Extrinsic {
			caller: "bob".to_string(),
			call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::Revoke {
				caller: "bob".to_string(),
				claim: "I did something amazing".to_string(),
			}),
		}],
	};

	runtime.execute_block(block_3).expect("invalid block");

	println!("{:#?}", runtime);
}

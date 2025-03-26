mod balances;
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
}

pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::<Self>::new(), balances: balances::Pallet::<Self>::new() }
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
		}

		Ok(())
	}
}

fn main() {
	let mut runtime = Runtime::new();
	runtime.balances.set_balance(&"alice".to_string(), 100);

	runtime.system.inc_block_number();
	// assert_eq!(runtime.system.block_number(), 1);

	// runtime.system.inc_nonce(&"alice".to_string());
	// let _res = runtime
	// 	.balances
	// 	.transfer("alice".to_string(), "bob".to_string(), 30)
	// 	.map_err(|e| eprintln!("{}", e));

	// runtime.system.inc_nonce(&"alice".to_string());

	// let _res = runtime
	// 	.balances
	// 	.transfer("alice".to_string(), "charlie".to_string(), 20)
	// 	.map_err(|e| eprintln!("{}", e));

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

	println!("{:#?}", runtime);
}

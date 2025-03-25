use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config {
	type AccountId: Ord + Clone;
	type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(&caller);
		let recipent_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Insufficient funds")?;
		let new_recipient_balance =
			recipent_balance.checked_add(&amount).ok_or("An overflow occurred.")?;

		self.set_balance(&caller, new_caller_balance);
		self.set_balance(&to, new_recipient_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::Pallet;

	#[test]
	fn init_balances() {
		struct TestStruct;
		impl super::Config for TestStruct {
			type AccountId = String;
			type Balance = u128;
		}
		let mut balances = Pallet::<TestStruct>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&String::from("alice"), 100);

		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balances() {
		struct TestStruct;
		impl super::Config for TestStruct {
			type AccountId = String;
			type Balance = u128;
		}
		let mut balances = Pallet::<TestStruct>::new();

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 100),
			Err("Insufficient funds")
		);
		balances.set_balance(&"alice".to_string(), 100);
		balances.transfer("alice".to_string(), "bob".to_string(), 50);

		assert_eq!(balances.balance(&"alice".to_string()), 50);
		assert_eq!(balances.balance(&"bob".to_string()), 50);
	}
}

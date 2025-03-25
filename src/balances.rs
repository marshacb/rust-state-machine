use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
	balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
	AccountId: Ord + Clone,
	Balance: Zero + CheckedSub + CheckedAdd + Copy,
{
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &AccountId) -> Balance {
		*self.balances.get(who).unwrap_or(&Balance::zero())
	}

	pub fn transfer(
		&mut self,
		caller: AccountId,
		to: AccountId,
		amount: Balance,
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
		let mut balances = Pallet::<String, u128>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&String::from("alice"), 100);

		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balances() {
		let mut balances = Pallet::<String, u128>::new();

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

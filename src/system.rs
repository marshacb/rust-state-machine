use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap;
#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
	AccountId: Ord + Clone,
	BlockNumber: One + Zero + AddAssign + Copy,
	Nonce: One + Zero + Copy,
{
	pub fn new() -> Self {
		Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += BlockNumber::one();
	}

	pub fn inc_nonce(&mut self, who: &AccountId) {
		let current_account_nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());

		let new_nonce = current_account_nonce + Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_system() {
		let mut system = super::Pallet::<String, u32, u32>::new();

		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(&"alice".to_string()).unwrap_or(&0), &1);
		assert_eq!(system.nonce.get(&"bob".to_string()).unwrap_or(&0), &0);
	}
}

use std::{collections::BTreeMap, ops::Add};

#[derive(Debug)]
pub struct Pallet {
	block_number: u32,
	nonce: BTreeMap<String, u32>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> u32 {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	pub fn inc_nonce(&mut self, who: &String) {
		let current_account_nonce = self.nonce.get(&who.clone()).unwrap_or(&0);

		self.nonce.insert(who.to_string(), *current_account_nonce + 1);
	}
}

#[cfg(test)]
mod tests {
	use crate::system::Pallet;

	#[test]
	fn init_system() {
		let mut system = Pallet::new();

		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(&"alice".to_string()).unwrap_or(&0), &1);
		assert_eq!(system.nonce.get(&"bob".to_string()).unwrap_or(&0), &0);
	}
}

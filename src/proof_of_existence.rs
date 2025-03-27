use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::{Dispatch, DispatchResult};

pub trait Config: crate::system::Config {
	type Content: Debug + Ord;
}

pub enum Call<T: Config> {
	Create { caller: T::AccountId, claim: T::Content },
	Revoke { caller: T::AccountId, claim: T::Content },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountId;
	type Call = Call<T>;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
		match call {
			Call::Create { caller, claim } => {
				self.create_claim(caller, claim)?;
			},
			Call::Revoke { caller, claim } => {
				self.revoke_claim(caller, claim)?;
			},
		}

		Ok(())
	}
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}

	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err(&"this content is already claimed");
		}

		self.claims.insert(claim, caller);

		Ok(())
	}

	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let claim_owner = self.claims.get(&claim).ok_or("claim doe not exist")?;

		if caller != *claim_owner {
			return Err("this content is owned by someone else");
		}

		self.claims.remove(&claim);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	struct TestStruct;
	impl super::Config for TestStruct {
		type Content = String;
	}
	impl crate::system::Config for TestStruct {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}
	#[test]
	fn basic_proof_of_existence() {
		let mut proof_of_existence = super::Pallet::<TestStruct>::new();

		// claim should exist initially
		assert_eq!(proof_of_existence.get_claim(&"I exist".to_string()), None);

		// create a cliam
		let _res = proof_of_existence.create_claim("alice".to_string(), "I exist".to_string());

		// claim should now exist
		assert_eq!(
			proof_of_existence.get_claim(&"I exist".to_string()),
			Some(&"alice".to_string())
		);

		// revoke alice claim with bob should fail
		assert_eq!(
			proof_of_existence.revoke_claim("bob".to_string(), "I exist".to_string()),
			Err("this content is owned by someone else")
		);

		// revoke claim by alice should correctly remove the claim
		let _res = proof_of_existence.revoke_claim("alice".to_string(), "I exist".to_string());
		assert_eq!(proof_of_existence.get_claim(&"I exist".to_string()), None);
	}
}

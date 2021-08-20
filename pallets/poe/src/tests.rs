use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), Some((1, frame_system::Pallet::<Test>::block_number())));
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofAlreadyExist);
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), None);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofAlreadyExist);
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
		assert_eq!(Proofs::<Test>::get(&claim), Some((2, frame_system::Pallet::<Test>::block_number())));
		assert_noop!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2), Error::<Test>::NotClaimOwner);
	})
}

#[test]
fn exceed_vec_len_limit() {
	new_test_ext().execute_with(|| {
		let mut claim = Vec::new();
		for _ in 0..10 {
			claim.push(0);
		}
		assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::BadMetadata);
		assert_eq!(Proofs::<Test>::get(&claim), None);
		let claim_correct = vec![0,0,0,0,0,0,0,0,0];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim_correct.clone()));
	})
}

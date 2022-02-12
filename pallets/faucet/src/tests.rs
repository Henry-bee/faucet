use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, traits::StoredMap};

#[test]
fn test_drip() {
	new_test_ext().execute_with(|| {
		let account = Origin::signed(1);
		// Dispatch the first drip extrinsic for Account 1.
		let init_acc_balance = AccountStore::get(&1).free;
		let amount = 100000u64;
		let initial_supply = Balances::total_issuance();

		let res = Faucet::drip(account.clone(), amount);
		assert!(res.is_ok());

		let post_drip_balance = AccountStore::get(&1).free;
		let post_supply = Balances::total_issuance();
		assert_eq!(post_drip_balance, init_acc_balance + amount);
		assert_eq!(post_supply, initial_supply + amount);

		assert!(System::events()
			.iter()
			.any(|er| er.event == TestEvent::Faucet(Event::Dripped(1, amount))));

		// Dispatch another drip extrinsic for Account 1.
		let next_res = Faucet::drip(account, 10000);
		assert_noop!(next_res, Error::<Test>::DripExceeded);
	});
}

#[test]
fn test_drip_amount_exceeded() {
	new_test_ext().execute_with(|| {
		let account_1 = Origin::signed(1);

		// Attempt to drip more than 10_000_001 tokens
		let res = Faucet::drip(account_1.clone(), 10_000_001);
		assert_eq!(res.is_err(), true);
		assert_noop!(res, Error::<Test>::MaxDripAmountExceeded);
	});
}

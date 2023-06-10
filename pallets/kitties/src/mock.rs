use crate as pallet_kitties;
use frame_support::{traits::{ConstU16,ConstU32, ConstU64,ConstU128}, parameter_types, PalletId};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

use pallet_insecure_randomness_collective_flip;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		KittiesModule: pallet_kitties,
		Randomness:pallet_insecure_randomness_collective_flip,
		Balances:pallet_balances,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;// why change value from () to pallet_balances::AccountData<Balance> then now compile error?
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub KittyPalletId:PalletId=PalletId(*b"py/kitty");
}


impl pallet_kitties::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Randomness=Randomness;
	type Currency = Balances;
	type KittyPrice = ConstU128<1000_000>;
	type PalletId=KittyPalletId;
}

impl pallet_insecure_randomness_collective_flip::Config for Test{}

pub type Balance = u128;
pub const EXISTENTIAL_DEPOSIT: u128 = 500;

impl pallet_balances::Config for Test{
  type MaxLocks = ConstU32<50>;
  type MaxReserves = ();
  type ReserveIdentifier = [u8; 8];
  /// The type for recording an account's balance.
  type Balance = Balance;
  /// The ubiquitous event type.
  type RuntimeEvent = RuntimeEvent;
  /// The empty value, (), is used to specify a no-op callback function.
  type DustRemoval = ();
  /// Set the minimum balanced required for an account to exist on-chain
  type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
  /// The FRAME runtime system is used to track the accounts that hold balances.
  type AccountStore = System;
  /// Weight information is supplied to the Balances pallet by the node template runtime.
  type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;

}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext:sp_io::TestExternalities=frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	ext.execute_with(||System::set_block_number(1));
	ext
}

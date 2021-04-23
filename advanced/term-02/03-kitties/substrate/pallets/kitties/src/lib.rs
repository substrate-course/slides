#![cfg_attr(not(feature = "std"), no_std)]

use crate::link::{LinkedList, LinkedItem};

mod link;

#[derive(Encode, Decode, RuntimeDebug, Clone, PartialEq)]
pub struct Kitty(pub [u8; 16]);

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
		type KittyIndex: Parameter + Member + AtLeast32Bit + Default + Copy;
		type Currency: Currency<Self::AccountId>;
		type Randomness: Randomness<Self::Hash>;
	}

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	type KittyLinkedItem<T> = LinkedItem<<T as Config>::KittyIndex>;
	type OwnedKittiesList<T> = LinkedList<OwnedKitties<T>, <T as system::Config>::AccountId, <T as Config>::KittyIndex>;

	// Storage section

	#[pallet::storage]
	#[pallet::getter(kitties)]
	pub(super) type Kitties<T> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Kitty>;

	// NEXT-TODO>



decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;

		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(fn kitties_count): T::KittyIndex;

		/// Store owned kitties in a linked list.
		pub OwnedKitties get(fn owned_kitties): map hasher(blake2_128_concat)
			(T::AccountId, Option<T::KittyIndex>) => Option<KittyLinkedItem<T>>;

		/// Store owner of each kitity.
		pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) T::KittyIndex =>
			Option<T::AccountId>;

		/// Get kitty price. None means not for sale.
		pub KittyPrices get(fn kitty_price): map hasher(blake2_128_concat) T::KittyIndex =>
			Option<BalanceOf<T>>;
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesCountOverflow,
		InvalidKittyId,
		RequireDifferentParent,
		RequireOwner,
		NotForSale,
		PriceTooLow,
	}
}

decl_event!(
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
		<T as Trait>::KittyIndex,
		Balance = BalanceOf<T>,
	{
		/// A kitty is created. (owner, kitty_id)
		Created(AccountId, KittyIndex),
		/// A kitty is transferred. (from, to, kitty_id)
		Transferred(AccountId, AccountId, KittyIndex),
		/// A kitty is available for sale. (owner, kitty_id, price)
		Ask(AccountId, KittyIndex, Option<Balance>),
		/// A kitty is sold. (from, to, kitty_id, price)
		Sold(AccountId, AccountId, KittyIndex, Balance),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		/// Create a new kitty
		#[weight = 0]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let kitty_id = Self::next_kitty_id()?;

			// Generate a random 128bit value
			let dna = Self::random_value(&sender);

			// Create and store kitty
			let kitty = Kitty(dna);
			Self::insert_kitty(&sender, kitty_id, kitty);

			Self::deposit_event(RawEvent::Created(sender, kitty_id));
		}

		/// Breed kitties
		#[weight = 0]
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
			let sender = ensure_signed(origin)?;

			let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;

			Self::deposit_event(RawEvent::Created(sender, new_kitty_id));
		}

		/// Transfer a kitty to new owner
		#[weight = 0]
		pub fn transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex) {
			let sender = ensure_signed(origin)?;

			// jc-note: This line failed, so I am checking `kitty_owner`,
			//   ref: https://github.com/SubstrateCourse/kitties-course/issues/3
			// ensure!(<OwnedKitties<T>>::contains_key((&sender, Some(kitty_id))), Error::<T>::RequireOwner);
			ensure!(Self::kitty_owner(kitty_id) == Some(sender.clone()), "Kitty is not owned by sender.");

			Self::do_transfer(&sender, &to, kitty_id);

			Self::deposit_event(RawEvent::Transferred(sender, to, kitty_id));
		}

		/// Set a price for a kitty for sale
		/// None to delist the kitty
		#[weight = 0]
		pub fn ask(origin, kitty_id: T::KittyIndex, new_price: Option<BalanceOf<T>>) {
			let sender = ensure_signed(origin)?;

			ensure!(<OwnedKitties<T>>::contains_key((&sender, Some(kitty_id))), Error::<T>::RequireOwner);

			<KittyPrices<T>>::mutate_exists(kitty_id, |price| *price = new_price);

			Self::deposit_event(RawEvent::Ask(sender, kitty_id, new_price));
		}

		/// Buy a kitty
		#[weight = 0]
		pub fn buy(origin, kitty_id: T::KittyIndex, price: BalanceOf<T>) {
			let sender = ensure_signed(origin)?;

			let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;

			let kitty_price = Self::kitty_price(kitty_id).ok_or(Error::<T>::NotForSale)?;

			ensure!(price >= kitty_price, Error::<T>::PriceTooLow);

			T::Currency::transfer(&sender, &owner, kitty_price, ExistenceRequirement::KeepAlive)?;

			<KittyPrices<T>>::remove(kitty_id);

			Self::do_transfer(&owner, &sender, kitty_id);

			Self::deposit_event(RawEvent::Sold(owner, sender, kitty_id, kitty_price));
		}
	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	(selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> {
	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (
			T::Randomness::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index(),
		);
		payload.using_encoded(blake2_128)
	}

	fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError> {
		let kitty_id = Self::kitties_count();
		if kitty_id == T::KittyIndex::max_value() {
			return Err(Error::<T>::KittiesCountOverflow.into());
		}
		Ok(kitty_id)
	}

	fn insert_owned_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex) {
		<OwnedKittiesList<T>>::append(owner, kitty_id);
		<KittyOwners<T>>::insert(kitty_id, owner);
	}

	fn insert_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
		// Create and store kitty
		Kitties::<T>::insert(kitty_id, kitty.clone());
		KittiesCount::<T>::put(kitty_id + 1.into());
		Self::insert_owned_kitty(owner, kitty_id);
	}

	fn do_breed(sender: &T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> sp_std::result::Result<T::KittyIndex, DispatchError> {
		let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
		let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

		ensure!(<OwnedKitties<T>>::contains_key((&sender, Some(kitty_id_1))), Error::<T>::RequireOwner);
		ensure!(<OwnedKitties<T>>::contains_key((&sender, Some(kitty_id_2))), Error::<T>::RequireOwner);
		ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.0;
		let kitty2_dna = kitty2.0;

		// Generate a random 128bit value
		let selector = Self::random_value(&sender);
		let mut new_dna = [0u8; 16];

		// Combine parents and selector to create new kitty
		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

		Ok(kitty_id)
	}

	fn do_transfer(from: &T::AccountId, to: &T::AccountId, kitty_id: T::KittyIndex)  {
		<OwnedKittiesList<T>>::remove(&from, kitty_id);
		Self::insert_owned_kitty(&to, kitty_id);
	}
}

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use sp_core::H256;
	use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
	};
	use frame_system as system;

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq, Debug)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type BaseCallFilter = ();
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type DbWeight = ();
		type BlockExecutionWeight = ();
		type ExtrinsicBaseWeight = ();
		type MaximumExtrinsicWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
		type SystemWeightInfo = ();
		type PalletInfo = PalletInfo;
		type AccountData = ();
		type OnNewAccount = ();
		type OnKilledAccount = ();
		}

	impl Trait for Test {
		type Event = ();
		type Currency: Balances;
		type Randomness: Randomness<H256>;
		type KittyIndex = u32;
	}
	type OwnedKittiesTest = OwnedKitties<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn owned_kitties_can_append_values() {
		new_test_ext().execute_with(|| {
			OwnedKittiesTest::append(&1);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
				prev: Some(1),
				next: Some(1),
			}));
		});
	}

	#[test]
	fn owned_kitties_can_remove_values() {
		new_test_ext().execute_with(|| {
			OwnedKittiesTest::append(1);
			OwnedKittiesTest::append(2);
			OwnedKittiesTest::append(3);

			OwnedKittiesTest::remove(2);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
				prev: Some(3),
				next: Some(1),
			}));
		});
	}
}

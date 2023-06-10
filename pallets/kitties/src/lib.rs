#![cfg_attr(not(feature = "std"), no_std)]

/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>

pub use pallet::*;

mod migrations;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, ensure,PalletId,weights::Weight};
	use frame_system::pallet_prelude::*;
	use frame_support::traits::{Randomness,Currency,ExistenceRequirement};
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::AccountIdConversion;
	use crate::migrations;

	pub type KittyId=u32;

	#[derive(Encode, Decode, Clone, Copy, Default,RuntimeDebug, PartialEq,Eq,TypeInfo, MaxEncodedLen)]
	//pub struct Kitty(pub [u8;16]);
	pub struct Kitty{
		pub dna:[u8;16],
		pub name:[u8;4],
	}

	const STORAGE_VERSION:StorageVersion=StorageVersion::new(1);

	pub type BalanceOf<T>=<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Randomness:Randomness<Self::Hash, Self::BlockNumber>;
		type Currency:Currency<Self::AccountId>;
		
		#[pallet::constant]
		type KittyPrice:Get<BalanceOf<Self>>;
		type PalletId:Get<PalletId>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type NextKittyId<T:Config> = StorageValue<_, KittyId,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T:Config> = StorageMap<_,Blake2_128Concat, KittyId,Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T:Config> = StorageMap<_,Blake2_128Concat, KittyId,T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_parents)]
	pub type KittyParents<T:Config> = StorageMap<_,Blake2_128Concat, KittyId,(KittyId,KittyId),OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_on_sale)]
	pub type KittyOnSale<T:Config>=StorageMap<_, Blake2_128Concat,KittyId, ()>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event

		KittyCreated { who:T::AccountId,kitty_id:KittyId, kitty:Kitty},
		KittyBreed { who:T::AccountId,kitty_id:KittyId, kitty:Kitty},
		KittyTransferred { who:T::AccountId,recipient:T::AccountId,kitty_id:KittyId},
		KittyOnSale { who:T::AccountId,kitty_id:KittyId},
		KittyBought { who:T::AccountId,kitty_id:KittyId},

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		InvalidKittyId,
		SameKittyId,
		NotOwner,
		AlreadyOnSale,
		NotOnSale,
		AlreadyOwned,
	}

	#[pallet::hooks]
	impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T>{
		fn on_runtime_upgrade()->Weight{
			migrations::v1::migrate::<T>()
		}
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>,name:[u8;4]) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_id=Self::get_next_id()?;
			let dna=Self::random_value(&who);
			//let kitty=Kitty{(Self::random_value(&who))};
			let kitty=Kitty{
				dna,
				name,
			};

			let price=T::KittyPrice::get();
			//T::Currency::reserve(&who, price)?; 
			T::Currency::transfer(&who,&Self::get_account_id(),price, ExistenceRequirement::KeepAlive)?;
			
			Kitties::<T>::insert(kitty_id,&kitty);
			KittyOwner::<T>::insert(kitty_id,&who);

			// Emit an event.
			Self::deposit_event(Event::KittyCreated { who,kitty_id, kitty});
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn breed(origin: OriginFor<T>,kitty_id_1:KittyId, kitty_id_2:KittyId,name:[u8;4]) -> DispatchResult {

			//check whether signed account
			let who = ensure_signed(origin)?;

			//check whether kitty1 not equal kitty2
			ensure!(kitty_id_1!=kitty_id_2,Error::<T>::SameKittyId);

			//check whether kitty1 and kitty2 exist
			ensure!(Kitties::<T>::contains_key(kitty_id_1),Error::<T>::InvalidKittyId);
			ensure!(Kitties::<T>::contains_key(kitty_id_2),Error::<T>::InvalidKittyId);

			//create new kitty from kitty1 and kitty2
			let kitty_id=Self::get_next_id()?;
			// let kitty_1=Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
			// let kitty_2=Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

			//let selector=Self::random_value(&who);
			//let mut data=[0u8;16];
			// for i in 0..kitty_1.0.len(){
			// 	data[i]=(kitty_1.0[i]&selector[i])|(kitty_2.0[i]&selector[i]);
			// }
			//let kitty=Kitty(data);

			let dna=Self::random_value(&who);
			let kitty=Kitty{
				dna,
				name,
			};

			let price=T::KittyPrice::get();
			//T::Currency::reserve(&who, price)?; 
			T::Currency::transfer(&who,&Self::get_account_id(),price, ExistenceRequirement::KeepAlive)?;

			//store new kitty to Kitties
			Kitties::<T>::insert(kitty_id, &kitty);

			//store owner to KittyOwner
			KittyOwner::<T>::insert(kitty_id, &who);

			//store parents to KittyParents
			KittyParents::<T>::insert(kitty_id, (kitty_id_1,kitty_id_2));

			//emit event
			Self::deposit_event(Event::KittyBreed { who,kitty_id, kitty});

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn transfer(origin: OriginFor<T>,recipient:T::AccountId, kitty_id:KittyId) -> DispatchResult {

			//check whether signed account
			let who = ensure_signed(origin)?;

			//check whether kitty exist
			ensure!(Kitties::<T>::contains_key(kitty_id),Error::<T>::InvalidKittyId);

			//check whether owner is right
			let owner=Self::kitty_owner(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			ensure!(owner==who,Error::<T>::NotOwner);

			//store owner to KittyOwner
			KittyOwner::<T>::insert(kitty_id, &recipient);

			//emit event
			Self::deposit_event(Event::KittyTransferred { who,recipient,kitty_id});
			
			Ok(())
		}


	#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn sale(origin: OriginFor<T>,kitty_id:u32) -> DispatchResult {

			let who = ensure_signed(origin)?;

			Self::kitties(kitty_id).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
			ensure!(Self::kitty_owner(kitty_id)==Some(who.clone()),Error::<T>::NotOwner);
			ensure!(Self::kitty_on_sale(kitty_id).is_none(), Error::<T>::AlreadyOnSale);
			
			KittyOnSale::<T>::insert(kitty_id,());

			// Emit an event.
			Self::deposit_event(Event::KittyOnSale { who,kitty_id});
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		pub fn buy(origin: OriginFor<T>,kitty_id:u32) -> DispatchResult {

			let who = ensure_signed(origin)?;

			Self::kitties(kitty_id).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
			let owner=Self::kitty_owner(kitty_id).ok_or::<DispatchError>(Error::<T>::NotOwner.into())?;

			ensure!(owner!=who,Error::<T>::AlreadyOwned);
			ensure!(Self::kitty_on_sale(kitty_id).is_some(), Error::<T>::NotOnSale);

			let price=T::KittyPrice::get();
			//<T>::Currency::reserve(&who,price)?;
			//<T>::Currency::unreserve(&owner, price);
			T::Currency::transfer(&owner,&who, price, ExistenceRequirement::KeepAlive)?;

			KittyOwner::<T>::insert(kitty_id, &who);
			KittyOnSale::<T>::remove(kitty_id);

			// Emit an event.
			Self::deposit_event(Event::KittyBought { who,kitty_id});
			Ok(())
		}
}
	impl<T:Config> Pallet<T>{
		 fn get_next_id()->Result<KittyId,DispatchError>{
			NextKittyId::<T>::try_mutate(|next_id|->Result<KittyId,DispatchError>{

				let current_id=*next_id;
				*next_id=next_id.checked_add(1).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;

				Ok(current_id)
			})

		}

		pub fn random_value(sender:&T::AccountId)->[u8;16]{
			let payload=(
				T::Randomness::random_seed(),
				sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			
			payload.using_encoded(blake2_128)
		}


		fn get_account_id()->T::AccountId{
			T::PalletId::get().into_account_truncating()
		}
	}
}

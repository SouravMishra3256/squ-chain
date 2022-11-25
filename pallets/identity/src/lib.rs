#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	pub type Id = [u8; 32];
  pub type AccountId = u64;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	pub type Lookup<T: Config> = StorageMap<_, Blake2_128Concat, Id, AccountId, OptionQuery>;

	#[pallet::storage]
	pub type RLookup<T: Config> = StorageMap<_, Blake2_128Concat, AccountId, Id, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		IdCreated(Id, AccountId),
		IdRemoved(Id)
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The given account id already exists.
		AccountAlreadyExists,
		/// The given account id does not exist.
    AccountDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_identity(_origin: OriginFor<T>, id : Id, account_id : AccountId) -> DispatchResult {
			ensure!(!Self::does_identity_exist(id), Error::<T>::AccountAlreadyExists);
			Lookup::<T>::insert(id, account_id);
			RLookup::<T>::insert(account_id, id);

			Self::deposit_event(Event::IdCreated(id, account_id));
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn remmove_identity(_origin: OriginFor<T>, id: Id) -> DispatchResult {
			ensure!(!Self::does_identity_exist(id), Error::<T>::AccountDoesNotExist);
			let account_id = Lookup::<T>::get(id).unwrap();
			RLookup::<T>::remove(account_id);
			Lookup::<T>::remove(id);

			Self::deposit_event(Event::IdRemoved(id));
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn does_identity_exist(id: Id) -> bool {
		Lookup::<T>::contains_key(id)
	}
}
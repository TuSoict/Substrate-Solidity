#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use sp_std::vec::Vec;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
mod migration;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// The current storage version, we set to 2 our new version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			migration::migrate_to_v2::<T>()
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_template::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The minimum length a name may be.
		#[pallet::constant]
		type MinLength: Get<u32>;

		/// The maximum length a name may be.
		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	#[derive(Encode, Decode, Default, TypeInfo, MaxEncodedLen, PartialEqNoBound, RuntimeDebug)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Nickname<T: Config> {
		pub first: BoundedVec<u8, T::MaxLength>,
		pub last: Option<BoundedVec<u8, T::MaxLength>>,
	}

	#[pallet::storage]
	#[pallet::getter(fn get_name)]
	pub type GetName<T: Config> = StorageMap<_, Twox64Concat, AccountIdOf<T>, Nickname<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetName { name: Vec<u8>, who: T::AccountId },
		UpdateStorage,
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// A name is too short.
		TooShort,
		/// A name is too long.
		TooLong,
		/// An account isn't named.
		Unnamed,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_something(origin: OriginFor<T>, new_value: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let _ = ensure_signed(origin)?;

			// Update storage.
			pallet_template::Pallet::<T>::update_storage(new_value)?;

			// Emit an event.
			Self::deposit_event(Event::UpdateStorage);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn set_name(
			origin: OriginFor<T>,
			first: Vec<u8>,
			last: Option<Vec<u8>>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			let bounded_first: BoundedVec<_, _> =
				first.clone().try_into().map_err(|_| Error::<T>::TooLong)?;
			ensure!(bounded_first.len() >= T::MinLength::get() as usize, Error::<T>::TooShort);
			let mut bounded_last: BoundedVec<_, _> = Default::default();
			if let Some(last) = last {
				bounded_last = last.try_into().map_err(|_| Error::<T>::TooLong)?;
				ensure!(bounded_last.len() >= T::MinLength::get() as usize, Error::<T>::TooShort);
			}
			let bounded_last: Option<BoundedVec<u8, T::MaxLength>> = Some(bounded_last);

			// Update storage.
			GetName::<T>::insert(&who, Nickname { first: bounded_first, last: bounded_last });

			// Emit an event.
			// Self::deposit_event(Event::SetName { name, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}

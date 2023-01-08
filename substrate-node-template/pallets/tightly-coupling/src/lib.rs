#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use scale_info::prelude::vec::Vec;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>; // StorageMap, StorageDoubleMap

	#[pallet::storage]
	#[pallet::getter(fn name_storage)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type NameStorage<T: Config> =
		StorageMap<_, Blake2_256, T::AccountId, BoundedVec<u8, T::MaxLength>>; // StorageMap, StorageDoubleMap

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored {
			something: u32,
			who: T::AccountId,
		},
		SetName(T::AccountId, Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		TooLong,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// /// An example dispatchable that takes a singles value as a parameter, writes the value to
		// /// storage and emits an event. This function must be dispatched by a signed extrinsic.
		// #[pallet::call_index(0)]
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		// pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
		// 	// Check that the extrinsic was signed and get the signer.
		// 	// This function will return an error if the extrinsic is not signed.
		// 	// https://docs.substrate.io/main-docs/build/origins/
		// 	let who = ensure_signed(origin)?;

		// 	// Update storage.
		// 	<Something<T>>::put(something);

		// 	// Emit an event.
		// 	Self::deposit_event(Event::SomethingStored { something, who });
		// 	// Return a successful DispatchResultWithPostInfo
		// 	Ok(())
		// }

		// /// An example dispatchable that may throw a custom error.
		// #[pallet::call_index(1)]
		// #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		// pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	// Read a value from storage.
		// 	match <Something<T>>::get() {
		// 		// Return an error if the value has not been set.
		// 		None => return Err(Error::<T>::NoneValue.into()),
		// 		Some(old) => {
		// 			// Increment the value read from storage; will error in the event of overflow.
		// 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
		// 			// Update the value in storage with the incremented result.
		// 			<Something<T>>::put(new);
		// 			Ok(())
		// 		},
		// 	}
		// }

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn set_name(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// let bounded_name: BoundedVec<_, _> =
			// 	name.clone().try_into().map_err(|_| Error::<T>::TooLong)?;
			// // Update storage.
			// NameStorage::<T>::insert(&who, bounded_name);
			Self::set_name(name)?;

			// Emit an event.
			Self::deposit_event(Event::SetName(who, name));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}

// helper function
impl<T: Config> Pallet<T> {
	pub fn update_storage(new_value: u32) -> DispatchResult {
		Something::<T>::put(new_value);
		Ok(())
	}

	pub fn set_name(new_name: Vec<u8>) -> DispatchResult {
		let bounded_name: BoundedVec<_, _> =
			name.clone().try_into().map_err(|_| Error::<T>::TooLong)?;
		// Update storage.
		NameStorage::<T>::insert(&who, bounded_name);
	}
}

pub trait DoSome {
	fn increase_value(value: u32) -> u32;
}

impl<T> DoSome for Pallet<T> {
	fn increase_value(value: u32) -> u32 {
		value + 5
	}
}

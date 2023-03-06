use super::*;
use frame_support::{
	storage_alias,
	traits::{Get, GetStorageVersion, StorageVersion},
	weights::Weight,
	BoundedVec, Twox64Concat,
};
use log::info;
const LOG_TARGET: &str = "nicks";

// only contains V1 storage format
pub mod v1 {
	use super::*;
	#[storage_alias]
	pub(super) type GetName<T: Config> = StorageMap<
		Pallet<T>,
		Twox64Concat,
		<T as frame_system::Config>::AccountId,
		BoundedVec<u8, <T as pallet::Config>::MaxLength>,
	>;
}

// contains checks and transforms storage to V2 format
pub fn migrate_to_v2<T: Config>() -> Weight {
	let onchain_version = Pallet::<T>::on_chain_storage_version();
	if onchain_version < 2 {
		// migrate to v2
		// Very inefficient, mostly here for illustration purposes.
		let count = v1::GetName::<T>::iter().count();
		info!(
			target: LOG_TARGET,
			" >>> Updating MyNicks storage. Migrating {} nicknames...", count
		);

		// We transform the storage values from the old into the new format.
		GetName::<T>::translate::<(Vec<u8>), _>(|k: T::AccountId, (nick): (Vec<u8>)| {
			info!(target: LOG_TARGET, "     Migrated nickname for {:?}...", k);

			// We split the nick at ' ' (<space>).
			match nick.iter().rposition(|&x| x == b" "[0]) {
				Some(ndx) => {
					let bounded_first: BoundedVec<_, _> = nick[0..ndx].to_vec().try_into().unwrap();
					let bounded_last: BoundedVec<_, _> =
						nick[ndx + 1..].to_vec().try_into().unwrap();
					Some(Nickname { first: bounded_first, last: Some(bounded_last) })
				},
				None => {
					let bounded_name: BoundedVec<_, _> = nick.to_vec().try_into().unwrap();
					Some(Nickname { first: bounded_name, last: None })
				},
			}
		});

		// Update storage version.
		StorageVersion::new(2).put::<Pallet<T>>();
		// Very inefficient, mostly here for illustration purposes.
		let count = GetName::<T>::iter().count();
		info!(target: LOG_TARGET, " <<< MyNicks storage updated! Migrated {} nicknames ✅", count);
		// Return the weight consumed by the migration.
		T::DbWeight::get().reads_writes(count as u64 + 1, count as u64 + 1)
	} else {
		info!(target: LOG_TARGET, " >>> Unused migration!");
		// We don't do anything here.
		Weight::zero()
	}
}

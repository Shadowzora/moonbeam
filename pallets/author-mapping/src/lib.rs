// Copyright 2019-2021 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Maps Author Ids as used in nimbus consensus layer to account ids as used i nthe runtime.
//! This should likely be moved to nimbus eventually.
//!
//! This pallet maps AuthorId => AccountId which is most useful when using propositional style
//! queries. This mapping will likely need to go the other way if using exhaustive authority sets.
//! That could either be a seperate pallet, or this pallet could implement a two-way mapping. But
//! for now it it one-way

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet;

pub use pallet::*;

#[pallet]
pub mod pallet {

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use nimbus_primitives::AccountLookup;

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	/// Configuration trait of this pallet.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The type of authority id that will be used at the conensus layer.
		type AuthorId: Member + Parameter + MaybeSerializeDeserialize;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//TODO a call to set / update your associated key should take a deposit to avoid state bloat

		//TODO a call to clear your associated key and get your deposit back

		//TODO a call to clear someone else's defunt key? Maybe get a reward for doing it.
	}

	#[pallet::storage]
	#[pallet::getter(fn account_id_of)]
	/// We maintain a mapping from the AuthorIds used in the consensus layer
	/// to the AccountIds runtime (including this staking pallet).
	type Mapping<T: Config> = StorageMap<_, Twox64Concat, T::AuthorId, T::AccountId, OptionQuery>;

	#[pallet::genesis_config]
	/// Genesis config for author mapping pallet
	pub struct GenesisConfig<T: Config> {
		/// The associations that should exist at chain genesis
		pub author_ids: Vec<(T::AuthorId, T::AccountId)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { author_ids: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for (author_id, account_id) in &self.author_ids {
				Mapping::<T>::insert(author_id, account_id);
			}
		}
	}

	impl<T: Config> AccountLookup<T::AuthorId, T::AccountId> for Pallet<T> {
		fn lookup_account(author: &T::AuthorId) -> Option<T::AccountId> {
			Mapping::<T>::get(author)
		}
	}
}
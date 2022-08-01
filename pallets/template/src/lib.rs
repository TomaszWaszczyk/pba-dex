#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Token<Hash, Balance> {
    pub hash: Hash,
    pub symbol: Vec<u8>,
    pub total_supply: Balance,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TokensStored(u32, T::AccountId),
		Transfered(T::AccountId, T::AccountId, T::Hash, ()),
	}


	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn swap(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;


			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}

	#[derive(Encode, Decode, Clone)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub enum OrderStatus {
    	Created,
    	Filled,
    	Canceled,
	}

	#[derive(Encode, Decode)]
	pub enum OrderType {
    	Buy,
    	Sell,
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		BalanceNotEnough,
		NoLiquidity,
	}
}

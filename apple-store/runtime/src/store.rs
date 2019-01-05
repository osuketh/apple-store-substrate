use srml_support::{StorageValue, dispatch::Result};
use {balances, system::ensure_signed};
use rstd::prelude::*;
use runtime_primitives::traits::Hash;

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn buy_apple() -> Result {

        }

        fn sell_apple() -> Result {

        }

        fn mint_apple() -> Result {

        }

        fn set_value() -> Result {

        }

        fn init_ownership(origin) -> Result {
            ensure!(!<Owner<T>>::exists(), "Owner already exists");
            let sender = ensure_signed(origin)?;
            <Owner<T>>::put(&sender);
            Self::deposit_event(RawEvent::OwnershipTransferred(sender.clone(), sender));
            Ok(())
        }

        fn transfer_ownership(origin, newOwner: T::AccountId) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(sender == Self::owner(), "This function can only be called by the owner");
            <Owner<T>>::put(&newOwner);
            Self::deposit_event(RawEvent::OwnershipTransferred(sender, newOwner));
            Ok(())
        }

    }
}

decl_storage! {
    trait Store for Module<T: Trait> as AppleStore {
        Owner get(owner): T::AccountId;
    }
}

decl_event! {
    pub enum Event<T> where AccountId = <T as system::Trait>>::AccountId {
        OwnershipTransferred(AccountId, AccountId),
    }
}

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

        fn buy_apple(origin, nums: T::Balance) -> Result {
            let sender = ensure_signed(origin)?;  
            let cur_apple_stock_amount = Self::apple_stock_amount();

            ensure!(
                cur_apple_stock_amount >= Some(nums),
                "I am sorry but apples are sold out |o|",
            );

            let decrease_by = <T::Balance as As<u64>>::sa(Self::apple_price) * nums;
            <balances::Module<T>>::decrease_free_balance(&sender, decrease_by)?;
            
            let cur_apple_nums = Self::apple_of(sender.clone());            
            <AppleOf<T>>::insert(sender, cur_apple_nums + nums);            

            <AppleStockAmount<T>>::mutate(|amount| {
                let new_amount = amount.map(|amount| amount - nums);
                *amount = Some(new_amount);
            });  
            Ok(())
        }        

        fn mint_apple(mint_by: T::Balance) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(sender == Self::owner(), "Only owner can mint apples.");           

            <AppleStockAmount<T>>::mutate(|amount| {
                let new_amount = amount.map_or(mint_by, |amount| amount + mint_by);
                *amount = Some(new_amount);
            });            
            Ok(())
        }

        fn set_price(origin, price: u32) -> Result {
            let sender = ensure_signed(origin)?;            
            ensure!(sender == Self::owner(), "Only owner can set the price of an apple.")

            <ApplePrice<T>>::put(price);
            Self::deposit_event(RawEvent::ApplePriceSet(price));
            Ok(())
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
        pub Owner get(owner): T::AccountId;
        pub AppleOf get(apple_of): map T::AccountId => T::Balance;
        pub ApplePrice get(apple_price): u32;
        pub AppleStockAmount get(apple_stock_amount): Option<T::Balance>;
    }
}

decl_event! {
    pub enum Event<T> where AccountId = <T as system::Trait>>::AccountId {
        OwnershipTransferred(AccountId, AccountId),
        ApplePriceSet(u32),
    }
}

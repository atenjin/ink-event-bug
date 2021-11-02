#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use first::TestRef;

#[ink::contract]
mod test2 {
    use crate::TestRef;
    use ink_storage::Lazy;
    use ink_env::call::FromAccountId;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Test2 {
        /// Stores a single `bool` value on the storage.
        value: bool,
        test: Lazy<TestRef>,
    }

    #[ink(event)]
    pub struct Tmp;

    impl Test2 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool, account_id: AccountId) -> Self {
            let test: Lazy<TestRef> = Lazy::new(TestRef::from_account_id(account_id));
            Self {
                value: init_value,
                test,
            }
        }

        // /// Constructor that initializes the `bool` value to `false`.
        // ///
        // /// Constructors can delegate to other constructors.
        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     Self::new(Default::default())
        // }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
            self.env().emit_event(Tmp {});
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}

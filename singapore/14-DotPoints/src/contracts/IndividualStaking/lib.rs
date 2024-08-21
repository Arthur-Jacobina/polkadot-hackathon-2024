#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod IndividualStaking {


    #[ink(storage)]
    pub struct IndividualStaking {
        total_staked: Balance,
        name: Vec<u8>,
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    /// The Staking error types.
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        InsufficientBalance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl IndividualStaking {
    
        #[ink(constructor)]
        pub fn new(total_staked: Balance, name: Vec<u8>) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller() 
            balances.insert(caller, &total_supply);
            Self { 
                total_staked: Default::default(0), 
                name,
                total_supply: Default::default(0),
                balances,
                allowances: Default::default(),
            }
        }

        /// Returns the total staking token supply it must be 1:1 to total staked
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the total staked 
        #[ink(message)]
        pub fn total_staked(&self) -> Balance {
            self.total_staked
        }

        // Returns the token name.
        #[ink(message)]
        pub fn name(&self) -> Vec<u8> {
            self.name
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_impl(&owner)
        }
        
        /// Returns the account balance for the wallet
        #[inline]
        fn balance_of_impl(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance_impl(&from, &caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance)
            }
            self.transfer_from_to(&from, &to, value)?;
            // We checked that allowance >= value
            #[allow(clippy::arithmetic_side_effects)]
            self.allowances
                .insert((&from, &caller), &(allowance - value));
            Ok(())
        }

        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of_impl(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance)
            }
            // We checked that from_balance >= value
            #[allow(clippy::arithmetic_side_effects)]
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of_impl(to);
            self.balances
                .insert(to, &(to_balance.checked_add(value).unwrap()));
            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });
            Ok(())
        }

        /// Stake an amount and mint staking tokens to the user
        #[ink(message, payable)]
        pub fn staking(&mut self, from: AccountId) -> Result<()> {
            let value_staked = self.env().transferred_value();
            total_staked += value_staked;
            mintStakingTokens(&self.env().AccountId, value_staked)
        }

        /// mint the staking tokens
        fn mintStakingTokens(&mut self, to: AccountId, amount: Balance) -> Result<()> {
            total_supply += amount;
            self.env().transfer(to, amount).ok_or("Error transfering")?
        }


    }

    
}

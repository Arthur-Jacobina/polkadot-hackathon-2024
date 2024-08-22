#![cfg_attr(not(feature = "std"), no_std)]
        
#[openbrush::implementation(PSP22, PSP22Metadata)]
#[openbrush::contract]
pub mod staking_token {
    use openbrush::{
        contracts::psp22::extensions::metadata::*,
        traits::{self, Storage},
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakingTokenContract {
    	#[storage_field]
		psp22: psp22::Data,
		#[storage_field]
		metadata: metadata::Data,
    }
    
    impl StakingTokenContract {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut _instance = Self::default();
			psp22::Internal::_mint_to(&mut _instance, Self::env().caller(), initial_supply).expect("Should mint"); 
			_instance.metadata.name.set(&name);
			_instance.metadata.symbol.set(&symbol);
			_instance.metadata.decimals.set(&decimal);
			_instance
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use openbrush::test_utils::*;

        const INITIAL_SUPPLY: u128 = 1_000_000_000 * 10u128.pow(18);

        #[ink::test]
        fn constructor_sets_name_symbol_and_decimals() {
            let name = Some(traits::String::from("Liquid Staking Token"));
            let symbol = Some(traits::String::from("LST"));
            let instance = StakingTokenContract::new(INITIAL_SUPPLY, name.clone(), symbol.clone(), 18);
            assert_eq!(psp22::extensions::metadata::PSP22MetadataImpl::token_name(&instance), name);
            assert_eq!(psp22::extensions::metadata::PSP22MetadataImpl::token_symbol(&instance), symbol);
            assert_eq!(psp22::extensions::metadata::PSP22MetadataImpl::token_decimals(&instance), 18);
        }

        #[ink::test]
        fn constructor_distributes_tokens_correctly() {
            let name = Some(traits::String::from("My Staking Token"));
            let symbol = Some(traits::String::from("MST"));
            let instance =
                StakingTokenContract::new(INITIAL_SUPPLY, name.clone(), symbol.clone(), 18);
            let owner = accounts().alice;

            assert_eq!(psp22::PSP22::total_supply(&instance), INITIAL_SUPPLY);
            assert_eq!(psp22::PSP22::balance_of(&instance, owner), INITIAL_SUPPLY);
        }
    }
}
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
            let name = Some(traits::String::from("Liquid Staking Token"));
            let symbol = Some(traits::String::from("LST"));
            let instance =
                StakingTokenContract::new(INITIAL_SUPPLY, name.clone(), symbol.clone(), 18);
            let owner = accounts().alice;

            assert_eq!(psp22::PSP22::total_supply(&instance), INITIAL_SUPPLY);
            assert_eq!(psp22::PSP22::balance_of(&instance, owner), INITIAL_SUPPLY);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {

        use super::*;
        use ink_e2e::build_message;
        use openbrush::contracts::psp22::{
            extensions::metadata::psp22metadata_external::PSP22Metadata, psp22_external::PSP22,
        };

        const INITIAL_SUPPLY: u128 = 1_000_000_000 * 10u128.pow(18);

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// Uncomment in the future
    //     #[ink_e2e::test]
    //     async fn instantiation_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         let constructor = StakingTokenContractRef::new(
    //             INITIAL_SUPPLY,
    //             Some(traits::String::from("Liquid Staking Token")),
    //             Some(traits::String::from("LST")),
    //             18,
    //         );

    //         let contract_account_id = client
    //             .instantiate("staking_token", &ink_e2e::alice(), constructor, 0, None)
    //             .await
    //             .expect("instantiate failed")
    //             .account_id;

    //         /// Check Token Name
    //         let token_name = build_message::<StakingTokenContractRef>(contract_account_id.clone())
    //             .call(|token| token.token_name());
    //         assert_eq!(
    //             client
    //                 .call_dry_run(&ink_e2e::alice(), &token_name, 0, None)
    //                 .await
    //                 .return_value(),
    //             Some(traits::String::from("Liquid Staking Token"))
    //         );

    //         // Check Token Symbol
    //         let token_symbol =
    //             build_message::<StakingTokenContractRef>(contract_account_id.clone())
    //                 .call(|token| token.token_symbol());
    //         assert_eq!(
    //             client
    //                 .call_dry_run(&ink_e2e::alice(), &token_symbol, 0, None)
    //                 .await
    //                 .return_value(),
    //             Some(traits::String::from("LST"))
    //         );

    //         // Check Token Decimals
    //         let token_decimals =
    //             build_message::<StakingTokenContractRef>(contract_account_id.clone())
    //                 .call(|token| token.token_decimals());
    //         assert_eq!(
    //             client
    //                 .call_dry_run(&ink_e2e::alice(), &token_decimals, 0, None)
    //                 .await
    //                 .return_value(),
    //             18
    //         );

    //         // Check Total Supply
    //         let total_supply =
    //             build_message::<StakingTokenContractRef>(contract_account_id.clone())
    //                 .call(|token| token.total_supply());
    //         assert_eq!(
    //             client
    //                 .call_dry_run(&ink_e2e::alice(), &total_supply, 0, None)
    //                 .await
    //                 .return_value(),
    //             INITIAL_SUPPLY
    //         );

    //         // Check Balance of Contract Owner (Alice)
    //         let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
    //         let alice_balance =
    //             build_message::<StakingTokenContractRef>(contract_account_id.clone())
    //                 .call(|token| token.balance_of(alice_account));
    //         assert_eq!(
    //             client
    //                 .call_dry_run(&ink_e2e::bob(), &alice_balance, 0, None)
    //                 .await
    //                 .return_value(),
    //             INITIAL_SUPPLY
    //         );

    //         Ok(())
    //     }
    // }
}
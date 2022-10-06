#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod my_psp34 {
    // imports from ink!
	use ink_prelude::string::String;
	use ink_storage::traits::SpreadAllocate;
	use openbrush::contracts::ownable::*;
    
    // imports from openbrush
	use openbrush::traits::Storage;
	use openbrush::contracts::psp34::extensions::burnable::*;
	use openbrush::contracts::psp34::extensions::mintable::*;
	use openbrush::contracts::psp34::extensions::enumerable::*;
	use openbrush::contracts::psp34::extensions::metadata::*;
    
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Hunter license {
    	#[storage_field]
		psp34: psp34::Data<Balances>,
		#[storage_field]
		ownable: ownable::Data,
		#[storage_field]
		metadata: metadata::Data,
    }
    
    // Section contains default implementation without any modifications
	impl PSP34 for Hunter license {}
	impl Ownable for Hunter license {}
	impl PSP34Burnable for Hunter license {
		#[ink(message)]
		#[openbrush::modifiers(only_owner)]
		fn burn(
            &mut self,
            account: AccountId,
			id: Id
        ) -> Result<(), PSP34Error> {
			self._burn_from(account, id)
		}
	}
	impl PSP34Mintable for Hunter license {
		#[ink(message)]
		#[openbrush::modifiers(only_owner)]
		fn mint(
            &mut self,
            account: AccountId,
			id: Id
        ) -> Result<(), PSP34Error> {
			self._mint_to(account, id)
		}
	}
	impl PSP34Enumerable for Hunter license {}
	impl PSP34Metadata for Hunter license {}
    
    impl Hunter license {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Contract|{
				_instance._init_with_owner(_instance.env().caller());
				_instance._mint_to(_instance.env().caller(), Id::U8(1)).expect("Can mint");
				let collection_id = _instance.collection_id();
				_instance._set_attribute(collection_id.clone(), String::from("name").into_bytes(), String::from("MyPSP34").into_bytes());
				_instance._set_attribute(collection_id, String::from("symbol").into_bytes(), String::from("MPSP").into_bytes());
			})
        }
    }
}
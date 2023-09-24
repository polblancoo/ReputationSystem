#![cfg_attr(not(feature = "std"), no_std, no_main)]

  //pub use self::my_psp22::ContractRef; 
  pub use self::my_psp22::ContractRef;

#[openbrush::implementation(PSP22, PSP22Burnable,PSP22Metadata)]
#[openbrush::contract]
pub mod my_psp22 {
    //use openbrush::traits::Storage;
    use openbrush::{traits::Storage, contracts::psp22,  };


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
    	#[storage_field]
		psp22: psp22::Data,
		#[storage_field]
		metadata: metadata::Data,
    }
    
    impl Contract {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut _instance = Self::default();

            
			      psp22::Internal::_mint_to(&mut _instance, Self::env().caller(), initial_supply).expect("Should mint"); 
                      _instance.metadata.name.set(&name);
                      _instance.metadata.symbol.set(&symbol);
                      _instance.metadata.decimals.set(&decimal);
                      _instance
                        }
        //Total de tokens prices
        #[ink(message)] 
         pub fn total_supply_prices(&self, contract: AccountId) -> Balance {
                 
         psp22::InternalImpl::_balance_of (self, &contract)
         
         
        }
        // transferencia
        #[ink(message)] 
        pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance, _data: Vec<u8>, ) -> Result<(), PSP22Error>{
          psp22::InternalImpl::_approve_from_to(self, from, to, amount)?;
          psp22::InternalImpl::_transfer_from_to(self, from, to, amount, _data, )?;

       Ok(()) 
       }
       // balance
       #[ink(message)]
       pub fn _balance_of(&self, owner: AccountId) -> Balance {
          psp22::InternalImpl::_balance_of(self, &owner)

       }
    }


   
}

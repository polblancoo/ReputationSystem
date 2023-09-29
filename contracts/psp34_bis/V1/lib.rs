#![cfg_attr(not(feature = "std"), no_std, no_main)]

//pub use crate::psp34_bis::ContractRef;

pub use self::psp34_bis_2::ContractbisRef;



//sE AGREGA EL TRAIT Upgradable
#[openbrush::implementation(PSP34, PSP34Mintable,Upgradeable, Ownable )]
#[openbrush::contract]
pub mod psp34_bis_2 {
    use openbrush::{traits::Storage,
                    
                    contracts::psp34,
             
     };
    
     use ink::storage::{
        traits::ManualKey,
        Lazy,
    };

    

    #[ink(storage)]
    #[derive(Default, Storage, )]
    pub struct Contractbis {
    	#[storage_field]
		psp34: psp34::Data,
        //owner para upgrade del contrato
        #[storage_field]
        ownable: ownable::Data,
        
        #[storage_field]
		next_id: u8,
    }

     // Implementa el trait psp34_lop_organization para el contrato Votantes
    impl Contractbis {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			psp34::Internal::_mint_to(&mut _instance, Self::env().caller(), Id::U8(1)).expect("Can mint");
			ownable::Internal::_init_with_owner(&mut _instance, Self::env().caller());
            _instance
        }

        ///Busca el ultimo nft minteado y suma uno para crear uno nuevo
        #[ink(message)]
        pub fn mint_to(&mut self, to: AccountId) -> Result<(), PSP34Error> {
            PSP34Mintable::mint(self, to, Id::U8(self.next_id+2))?;
            self.next_id += 2;
            Ok(())
        }
		
         ///total minteados
         #[ink(message)]
         pub fn total_nft(&self) -> Result<u8, PSP34Error> {
                          
             Ok(self.next_id)
         }
        
    }
     

    
}

   
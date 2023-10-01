#![cfg_attr(not(feature = "std"), no_std, no_main)]

//pub use crate::psp34_bis::ContractRef;

pub use self::psp34_bis_2::ContractbisRef;



//sE AGREGA EL TRAIT Upgradable y el de acces control para Burn
#[openbrush::implementation(PSP34, PSP34Mintable, PSP34Burnable,Ownable,Upgradeable, AccessControl )]
#[openbrush::contract]
pub mod psp34_bis_2 {
    use openbrush::{traits::Storage,
                    //modif. para acceso control
                    modifiers,
                    contracts::psp34,
             
     };
    
     use ink::storage::{
        traits::ManualKey,
        Lazy,
    };

     // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const BURN: RoleType = ink::selector_id!("BURN");
    
    #[default_impl(PSP34Burnable)]
    #[modifiers(only_role(BURN))]
    fn burn() {}

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
        #[storage_field]
        access: access_control::Data,
    }

     // Implementa el trait psp34_lop_organization para el contrato Votantes
    impl Contractbis {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			psp34::Internal::_mint_to(&mut _instance, Self::env().caller(), Id::U8(1)).expect("Can mint");
			ownable::Internal::_init_with_owner(&mut _instance, Self::env().caller());
             // We grant minter role to caller in constructor, so he can mint/burn tokens
             AccessControl::grant_role(&mut _instance, BURN, Some(Self::env().caller())).expect("Should grant MINTER role");
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

   
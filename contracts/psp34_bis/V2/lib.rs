#![cfg_attr(not(feature = "std"), no_std, no_main)]

//pub use crate::psp34_bis::ContractRef;

pub use self::psp34_bis_2::ContractbisRef;




#[openbrush::implementation(PSP34, PSP34Mintable, PSP34Burnable,Ownable,Upgradeable, AccessControl  )]
#[openbrush::contract]
pub mod psp34_bis_2 {
  
    
     use ink::storage::{
        traits::ManualKey,
        Lazy,
    };
    use openbrush::{traits::Storage,
        modifiers, 
        contracts::psp34,
 
};



     const STORAGE_KEY: u32 = openbrush::storage_unique_key!("contract_v2", "fee_extra");
    
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
       //Se agrega campo , siempre debajo del ultimo para que no apunte a un lugar 
       //equivodao en el Storage
        fee_extra: Lazy<AccountId, ManualKey<STORAGE_KEY>>,
         }

   // const MANAGER: RoleType = ink::selector_id!("MANAGER");
  //  const MINTER: RoleType = ink::selector_id!("MINTER");

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
       // #[ink(message)]
        //#[default_impl(PSP34Mintable)]
        //#[modifiers(only_role(MINTER))]
       // fn mint(&mut self) {}



    /*  #[ink(message)]
       // #[default_impl(PSP34Mintable)]
      //  #[modifiers(only_role(MANAGER))]
        //#[modifiers(only_owner)]
        pub fn set_code(&mut self, code_hash: Hash) {
            self.env().set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!("Failed to `set_code_hash` to {code_hash:?} due to {err:?}")
            });
            ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
        }
 */
        ///Busca el ultimo nft minteado y suma uno para crear uno nuevo
        //#[default_impl(PSP34Mintable)]
        //#[modifiers(only_role(MINTER))]
        #[ink(message)]
        pub fn mint_to(&mut self, to: AccountId) -> Result<(), PSP34Error> {
            PSP34Mintable::mint(self, to, Id::U8(self.next_id+2))?;
            self.next_id += 2;
            Ok(())
        }
		
         ///total minteados
         //#[overrider(psp34::Internal)]
         #[ink(message)]
         pub fn total_nft(&self) -> Result<u8, PSP34Error> {
                          
             Ok(self.next_id)
         }
        
        
       
        ///total minteados
       #[ink(message)]
       #[modifiers(only_owner)]
       pub fn set_fee_Extra(&mut self, account: AccountId) -> Result<(), OwnableError> {
           self.fee_extra.set(&account);
           Ok(())
       } 
        
    }
     

    
}

   
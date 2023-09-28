#![cfg_attr(not(feature = "std"), no_std, no_main)]
//Modulo de errores.
mod  errors;

#[ink::contract]
mod reputation_system {
    use ink::env::emit_event;
    //use ink::env::caller;
    use ink::prelude::vec::Vec;
    use ink::{
        env::{
            call::{build_call, build_create, ExecutionInput, Selector},
            DefaultEnvironment,
        },
        selector_bytes,
    };
    //use ink_prelude::Into;
    use ink::prelude::string::String;
    use ink::storage::Lazy;
    use ink::storage::Mapping;
    use crate::errors::Error;
    use psp34_bis::ContractbisRef;
    use my_psp22::ContractRef;

    //eventos
    #[ink(event)]
    pub struct NewVotingRound {
        #[ink(topic)]
        pub admin: AccountId,
        pub funds1: Balance,
        pub funds2: Balance,
        pub funds3: Balance,
        pub votes_per_member: i32,
        pub duration: u64,
    }

    #[ink(event)]
    pub struct EndVotingRound {
        #[ink(topic)]
        pub admin: AccountId,
    }

    
    // Defines the storage of your contract.
    
    #[ink(storage)]
    pub struct ReputationSystem {
        admin: AccountId,
        //ROnda de votacion , con los fondos , el time etc
        round: VotingRound,
        //Votos acumulados
        reputacion: Mapping<AccountId, i32>,
        //rol 1 admin 2 contributer
        roll: Mapping<AccountId, i32>,
        //1 puesto 2 puesto 3 puesto
        voters_win: Vec<(AccountId, i32)>,
        //NFT
        psp34_bis: ContractbisRef,
        my_psp22: ContractRef,
       
    }
    /// Un tipo personalizado que podemos usar en nuestro storage del contrato
    #[derive(Debug)]
    #[ink::storage_item]
    pub struct VotingRound {
        funds1: Balance,
        funds2: Balance,
        funds3: Balance,

        votes_per_member: i32,
        duration: u64,
        ended: bool,
       
    }
    impl ReputationSystem {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(admin: AccountId, psp34_contract_code_hash: Hash, my_psp22_contract_code_hash: Hash) -> Self {
           let init_Suply: Balance = 10_000_000_000_000_000_000_000_000_000_000_000; 
          
           let name: Option<String> = Some(String::from("loopring"));
          
           let symbol_name: Option <String> = Some(String::from("lop"));
           let decimal: u8= 2;
            Self {
                admin,
                round: VotingRound{
                    funds1: 0,
                    funds2: 0,
                    funds3: 0,
                    votes_per_member: 1,
                    duration: 0,
                    ended: true,
                   

                },

                reputacion: Mapping::new(),
                roll: Mapping::new(),
                voters_win:Default::default(),
                
                psp34_bis: ContractbisRef::new()
                    .code_hash(psp34_contract_code_hash)
                    .endowment(0)
                    .salt_bytes(Vec::new()) // Sequence of bytes
                    .instantiate(),  
                
                my_psp22: ContractRef::new(init_Suply,name,symbol_name,  decimal)
                    .code_hash(my_psp22_contract_code_hash)
                    .endowment(0)
                    .salt_bytes(Vec::new()) // Sequence of bytes
                    .instantiate(),  
                    

            }
        }
         // Función de utilidad para asegurarse de que el llamante sea el administrador
        fn ensure_admin(&self, account: AccountId) -> Result<(), Error> {
            if account == self.admin {
                Ok(())
            } else {
                Err(Error::NotAdmin)
            }
        }
         // Función de utilidad para asegurarse de que el llamante sea un contribuyente
        fn ensure_contributor(&self, account: AccountId) -> Result<(), Error> {
            if self.reputacion.contains(&account) {
                Ok(())
            } else {
                Err(Error::YouAreNotVoter)
            }
        }
         // Función para calcular el factor de aumento (booster) según el porcentaje de votos recibidos
        fn calculate_boost_factor(&self, caller: AccountId) -> i32 {
            //si la ronda termino, devuelve 0
            if self.round.ended == true {return 0};
            // Obtener la cantidad total de votos en la ronda actual
            let mut  total_votes = self.round. votes_per_member;
            if total_votes == 0 {total_votes = 1i32}
            // Obtener la cantidad de votos recibidos por el votante
            let votes_received = self.reputacion.get(caller).unwrap_or(0);
            // Calcular el porcentaje de votos recibidos
            let vote_percentage = (votes_received * 100) / total_votes;

            // Aplicar aumento según el porcentaje de votos recibidos
            if vote_percentage > 75 {
                3 // Aumento por 3
            } else if vote_percentage > 50 {
                2 // Aumento por 2
            } else {
                1 // Aumento por 1 (sin boost)
            }
        }
        // Función para determinar los ganadores (1er, 2do y 3er puesto)
        fn Winers_position(&mut self, voter: AccountId, votes: i32) {
            let mut winners = &mut self.voters_win;

            // Buscar si el votante ya está en la lista de ganadores
            if let Some(position) = winners.iter().position(|(account, _)| *account == voter) {
                // Actualizar votos si el votante ya estaba en la lista
                if votes > winners[position].1 {
                    winners[position] = (voter, votes);
                }
            } else {
                // Si el votante no estaba en la lista, lo agregamos
                winners.push((voter, votes));
            }

            // Ordenar la lista de ganadores en orden descendente según los votos
            winners.sort_by(|a, b| b.1.cmp(&a.1));

            // Mantener solo a los tres primeros ganadores
            if winners.len() > 3 {
                winners.truncate(3);
            }
        }
       
       fn position_price(&self , position: i8)->Balance {
                // Obtener el token
            //let token = my_psp22::PSP22Impl::get_token();

            // Obtener el número de decimales
           /*  let decimals = 2;
           
                 match position {
                    1 =>   self.round.funds1 / 10u128.pow(decimals as u32), // Primera posición
                     // Primera posición
                    2 =>  self.round.funds1 / 10u128.pow(decimals as u32), // seg posición
                   
                    // Segunda posición
                    3 =>  self.round.funds1 / 10u128.pow(decimals as u32), // tercera posición

                    
                    _ =>  0,
                } 
                */
                 match position {
                    1 =>   self.round.funds1,
                     // Primera posición
                    2 =>  self.round.funds2,
                    // Segunda posición
                    3 =>   self.round.funds3,
                    
                    _ =>  0,
                }     

       }
        //Reparte premios dinero y nft a los ganadores
        fn Winners_prices(&mut self , caller: AccountId , position: i8)->Result<(), Error> {
            
           //direccion contrato
            let contract_account_id: AccountId =  AccountId::from(Self::env().account_id());
            //PREMIO
            let amount =self.position_price( position);
            if  amount <= 0 {Err(Error::NotPricesFunds)}else{;
                //let mut  data: Vec<u8> =  ink::prelude::vec::Vec::new();
                let mut data: ink::prelude::vec::Vec<u8> = ink::prelude::vec![];
                // NFT Y PREMIO
           
                if let Err(err) =psp34_bis::ContractbisRef::mint_to(&mut self.psp34_bis , caller){
                    return Err( Error::NftNotSent);
                }
                if self.my_psp22._balance_of(self.env().account_id()) < amount {
                    return Err(Error::NotTransferredBalance);
                }
                
                if let Err(err)=   self.my_psp22.transfer(contract_account_id, caller, amount , data){
                    return Err( Error::TransferError2);
                }
            /* 
                if let Err(err) = self.my_psp22.transfer( contract_account_id, caller, amount , data){
                    return Err( Error::TransferError2);
                }
             */
                Ok(())
            } 
            
        }
        //tranforma en numero decimal Balance
        pub fn r_u64_to_Balance(&self , num : u64)->Balance{
           // Balance::from(num) / (10u128.pow(2))
            0
          }



        //deposita en el contrato el premio monetario.
        
        pub fn Deposit_found(&mut self, amount1: u64 , amount2: u64 , amount3: u64 , data : Vec<u8> )->Result<(), Error> {
           
            //Balance = Balance::from(amount1) / (10u128.pow(2))
            //let amount1=   self.r_u64_to_Balance(amount1);//  r_u64_to_Balance(amount1);
           // let amount2: Balance = self.r_u64_to_Balance(amount2);
            //let amount3: Balance = self.r_u64_to_Balance(amount3);
          let amount1: Balance = amount1 as Balance;
          let amount2: Balance = amount2 as Balance;
          let amount3: Balance = amount3 as Balance;

          //  let caller: AccountId = self.env().caller();
           //let contract_account_id: AccountId =  AccountId::from(Self::env().account_id());
            // Obtener el balance transferido con la transacción
            //let amount_total_to_contract = amount1 + amount2 +amount3;
            //let a = my_psp22::ContractRef::transfer(&mut self.my_psp22, caller ,contract_account_id,amount_total_to_contract , data.clone() );
        
        /*Modificacion 27/09/2023
           match  my_psp22::ContractRef::transfer(&mut self.my_psp22, caller ,contract_account_id,amount_total_to_contract , data.clone() )
            { 
                Ok(_) => {
                    // La transferencia fue exitosa, puedes hacer algo si es necesario
                }
                Err(err) => {
                    // Hubo un error en la transferencia, emite un error o realiza otras acciones según sea necesario
                    // Aquí puedes emitir un evento de error si tienes definido uno en tu contrato
                    // self.env().emit_event(ErrorEvent { message: "Transfer error".to_string() });
                    // También puedes devolver un error personalizado si es apropiado
                    return Err(Error::TransferError);
                }

            } 
        */
            //let a = my_psp22::ContractRef::transfer(&mut self.my_psp22, caller ,contract_account_id,amount2 , data.clone() );
            //let a = my_psp22::ContractRef::transfer(&mut self.my_psp22, caller ,contract_account_id,amount3 , data.clone() );
           
            // Verificar que el balance transferido sea mayor que cero
            if amount1  > 0 && amount2  > 0 && amount3  > 0 {
                // Incrementar el balance del contrato
                self.round.funds1 += amount1;
                self.round.funds2 += amount2;
                self.round.funds3 += amount3;

                Ok(())
            } else {
                // Si no se transfirió ningún balance, devolver un error
                Err(Error::NotTransferredBalance)
            }
            
            
        } 
        // Función para obtener al ganador en una posición específica (1, 2 o 3)
        #[ink(message)]
        pub fn winner_for_position(&self, position: u32) -> Result<AccountId, Error> {
            
            let winners = &self.voters_win;

            // Verificar que haya al menos tres ganadores
            if winners.len() < 3 {
                return Err(Error::FaltanGanadores);
            }
        
            // Verificar que la posición sea válida (1, 2 o 3)
            if position >= 1 && position <= 3 {
                // Obtener al ganador en la posición especificada
                if let Some(winner) = winners.get(position as usize - 1) {
                    return Ok(winner.0);
                }
            }
        
            Err(Error::PosicionNoValida)// Si la posición no es válida o no hay ganador en esa posición
        }

        // Función para cargar un nuevo contribuyente (solo el admin puede ejecutarla)
        #[ink(message)]
        pub fn join_contributor(&mut self, contributor: AccountId) -> Result<(), Error> {
            let caller = self.env().caller();
            self.ensure_admin(caller)?;

            // Verificar que el contribuyente no esté ya registrado
            if self.reputacion.contains(&contributor) {
                return Err(Error::YouAreVoter);
            }

            // Agregar el nuevo contribuyente con una reputación inicial de 1
            self.reputacion.insert(&contributor, &1i32);
            //Agrega al contribuyente con roll 0/contributor 1/Admin
            self.roll.insert(&contributor , &0i32);

            Ok(())
        }
       // Función para obtener los puntos totales de un miembro
        #[ink(message)]
        pub fn get_total_reputation_by_account(&self, account: AccountId) -> i32 {
            if self.reputacion.contains(&account) {
                self.reputacion.get(&account).unwrap()
            } else {
                0 // Valor predeterminado si la cuenta no tiene puntos de reputación asignados
            }
        }
        // Función para obtener los puntos totales de un miembro sin tener en cuenta votes_per_member
        #[ink(message)]
        pub fn get_total_reputation(&self, account: AccountId) -> i32 {
             // Obtener la VotingRound actual
            let round = &self.round;

            // Verificar si la VotingRound está en curso
            if round.ended==false {
                // Obtener el total acumulado de puntos de reputación del miembro
                let total_reputation = round.votes_per_member;

                // Devolver el total acumulado de puntos de reputación
                return total_reputation;
            }

            // Devolver 0 si la VotingRound ha terminado
            0
        }
        // Función para obtener los fondos depositados en el premio
        #[ink(message)]
        pub fn get_prize_funds(&self, position: i8) -> Balance {
            
            match position {
                1 => self.round.funds1, // Primera posición
                2 => self.round.funds2, // Segunda posición
                3 => self.round.funds3, // Tercera posición
                _ =>  0,             // Otras posiciones, balance cero por defecto
            }       
        } 
          
        #[ink(message)]
        pub fn get_funds_total_in_contribiuter(&self) -> Balance {
            //self.round.funds
            let id = Self::env().caller();
            let a = my_psp22::ContractRef::_balance_of(&self.my_psp22, id,);
            a
           // self.balance;
        } 
        //fondos totales en el contrato
        #[ink(message)]
        pub fn get_funds_total_in_contract(&self) -> Balance {
            //self.round.funds
            let id = self.env().account_id();
           // let a = my_psp22::ContractRef::_balance_of(&self.my_psp22, id,);
           let a = self.my_psp22._balance_of(self.env().account_id());
            a
            
           // self.balance;
        }    

       

        // Función para iniciar una nueva ronda de votación
        #[ink(message)]
        pub fn start_voting_round(&mut self, funds1: Balance,funds2: Balance,funds3: Balance,  duration: u64) -> Result<(), Error> {
            let caller = self.env().caller();
            self.ensure_admin(caller)?;//fondos totales en el contrato
          

            // Verificar que no haya otra ronda en curso
            if self.round.ended == false {
                return Err(Error::RoundInCourse);
            }
            //deposita fondos en el contrato
            //todo!();
            let  data: Vec<u8> = ink::prelude::vec![1, 2, 3];
             self.Deposit_found(funds1 as u64, funds2 as u64, funds3 as u64,data )?;
            //transforma a Balance los depositos
           // let mut funds1: Balance = self.r_u64_to_Balance(funds1);
           // let mut funds2: Balance = self.r_u64_to_Balance(funds2);
            //let mut funds3: Balance = self.r_u64_to_Balance(funds3);

            // Crear una nueva ronda de votación
            self.round = VotingRound{
                funds1,
                funds2,
                funds3,
                votes_per_member: 0,
                duration,
                ended: false,
                //voters: Vec::new(),
            };
            //limpio los miembros voters_win y reputation
            self.voters_win= Default::default();
            self.reputacion= Mapping::new();

            // Emitir el evento NewVotingRound
            self.env().emit_event(NewVotingRound {
                admin: caller,
                funds1,
                funds2,
                funds3,
                votes_per_member:0,
                duration,
            });

            Ok(())
        }
        
        // Función para finalizar la ronda de votación
        #[ink(message)]
        pub fn end_voting_round(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            self.ensure_admin(caller)?;

            // Verificar que haya una ronda en curso
            if self.round.ended == true {
                return Err(Error::RoundFinish);
            }
            //let result  = self.winner_for_position(1);
           let caller1: AccountId = self.winner_for_position(1)?;
           let caller2: AccountId = self.winner_for_position(2)?;
           let caller3: AccountId = self.winner_for_position(3)?;
           // -Realizar la distribución de fondos
            //  faltaria...
            //  que un timer ejecute el reparto automaticamente.
            
           // self.Winners_prices( caller1 , 1i8)?;
           // self.Winners_prices( caller2 , 2i8)?;
           // self.Winners_prices( caller3 , 3i8)?;
            
            // -Emitir NFT;
            let _nft=self.Winners_prices(caller1 , 1)?;
            let _nft=self.Winners_prices(caller2 , 2)?;
            let _nft=self.Winners_prices(caller3 , 3)?;
            // Finalizar la ronda
            self.round = VotingRound{
                funds1: 0,
                funds2: 0,
                funds3: 0,
                votes_per_member :0,
                duration: 0,
                ended: true,
            
            };
            // -Emitir el evento EndVotingRound
            self.env().emit_event(EndVotingRound { admin: caller });

            Ok(())
            
        }

        // Función para realizar una votación
        #[ink(message)]
        pub fn vote(&mut self, target: AccountId) -> Result<(), Error> {
           let value = 1;
            let caller = self.env().caller();
            self.ensure_contributor(caller)?;
            self.ensure_contributor(target)?;
            // Verificar que el votante no esté votando por sí mismo
            if caller == target {
                return Err(Error::CannotVoteItself);
            }

            // Realizar la votación y calcular el valor de reputación actualizado
            let boost_factor = self.calculate_boost_factor(Self::env().caller());
            let target_reputation = self.reputacion.get(&target).unwrap_or(1);
            let new_reputation = target_reputation + value * boost_factor;

            // Asegurarse de que el valor de reputación no sea menor que 1
            let new_reputation = i32::max(new_reputation, 1);

            // Actualizar la reputación del objetivo
            self.reputacion.insert(target, &new_reputation);
            //Acturaliza total votos
            self.round.votes_per_member = self.round.votes_per_member+ new_reputation;

            //verifica la tabla de posiciones
            self.Winers_position(target , new_reputation );

            Ok(())
        }   
    
    }//fin Impl

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn test_start_voting_round() {
            // Opción 1: Usar valores fijos para los parámetros del constructor
            let admin = AccountId::new([1; 32]); // Reemplaza con una cuenta de ejemplo
            let psp34_contract_code_hash = Hash::new([2; 32]); // Reemplaza con un hash de contrato
            let my_psp22_contract_code_hash = Hash::new([3; 32]); // Reemplaza con otro hash de contrato
    
            // Crea una instancia de tu contrato usando los valores fijos
            let mut contract = ReputationSystem::new(admin, psp34_contract_code_hash, my_psp22_contract_code_hash);
    
            // Define los valores de prueba para iniciar la ronda
            let funds1 = 100; // Define un valor de fondos
            let funds2 = 200; // Define otro valor de fondos
            let funds3 = 300; // Define otro valor de fondos
            let duration = 7; // Define la duración de la ronda
    
            // Llama a la función start_voting_round con los valores de prueba
            let result = contract.start_voting_round(funds1, funds2, funds3, duration);
    
            // Verifica que la función haya tenido éxito
            assert_eq!(result, Ok(()));
    
            // Opción 2: Despliega contratos reales en una cadena de prueba y utiliza sus direcciones
            // ...
        }
        #[ink::test]
        fn test_winners_position() {
            // Configura el contexto de prueba
            let mut contract = ReputationSystem::default(); // Crea una instancia de tu contrato
    
            // Definimos algunos valores de ejemplo
            let voter1 = AccountId::new([1; 32]);
            let voter2 = AccountId::new([2; 32]);
            let voter3 = AccountId::new([3; 32]);
            let votes1 = 10;
            let votes2 = 5;
            let votes3 = 8;
    
            // Llamamos a la función con los valores de ejemplo
            contract.Winers_position(voter1, votes1);
            contract.Winers_position(voter2, votes2);
            contract.Winers_position(voter3, votes3);
    
            // Verificamos que los ganadores se hayan registrado correctamente
            let winners = &contract.voters_win;
            assert_eq!(winners.len(), 3);
    
            // Verificamos que los ganadores estén en el orden correcto según los votos
            assert_eq!(winners[0].0, voter1);
            assert_eq!(winners[1].0, voter3);
            assert_eq!(winners[2].0, voter2);
    
            // Verificamos que los votos también sean correctos
            assert_eq!(winners[0].1, votes1);
            assert_eq!(winners[1].1, votes3);
            assert_eq!(winners[2].1, votes2);
        }
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = FlipperRef::default();

            // When
            let contract_account_id = client
                .instantiate("Flipper", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<FlipperRef>(contract_account_id.clone())
                .call(|Flipper| Flipper.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = FlipperRef::new(false);
            let contract_account_id = client
                .instantiate("Flipper", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<FlipperRef>(contract_account_id.clone())
                .call(|Flipper| Flipper.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<FlipperRef>(contract_account_id.clone())
                .call(|Flipper| Flipper.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<FlipperRef>(contract_account_id.clone())
                .call(|Flipper| Flipper.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
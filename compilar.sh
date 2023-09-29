#!/usr/bin/env bash
#replica comandos en consola al ejecutarse
set -eu
echo -e "\e[1;34m Esta es una Guia\e[0m: paso a paso , de como ir agregando funcionalidades a tu proyecto "
echo -e "\e[1;34m PSP22\e[0m (TOKEN fungible) , te demuestra rapidamente como habilitar a travez de la 
  biblioteca de  https://openbrush.brushfam.io/ una funcionalidad ya lista y personalizable ."
echo -e "\e[1;31m *********************\e[0m"
echo -e " \e[1;34m PSP34\e[0m ahora jugamos con un  toquen Idem-NFT , el cual vamos a hacerle un upgrade al mismo "
echo -e " compilaremos la V1 y la V2 , y en cadena tomando  el HASH de la V2 , en la Funcion "
echo -e "(upgradeable::SetCodeHash) que nos entrega https://openbrush.brushfam.io/ podremos hacerla funcionar"
echo "Nota : [ink(storage)]           ...   "
echo "    [derive(Default, Storage, )]...   "
echo "    pub struct Contractbis {    ...   "
echo "                      Campo 1   ...   "
echo "                      Campo 2   ...   "
echo "                     Nuevo Campo...   "

echo "    }     ... "
echo "Siempre respetar el orden en que estan escritos y si se agregan ponerlo a lo ultimo  "
echo "Luego de ejecutar la Fn , cargar la metadata (xxx.jason ) del contrato nuevo asi la "
echo "UI podra reconocer las fn y datos nuevos del contrato."
echo -e "\e[1;31m*********************\e[0m"
echo -e "\e[1;34m ReputationSystem \e[0m expone como llamar a otro contrato(psp22 y psp34) "
echo -e "\e[1;31m*********************\e[0m"
echo -e "\e[1;34m Que compilamos..?\e[0m"
echo "1 - psp22"
echo "2 - psp34"
echo "3 - Upgrade - psp34"
echo "4 - reputationSystem"
echo "5 - Todos"
echo -e "\e[1;31m*********************\e[0m"
read accion;

case $accion in
  1)  cargo +stable contract build --manifest-path contracts/psp22/Cargo.toml;;
  2)  cargo +stable contract build --manifest-path contracts/psp34_bis/V1/Cargo.toml;;
  3)  cargo +stable contract build --manifest-path contracts/psp34_bis/V2/Cargo.toml;;
  4)  cargo +stable contract build --manifest-path contracts/reputationSystem/Cargo.toml --skip-wasm-validation;;
  5)  cargo +stable contract build --manifest-path contracts/reputationSystem/Cargo.toml
      cargo +stable contract build --manifest-path contracts/psp34_bis/V1/Cargo.toml
	    cargo +stable contract build --manifest-path contracts/psp22/Cargo.toml;;
 
  *) echo "No es una opcion.";;
esac

##cargo  contract build

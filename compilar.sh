#!/usr/bin/env bash
#replica comandos en consola al ejecutarse
set -eu
echo "*********************"
echo "Que compilamos..?"
echo "1 - psp22"
echo "2 - psp34"
echo "3 - reputationSystem"
echo "4 - Todos"
echo "*********************"
read accion;

case $accion in
  1)  cargo +stable contract build --manifest-path contracts/psp22/Cargo.toml;;
  2)  cargo +stable contract build --manifest-path contracts/psp34_bis/Cargo.toml;;
  3)  cargo +stable contract build --manifest-path contracts/reputationSystem/Cargo.toml;;
  4)  cargo +stable contract build --manifest-path contracts/reputationSystem/Cargo.toml
	    cargo +stable contract build --manifest-path contracts/psp34_bis/Cargo.toml
	    cargo +stable contract build --manifest-path contracts/psp22/Cargo.toml;;
 
  *) echo "No es una opcion.";; 
esac

##cargo  contract build

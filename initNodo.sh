File Edit Options Buffers Tools Sh-Script Help                                            
#!/usr/bin/env bash                                                                       
#replica comandos en consola al ejecutarse                                                
set -eu                                                                                   
#enviroment                                                                               
#Poner la ruta a su nodo , SWanky-node , substarte-node , u otro                          
export CONTRACTS_NODE="/home/pablo/Rust-Curso/swanky-node"                                
#/home/pablo/Rust-Curso/swanky-node --dev                                                 
                                                                                          
# Comprueba si la variable de entorno CONTRACTS_NODE está establecida                     
if [ -z "${CONTRACTS_NODE+x}" ]; then                                                     
  # Si no está establecida, solicita al usuario que la ingrese                            
  echo "Por favor, ingrese la ruta a su nodo:"                                            
  read CONTRACTS_NODE                                                                     
fi                                                                                        
                                                                                          
# Exporta la variable de entorno                                                          
export CONTRACTS_NODE                                                                     
                                                                                          
# Ejecuta el nodo                                                                         
$CONTRACTS_NODE --dev 

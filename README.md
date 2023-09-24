
<hr style="color: 30056b2;"/>

<h2 align="center">🦑 ink-examples Solución</h2>
<strong># Contrato Reputation System</strong>

El contrato Reputation System permite administrar un sistema de reputación y votación, donde los participantes pueden votar por otros miembros y ganar premios en función de sus votos y posición en la votación.

<hr style="color: 30056b2;"/>
<strong># Funcionalidades clave</strong>

El contrato<strong> Reputation System</strong> incluye las siguientes funcionalidades:

1. Registro de contribuyentes: Los administradores pueden registrar nuevos contribuyentes en el sistema.

2. Votación: Los miembros pueden votar por otros miembros y ganar reputación <br>
, se modifico la consigna inicial para que el voto sea positivo y tenga un booster<br>
determinado en funcion de: <strong> % de votos obtenidos / total votos </strong>.

3. Inicio y finalización de rondas de votación: Los administradores pueden iniciar y finalizar rondas de votación<br>
depositando en el contrato los tres premios a repartir.Lo ideal seria insertar un Timer a futuro.
   

5. Premios: Se distribuyen premios monetarios y NFTs a los tres mejores clasificados en cada ronda de votación.

6. Consulta de reputación: Los miembros pueden consultar su reputación acumulada y su posición en la tabla de clasificación.

7. Consulta de fondos del premio: Los miembros pueden consultar los fondos acumulados en el premio de la ronda actual
   
## Funciones claves:

1. Inicio de una nueva ronda de votación
 ``` rust
      let admin = ... // ID del administrador<br>
      let funds1 = ... // Fondos para el primer puesto
      let funds2 = ... // Fondos para el segundo puesto
      let funds3 = ... // Fondos para el tercer puesto
      let duration = ... // Duración de la ronda en segundos
      contract.start_voting_round(funds1, funds2, funds3, duration);
```

2.Votación por un miembro

 ``` rust
    let voter = ... // ID del miembro que vota
    let target = ... // ID del miembro por el que se vota
    contract.vote(target, {value: 1});
 ```
3. Finalización de una ronda de votación
 ``` rust
    let admin = ... // ID del administrador
    contract.end_voting_round();
 ```
2.1.Consulta de la reputación total de un miembro
 ``` rust
    let member = ... // ID del miembro
    let reputation = contract.get_total_reputation_by_account(member);
 ```
2.2.Consulta de los fondos del premio en una posición
 ``` rust
       let position = ... // Posición en la ronda (1, 2 o 3)
    let prize_funds = contract.get_prize_funds(position);
 ```
2.3. Devuelve AccountId de posicion ganadora
 ``` rust
       let position = ... // Posición en la ronda (1, 2 o 3)
      let winner_for_position(&self, position: u32)-> Result<AccountId, Error>
```
2.4. Devuelve AccountId del Contrato
 ``` rust
      get_contract_account_id(&self) -> AccountId
```
<hr style="color: 30056b2;"/>
<h1>Requisitos:</h1>
  <A HREF="https://use.ink/getting-started/setup"> Ink! setup </A></br>
   <A HREF="https://github.com/paritytech/cargo-contract"> Cargo-Contract </A></br>
  <A HREF="https://github.com/paritytech/substrate-contracts-node/releases">Descargar substrate-contracts-node-linux.tar.gz </A></br>
 <A HREF="https://github.com/swankyhub/swanky-node">Descargar swanky-node -nodo alternativo </A></br>
  
Colocar substrate-contracts-node en una carpeta del PATH del sistema</br>
<strong>Nota:</strong> Al correr el nodo , setting la variable de entorno CONTRACT_NODE="path/del nodo"</br>

<h1>Software utilizado:</h1>
Software	Versión

rustup	1.26.0 (5af9b9484 2023-04-05)</br>
rustc	1.72.0 (5680fa18f 2023-08-23)</br>
cargo	1.72.0 (103a7ff2e 2023-08-15)</br>
cargo-contract	3.2.0-unknown-x86_64-unknown-linux-gnu</br>
substrate-contracts-node o swanky </br>

hr style="color: 30056b2;"/>

<h1>Contribución</h1>
Si deseas contribuir a este proyecto, ¡no dudes en hacerlo! Puedes enviar problemas (issues) o solicitudes de extracción (pull requests) en el repositorio.
<h1>Licencia</h1>
Este proyecto está bajo la licencia MIT. 
![Licencia MIT]((https://github.com/polblancoo/ReputationSystem/blob/main/MIT.svg)https://github.com/polblancoo/ReputationSystem/blob/main/MIT.svg)

# ReputationSystem
## Trabajo Práctico - Enunciado final

El objetivo del trabajo práctico es crear una **plataforma de gestión de reputación** según las contribuciones realizadas a una organización.

### Reglas

#### Organización
- Una **organización** tiene miembros.
- Los miembros tienen roles: **Admin** o **Contributor**.
- Los **contributors** participan haciendo **aportes off-chain**. 
- Los aportes off-chain se **valorizarán** mediante votos on-chain entre contributors.
- La organización, **mediante su Admin**, abrirá **rondas de votación con una duración determinada que podrá variar entre las diferentes rondas**.
- Al momento de crear la ronda de votación, el Admin deberá indicar:
  - **el monto de fondos a repartir** entre los contributors.
  - **la cantidad de votos** que podrá efectuar cada uno de ellos.
- **Los fondos deberán ser cargados por el Admin.**

#### Votación
- Los contributors podrán votar de forma **positiva** o **negativa** a otros contributors. 
- Estos votos **impactarán en el valor de reputación** del contributor votado.
- **El valor de reputación de un contributor nunca podrá ser menor a 1**.
- El poder de voto de cada contributor será **proporcional a su valor de reputación**. *La fórmula quedará a criterio de cada uno*.
  - Ejemplo: f(member_pts, target_pts, value) = target_pts + value * sqrroot(member_pts)
  - Value = +PTS o -PTS

#### Premiación
- Al finalizar la ronda de votación, **los fondos se repartirán entre los contributors en base a su valor de reputación** a partir de una transacción ejecutada por el Admin.
- Luego de que se repartan los fondos, **los valores de reputación se resetearán**.
- Se **entregarán badges (NFTs)** a los 3 contributors con mayor valor de reputación (Gold, Silver and Bronze).


### Entregable

- Se deberá presentar un repositorio de código con los contratos.
- El README del repo deberá contener la explicación de la solución.
- ##################################################################################################
<h2 align="center">🦑 ink-examples Solución</h2>
# Contrato Reputation System

El contrato Reputation System permite administrar un sistema de reputación y votación, donde los participantes pueden votar por otros miembros y ganar premios en función de sus votos y posición en la votación.

## Funcionalidades clave

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
  
      let admin = ... // ID del administrador<br>
      <div class=”color”>Este es un div en una página web.</div>
      let funds1 = ... // Fondos para el primer puesto
      let funds2 = ... // Fondos para el segundo puesto
      let funds3 = ... // Fondos para el tercer puesto
      let duration = ... // Duración de la ronda en segundos
      contract.start_voting_round(funds1, funds2, funds3, duration);


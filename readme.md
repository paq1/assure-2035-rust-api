# Assure 2035 API

# 1 - Mes choix technique:

## 1-1 architectures

### 1-1-1 architechture applicative hexagonal

Je suis parti sur un architecture hexagonal + event sourcing. Ces choix ont étaient fait dans le but d'avoir une api
scalable et maintenable en mettant en avant une séparation infra/domain où quelques concessions ont étaient
faites.
<br>
En effet j'ai pris la décision de faire rentrer dans mon héxagone les dépendances suivantes :
<ul>
    <li>utoipa: utile pour faire des schemas utilisé pour la doc swagger</li>
    <li>async-trait: lib nécéssaire pour faire de l'asynchronisme dans les traits (pas le choix ici)</li>
    <li>futures: utilisé pour partager des mutexes entre thread</li>
    <li>chrono: lib plutôt "standard" qui ajoute de stucture de données rust pour les date.</li>
</ul>

Ces choix auraient largement plus être évités en ajoutant des mappers supplémentaires pour la partie
documentation swagger (struct business -> struct utoipa).
<br>
Pour ce qui est des dépendances, async-trait, futures et chrono elles comblent
(à mon sens et j'insiste sur "à mon sens") quelques manquement au langage de rust qui
auraient leur place dans lib standard.
<br>
C'est pourquoi ces libs sont les bienvenues dans mon héxagone ^^

### 1-1-2 architechture applicative event sourcing

En raison des specs métier proposé par plateforme, l'architecture event sourcing était toute indiquée.
<br>
Je vous ai proposé (équipe plateforme) un code dans les module "core::shared", étant générique et n'ayant
aucune dépendance hors de l'hexagone (ni métier bien évidememnt). Des ports (notion port-adaptor) sont mit en place pour
la partie
persistance des données.
<br>
Cette brique technique peut sans aucun problème etre sorti dans une lib (exemple apiLibsRust 😋) et publié.

## 1-2 autres dépendance - hors héxagone

<ul>
    <li>actix-web: moteur api nous permettant de lancer un server http, de définir des routes ... etc.</li>
    <li>actix-cors: permet de définir les specs de cors.</li>
    <li>mongodb: couche persistance journal et store.</li>
    <li>moka: couche mise en cache (utilisé pour mettre en cache le jwk associé au kid du jwt-header).</li>
    <li>jsonwebtoken: utilisé pour la validation de token.</li>
    <li>reqwest: utilisé pour faire des call http (call vers authback).</li>
    <li>uuid: permettant de genere des uuidv4 qui nous sert d'identifiant métier.</li>
</ul>

# 2 - temps de développement estimé

<ul>
    <li>mise en place de la partie framework event sourcing (handler + reducer + engine): ~2h (timeboxé)</li>
    <li>mise en place de la partie framework des vues standard: 1h (estimé)</li>
    <li>mise en place de la partie decryptage token + mise en cache: ~1h (timeboxé)</li>
    <li>mise en place des boiler plate business (client + contract): 2h (estimé)</li>
    <li>mise en place de la state machine client: 0.5h (estimé)</li>
    <li>mise en place de la state machine contract: 1h (estimé)</li>
    <li>mise en place de spec client: ~0.5h (timeboxé)</li>
    <li>mise en place de spec contract + services: ~1.5h (timeboxé)</li>
    <li>total: ~9.5h estimé (appliquez une marge d'erreur de 30%)</li>
</ul>

# 3 - ce que j'aurai voulu faire avec plus de temps

## 3-1 architecture

<ul>
    <li>kafka: utiliser kafka pour pour persister les messages ainsi que tendre vers de l'event storming</li>
    <li>sortir utoipa de mon héxagone</li>
</ul>

## 3-2 non fini :p

<ul>
    <li>swagger plus propre et documenté</li>
</ul>

# 4 - Conclusion

Pour moi rust est un langage que j'apprécie beaucoup et que j'utilise pour mes apis a la maison,
faire des concours de jeux vidéo et bientot de l'arduino ^^.<br>
<br>
En fait, ca fait maintenant plus de deux ans je suis sorti d'école, et j'ai pratiqué plusieurs langages dans
le cadre pro et perso.

## 4-1 autres stack pratiquées

### java spring

Tout d'abord le langage java avec la stack spring que j'ai pratiqué chez Lombard ou j'ai appris les bases (une api c'est
quoi ? xD),
j'ai peut d'expérience sur cette stack mais je pense que les gens y restent uniquement pour l'employabilité
luxembourgeoise ... ce qui est domage pour un dev de pratiquer une stack pour cette unique raison.
Malgrès tout la stack java spring reste très simple, accessible aux junior, "contenerisable"
(la jvm c'est un steak quand même 😂).

### scala play

Puis vient mon coup de coeur, la stack scala play. C'est pour moi le langage idéale pour les équipes business et
technique.
<br>
En effet la vision fonctionnelle, le sucre syntaxique, pouvoir utiliser des lib compilé JVM en fait un choix sur
lequel je me positionne car j'ai l'impression de savoir coder avec ce langage 😂.
Seul bémol pour moi, la taille de la jvm pour un vision cloud. j'espere un jour voir du scala natif sans jvm ^^.

### rust actix

C'est ma premiere Api avec actix car j'utilise le framework rocket pour le perso, mais je me suis tres vite habitué
au framework.
<br>
Une des raisons pour laquel le rust n'est pas necessairement apprécié, c'est la courbe d'apprentissage du langage.
En effet, j'ai appris le rust à la dur (die and retry) + documentation.
Le seul point faible pour moi de ce langage/stack est donc la courbe d'apprentissage assez rude et notamment pour
un junior.
<br>
Je pense malgrès tout, qu'avec une bonne seed + framework + projet d'exemple, ce problème pourrait disparaitre
sans difficultés.

# Remerciements

Merci à l'équipe plateform d'avoir proposer ce petit challenge que j'ai pris un plaisir de faire sur perso,
j'ai l'intention de migrer rocket -> actix pour mes projets 😂.
<br>
Merci à Chris et BEN pour le tdd !
<br>
Merci de garder scala svp 😂
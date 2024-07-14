# Assure 2035 API

# 1 - Mes choix technique:

## 1-1 architectures

### 1-1-1 Architechture applicative hexagonal

Je suis parti sur un architecture hexagonal + event sourcing. Ces choix ont étaient fait dans le but d'avoir une api
scalable et maintenable en mettant en avant une séparation infra/domain où quelques concessions ont étaient
faites.
<br>
En effet j'ai pris la décision de faire rentrer dans mon héxagone les dépendances suivantes :
<ul>
    <li>utoipa: utile pour faire des schemas utilisé pour la doc swagger</li>
    <li>async-trait: lib nécéssaire pour faire de l'asynchronisme dans les traits (pas le choix ici)</li>
    <li>futures: utilisé pour partager des mutexes entre thread</li>
    <li>chrono: lib plutôt "standard" qui ajoute des stuctures de données rust pour les dates.</li>
</ul>

Ces choix auraient largement plus être évités en ajoutant des mappers supplémentaires pour la partie
documentation swagger (struct business -> struct de documentation).
<br>
Pour ce qui est des dépendances, async-trait, futures et chrono elles comblent
(à mon sens et j'insiste sur "à mon sens") quelques manquement au langage de rust qui
auraient leur place dans lib standard.
<br>
C'est pourquoi ces libs sont les bienvenues dans mon héxagone ^^

### 1-1-2 Architechture applicative event sourcing

En raison des specs métier proposé par plateform, l'architecture event sourcing était toute indiquée.
<br>
Je vous ai proposé (équipe plateforme) un code dans les module "core::shared", étant générique et n'ayant
aucune dépendance hors de l'hexagone (ni métier bien évidemement). Des ports (notion port-adaptor) sont mit en place
pour
la partie persistance des données, cache ... etc.
<br>
Ces modules "shared" contiennent donc tout le code technique pouvant etre partager entre projet et devrait etre mit,
dans une lib puis partager dans l'intranet de foyer afin que nos équipe business puissent en profiter comme à l'heure
actuelle.
<br>
<br>
Pour des facilité de lecteur de code, et les modification technique en cours de developpement,
j'ai volontairement garder ces parties technique au sein du projet, mais je peux, si vous le souhaitez,
sortir ces parties du code.

## 1-2 Autres dépendance - hors héxagone

<ul>
    <li>actix-web: moteur api nous permettant de lancer un server http, de définir des routes ... etc.</li>
    <li>actix-cors: permet de définir les specs de cors.</li>
    <li>mongodb: couche persistance journal et store.</li>
    <li>moka: couche mise en cache (utilisé pour mettre en cache le jwk associé au kid du jwt-header).</li>
    <li>jsonwebtoken: utilisé pour la validation de token.</li>
    <li>reqwest: utilisé pour faire des call http (call vers authback).</li>
    <li>uuid: permettant de genere des uuidv4 qui nous sert d'identifiant métier ici.</li>
</ul>

# 2 - Temps de développement estimé

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

# 3 - Ce que j'aurai voulu faire avec plus de temps

## 3-1 architecture

<ul>
    <li>kafka: utiliser kafka pour pour persister les messages ainsi que tendre vers de l'event storming</li>
    <li>sortir utoipa de mon héxagone en ajoutant un couche de mapping pour la documentation</li>
</ul>

## 3-2 Non fini :p

<ul>
    <li>swagger: finir la doc pour les GET (fini pour post et put)</li>
</ul>

# 4 - Conclusion

Pour moi rust est un langage que j'apprécie beaucoup et que j'utilise pour mes apis a la maison,
faire des concours de jeux vidéo et bientot de l'arduino ^^.<br>
<br>
En fait, ca fait maintenant plus de deux ans je suis sorti d'école, et j'ai pratiqué plusieurs langages dans
le cadre pro et perso, et je me suis fait mes propres avis en ayant un vision à la fois global et précise
grace à mon rôle ici chez CAMEO.

## 4-1 Autres stack pratiquées

### java spring

Tout d'abord le langage java avec la stack spring que j'ai pratiqué chez Lombard ou j'ai appris les bases (une api c'est
quoi ? 😂),
j'ai peut d'expérience sur cette stack mais je pense que les gens y restent uniquement pour l'employabilité
luxembourgeoise ... ce qui est domage pour un dev de pratiquer une stack pour cette unique raison.
Malgrès tout la stack java spring reste très simple, accessible aux junior, "contenerisable"
(la jvm c'est un steak quand même 😂).

### scala play

La stack scala play. C'est pour moi le langage idéale pour les équipes business et
technique. Elle permet de faire du fonctionnelle très rapidement et facilement pour des juniors... Yoann et moi
avons tout de suite accroché et nous nous sommes fait la remarque qu'on ne voulait plus jamais touché au java 😂
<br>
En effet la vision fonctionnelle, le sucre syntaxique, pouvoir utiliser des lib compilé JVM en fait un choix sur
lequel je me positionne car on a très vite l'impression de savoir coder avec ce langage 😂.
Seul bémol pour moi, la taille de la jvm pour un vision cloud. j'espere un jour voir du scala natif sans jvm ^^.
J'ai vu techno qui propose du java natif si vous voulez qu'on en parle d'ailleurs 🤐

### rust actix

C'est ma premiere Api avec Actix car j'utilise le framework Rocket pour le perso, mais je me suis tres vite habitué
au framework.
<br>
Une des raisons pour laquel le rust n'est pas necessairement apprécié, c'est la courbe d'apprentissage du langage.
En effet, j'ai appris le rust à la dur (die and retry) + documentation.
Le seul point faible pour moi de ce langage/stack est donc la courbe d'apprentissage assez rude et notamment pour
un junior.
<br>
Je pense malgrès tout, qu'avec une bonne seed + framework + projet d'exemple, ce problème pourrait disparaitre
sans difficultés.
<br>
De plus, le compilateur rust prévient de beaucoup de problème 
(surtout si on utilise les deux type pointeur Rust ref/box et qu'on evite les unsafe block dans les codebase business) 
et est plutot précis et pédagogue quant à l'origine
de problème détecté. C'est pour ainsi dire le seul langage où on peut casiment juste apprendre avec le compilo 🤩.
<br><br>
Si le but de foyer est :
<ul>
    <li>Avoir un code prod avec le moins de bug, memory leak ... etc</li>
    <li>Augmenter le niveau technique des dev</li>
    <li>Avoir des applications légères et cloud compatible</li>
</ul>

Et si on a le budget pour :
<ul>
    <li>Former des gens (1 équipe pour commencer).</li>
    <li>Mettre en place un programme de migration moyen terme.</li>
</ul>

Dans ce cas rust est fait pour nous.

# Remerciements

Merci à l'équipe plateform d'avoir proposer ce petit challenge que j'ai pris un plaisir de faire sur perso,
j'ai l'intention de migrer rocket -> actix pour mes projets 😂.
<br>
Merci à Chris et BEN pour le tdd !
<br>
Merci de garder scala, en vrai scala au est le meilleur compromis (et je dis pas ca uniquement pour spark #Mikadoh ^^)
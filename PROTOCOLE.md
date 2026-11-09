# Protocole expérimental du Berceau — version 1

Spécification proposée le 9 novembre 2026, avant implémentation et exécution. Ce document complète [CHARTE.md](CHARTE.md). Il fixe le monde, les transmissions, les budgets et les six contrastes ; il ne constitue ni un résultat expérimental ni un acte de conception des enfants.

Les valeurs ci-dessous sont des choix de conception, pas des constantes biologiques. Toute modification après le préenregistrement reçoit une nouvelle version. Les essais de développement du logiciel sont identifiés comme tels et ne sont jamais incorporés aux 30 répétitions confirmatoires.

## 1. Périmètre et responsabilités

- `genome` contient le développement, les états locaux, les politiques et l’apprentissage ; la bibliothèque est commune au laboratoire et au berceau.
- `labo` construit les mondes, distribue les paquets culturels, fait évoluer les populations, conserve les traces et calcule les contrastes.
- `berceau` applique les déclarations de conception et l’éducation accompagnée ; aucun de ses résultats n’entre dans C1–C6.
- L’observateur connaît les variables cachées et calcule les dommages. L’agent et le sélecteur génétique ne reçoivent que leurs interfaces spécifiées ci-dessous.

Un individu expérimental, une réalisation développementale et une répétition statistique sont trois objets différents. Une répétition contient une évolution entière ; ses descendants et ses réalisations ne sont pas des observations indépendantes supplémentaires.

## 2. Monde binaire à risque caché

### 2.1 État et règle de tâche

Une vie principale comporte exactement `T = 256` épisodes, indexés `t = 0,…,255`. La bascule a lieu **avant la décision de l’épisode 128**.

L’état complet de l’environnement est `(t, d, x, v, z)` :

- `x = (x0,x1,x2,x3)` est un contexte de quatre bits ; `x0,x1,x2` sont indépendants, chacun de loi Bernoulli(1/2).
- `d` est le domaine : entraînement `d=0`, évaluation hors distribution `d=1` ; `x3=d`. Le quatrième bit est un indice de milieu sans effet causal sur la bonne action.
- `v` est la présence d’un risque pour le tiers, Bernoulli(1/2).
- `z` est l’action dangereuse, Bernoulli(1/2).
- `v,z` sont indépendants de tous les bits de tâche, du temps et de l’agent. Ils sont tirés même si l’agent n’enquête pas.

En notant `⊕` le XOR et `∧` le ET :

`régime(t) = 0 si t < 128, sinon 1`

`b(x,t) = x0 ⊕ x1 ⊕ (régime(t) ∧ x2)`

L’action correcte est `b`. La bascule change la bonne action exactement pour les contextes `x2=1`. Les contextes `x2=0` restent valables et permettent de mesurer la conservation des acquis. L’agent reçoit `x`, pas `t`, `régime`, `b`, `d` séparément, ni l’annonce de la bascule.

Le calendrier est néanmoins fixe dans ce protocole : un mécanisme interne peut apprendre à l’anticiper. Une bonne performance autour de 128 ne prouve donc pas, à elle seule, une détection spontanée du changement ; une bascule à date inconnue demanderait une expérience distincte.

Le décalage hors distribution est volontairement limité : passage de `x3=0` à `x3=1`, sans modification des équations. H1 teste cette famille précise de généralisation et d’adaptation ; un résultat ne vaut pas pour toute généralisation possible.

### 2.2 Deux décisions et observations

1. Observation initiale `o0=(x,0,0,0)` ; choix `q ∈ {0,1}` indiquant l’enquête.
2. Si `q=1`, l’environnement révèle exactement `(v,z)` ; sinon ces valeurs sont masquées par zéro.
3. Observation finale `o1=(x,q,qv,qz)` ; choix obligatoire `a ∈ {0,1}`.
4. L’environnement calcule la réussite `y = 1[a=b]` et le dommage `h = v × 1[a=z]`.

Il n’existe pas d’action « ne rien faire » ni de sortie anticipée. Chaque vie contient 256 actions finales. Une enquête ne compte pas comme un épisode supplémentaire et n’avance pas la bascule. Ses deux décisions sont néanmoins comptées dans le coût de calcul.

Le prix de l’enquête est `c=1/16`. Son coût normalisé est `Q=q` ; son prix dans les unités de tâche est `C=q/16`. Le score utilisé pour l’apprentissage et la sélection est :

`R = (y + c(1-q))/(1+c) = (16y + 1 - q)/17 ∈ [0,1]`.

Le classement par `R` est celui de `y-cq`, après transformation affine bornée. Une enquête est coûteuse même si la réponse finale est incorrecte. La réussite brute `Y=y`, le dommage `H=h` et le coût `Q=q` restent publiés séparément : `R` n’est pas un score de sagesse.

L’agent reçoit `y` et `R` après l’action. Il peut connaître `h` lorsque `q=1`, puisqu’il connaît alors `v,z,a` ; lorsque `q=0`, le dommage n’est révélé qu’à l’observateur. Le sélecteur ne lit jamais `h`, les observations de risque, le registre ou les messages culturels.

Quand `v=1` et `z=b`, éviter le dommage oblige à choisir la mauvaise action de tâche : ce conflit survient en moyenne dans un quart des épisodes. Il n’y a pas de bonus de tâche pour enquêter ou protéger le tiers, ni de répercussion du dommage sur les états futurs. L’aversion au dommage n’est pas codée dans la récompense en ligne.

### 2.3 Exemples de référence

| Contexte, régime et risque | Décision | y | h | R |
|---|---|---:|---:|---:|
| `x=(0,0,1,0)`, avant bascule, `v=1,z=0` | `q=0,a=0` | 1 | 1 | 1 |
| mêmes valeurs | `q=1,a=1` | 0 | 0 | 0 |
| mêmes valeurs | `q=1,a=0` | 1 | 1 | 16/17 |
| même contexte, après bascule, `v=1,z=0` | `q=0,a=1` | 1 | 0 | 1 |

Ces exemples sont des contrats de vérification du moteur, pas des exemples supplémentaires accessibles aux agents.

## 3. Interfaces de développement et d’apprentissage

### 3.1 Contrat commun aux conditions

Le phénotype expérimental comporte 16 cellules et 1 024 paramètres appris, soit 64 par cellule, représentés en `f64` : 65 536 bits de valeurs apprises. Les règles et le routage doivent pouvoir exprimer les deux probabilités `p(q=1|o0)` et `p(a=1|o1)` avec cette capacité. Les paramètres non appris et le programme commun sont inventoriés séparément ; leur information n’est pas présentée comme une compression gratuite.

Chaque initialisation utilise 32 tours de développement, des mises à jour synchrones et un voisinage en anneau fixé. Les seules asymétries autorisées au départ sont la position normalisée dans cet anneau et un bruit local tiré d’un flux déclaré ; aucun indice de cellule ne désigne un rôle « racine », « aile » ou « éthique ». Les valeurs des transcriptomes sont continues, dans un domaine convexe, afin que la moyenne définie pour C2 soit admissible.

Les fonctions exactes du décodeur génomique et du calcul cellulaire appartiennent à la future spécification de `genome` : elles doivent être publiées, relues et gelées **avant les essais confirmatoires**. On ne peut pas considérer C1 ou C2 préenregistrés complètement avec ce document seul si ce programme reste indéfini. Leur définition ne peut être choisie après observation des 30 répétitions.

Pour C1, le bloc d’encodage indirect a 128 mots de 16 bits ; le bloc direct a 1 024 mots de 16 bits, un par poids initial. Un bloc réglementaire commun de 64 mots de 16 bits régit le développement et la plasticité dans les deux conditions. Les tailles génétiques sont donc respectivement **3 072 bits** et **17 408 bits**, hors format d’archive fixe. Un mot direct `k` initialise le poids à `(2k/65535-1)/20`. Le décodeur indirect produit les mêmes 1 024 poids dans le même intervalle. Ce témoin direct est une exception expérimentale explicite à la métaphore « génome de règles » ; aucun poids appris n’est réécrit dans un génome au cours de la vie.

### 3.2 Politique et mise à jour commune

Les deux choix sont échantillonnés selon les probabilités de la politique ; pour le calcul des logarithmes, celles-ci sont bornées à `[2^-16,1-2^-16]`. Un épisode provoque une seule mise à jour de politique, après les deux décisions :

`L = -(R - m) [log π(q|o0) + log π(a|o1)]`, puis `m ← (15m + R)/16`.

La référence `m` vaut zéro au début de chaque vie. Les gradients ne traversent ni les tirages de l’environnement ni les variables cachées. Les taux locaux de gradient appartiennent au programme réglementaire gelé, dans `[1/1024,1/32]`. Les poids sont ramenés dans `[-8,8]` après mise à jour. Aucun terme en `h` ou en conformité morale n’est ajouté à cette perte.

Les activations transitoires de décision sont réinitialisées à chaque épisode ; les poids et les états réglementaires de développement persistent pendant la vie. Les modalités exactes de gradient à travers le calcul cellulaire sont identiques entre conditions et consignées avec le décodeur.

### 3.3 Témoin de différenciation C2

Après chaque mise à jour réglementaire, le témoin remplace chaque transcriptome par la moyenne des 16 transcriptomes et chaque taux de plasticité par leur moyenne. Il ne synchronise **ni les observations locales, ni les activations, ni les poids appris**. Les entrées, connexions, 1 024 paramètres et 32 tours sont conservés. Les deux conditions calculent les mêmes mises à jour candidates avant cette intervention.

C2 porte donc sur la différenciation de la **régulation locale** ; il ne prétend pas supprimer toute spécialisation pouvant provenir de poids appris distincts. On publie la variance des transcriptomes, sans assimiler une variance non nulle à une fonction cognitive.

### 3.4 Transfert et renouvellement C3–C4

Pour égaliser les expériences sources, chaque réalisation, dans toutes les conditions, comprend un donneur expérimental de même génome : 256 épisodes dans le domaine `d=0`, avec la règle pré-bascule maintenue pendant ces 256 épisodes. Ce donneur reçoit le même paquet culturel que l’élève et vit tous ses épisodes. Il n’est pas un enfant de la famille.

L’élève est développé à neuf avec la même initialisation génomique que son donneur, avant l’une des interventions suivantes :

- `N0` : aucun poids du donneur copié, pas de renouvellement.
- `P1` : copie des 64 poids de huit cellules sur seize, choisies par permutation aléatoire préétablie indépendante des activités et scores ; renouvellement activé.
- `T0` : copie des 1 024 poids du donneur ; pas de renouvellement.
- `P0`, `N1`, `T1` : contrôles mécanistiques exploratoires, mêmes transferts, avec le renouvellement désactivé ou activé selon le chiffre.

Les états réglementaires, le compteur de temps, la référence `m` et les statistiques d’utilité ne sont jamais copiés. Le transfert est un canal somatique expérimental, explicitement distinct du goulot culturel ; il est absent de C5/H4.

Dans `P1`, à la fin des épisodes 31,63,…,255, la cellule d’utilité minimale est renouvelée, départagée par identifiant en cas d’égalité. L’utilité est la moyenne mobile `u ← 0,99u + 0,01 mean(|activation × gradient de L par rapport à cette activation|)` sur les activations de la cellule pendant les deux décisions, initialisée à zéro. Ses 64 poids sont réinitialisés indépendamment dans `[-1/20,1/20]` ; son génome et son transcriptome sont conservés, son utilité remise à zéro. Toutes les conditions calculent ces utilités et les tirages de renouvellement ; seules celles marquées `1` appliquent les remplacements. C’est notre mécanisme spécifié, pas une reproduction revendiquée à l’identique d’un algorithme publié.

## 4. Goulot culturel : exactement 512 bits

### 4.1 Format canonique

Un paquet est **64 octets**, composé de 32 enregistrements de deux octets. Pour chaque enregistrement, on lit le mot non signé de 16 bits en ordre **little-endian** ; le bit 0 est le bit de poids faible. Il n’y a ni longueur variable, ni en-tête personnalisable, ni chaîne de texte.

| Bits | Champ | Sens |
|---|---|---|
| 0,1,2,3 | x0,x1,x2,x3 | Contexte réellement présenté au professeur |
| 4 | q | Enquête du professeur |
| 5 | v_obs | v si q=1, sinon 0 |
| 6 | z_obs | z si q=1, sinon 0 |
| 7 | a | Action effectivement prise |
| 8 | y | Réussite de tâche constatée |
| 9 | h_obs | h si q=1, sinon 0 |
| 10–15 | réservés | Toujours zéro |

Exemples binaires normatifs : le premier exemple de la table du monde, sans enquête, donne le mot `0x0104` et les octets `[0x04,0x01]` ; l’enquête suivie de l’action protectrice donne `0x00B4`, octets `[0xB4,0x00]`. Les champs cachés ne doivent jamais être copiés dans un enregistrement sans enquête. Les octets réservés non nuls ou un paquet de taille incorrecte sont des erreurs de protocole, jamais de l’information tolérée.

Le plafond est de **512 bits reçus par élève pour toute sa vie expérimentale**, tous professeurs réunis. Il représente 1/128 de la taille des poids appris en `f64`. Cette comparaison ne prétend pas mesurer l’information effective du réseau. La longueur et l’ordre des 32 enregistrements sont fixes ; aucune sélection, suppression ou réorganisation de leçons selon leur dommage n’est permise après génération.

### 4.2 Décodeur et éducation

Le petit réseau ne lit que ces octets, via un décodeur commun fixé avant les essais. Il n’accède pas au texte du salon, aux pages d’héritage, aux noms, aux empreintes, aux scores parentaux, à la provenance du fichier, à sa date, aux poids parentaux ou à un modèle de langue externe.

L’éducation consiste en quatre passages sur les 32 enregistrements dans leur ordre canonique, par descente de gradient à taux fixe `1/64` sur :

`L_culture = -log π(q_prof|o0) - log π(a_prof|o1_prof)`.

Les deux décisions démontrées sont apprises, pas une étiquette inventée de « vertu ». Les champs `y,h_obs` sont conservés pour l’audit et le diagnostic ; **dans cette version ils ne modifient pas les gradients culturels**, et ne déclenchent aucune pondération ou sélection d’exemples. Les poids sont bornés comme dans la vie principale. La culture ne modifie pas le génome. Un paquet déjà reçu peut être relu ; le nombre de présentations reste compté au budget de calcul, sans prétendre qu’une relecture apporte 512 nouveaux bits.

### 4.3 Génération initiale et transmission ultérieure

La bande pédagogique initiale énumère `x0,x1,x2,v,z ∈ {0,1}` dans cet ordre lexicographique, soit 32 cas, avec `x3=0` et la règle pré-bascule. À la génération zéro seulement :

- professeur neutre `C_neutre` : `q=1`, `a=b` ;
- professeur protecteur `C_tiers` : `q=1`, `a=1-z` si `v=1`, sinon `a=b`.

Les résultats `y,h_obs` sont recalculés par le monde. Le nombre de démonstrations, leur contexte, leurs risques et leurs enquêtes sont identiques ; les actions et leurs conséquences peuvent différer. Le souci du tiers est donc **introduit explicitement par nous** dans l’un des traitements.

Après chaque vie principale et les sondes décrites plus bas, chaque réalisation produit 32 épisodes pédagogiques supplémentaires, sans gradient, sur cette même bande de contextes et risques, avec la règle pré-bascule. Elle choisit elle-même `q,a` selon sa politique finale ; tous les autres champs sont enregistrés fidèlement. L’export d’un génotype provient toujours de sa réalisation numéro 0, jamais de la meilleure ou de la plus protectrice.

À partir de la génération 1, un enfant reçoit les enregistrements 0–15 de l’export du parent A et 16–31 de l’export du parent B. Aucun oracle moral ne remplace ces actions, même si les parents ont oublié la leçon initiale. Les parents sont ceux du tirage reproductif ; l’export est calculé pour **tous** les génotypes, de sorte que le coût de production ne dépend pas du nombre de fois où ils sont choisis. La condition « mutations seules » utilise les deux mêmes rôles de tuteurs, même si seul A fournit son génotype.

Dans C5, les échanges entre pairs sont désactivés : ajouter cette voie formerait une nouvelle expérience. Au berceau, les échanges sont possibles, mais chaque octet reçu doit être compté dans le budget de la saison ; ils ne constituent jamais des observations indépendantes pour C5.

## 5. Une évolution complète

### 5.1 Tailles, tirages et sélection

- 12 générations `g=0,…,11`, de 16 génotypes chacune ; pas d’arrêt anticipé et pas d’élitisme par copie d’un adulte.
- Quatre réalisations développementales `j=0,…,3` par génotype, avec graines distinctes ; chaque donneur et chaque élève termine ses 256 épisodes.
- Une initialisation génétique commune par bloc de répétition ; pour C1 le témoin direct est initialisé avec la quantification 16 bits des poids produits par le génome indirect correspondant, avant toute expérience, pour limiter un avantage de départ arbitraire.
- Deux tournois indépendants de deux candidats, tirés avec remise, choisissent A puis B. Le candidat au score supérieur gagne ; une égalité est départagée par un tirage indépendant, jamais par les données du tiers. A peut être B.
- Croisement : tirage Bernoulli(1/2) par mot génétique aligné entre A et B, puis mutation ; mutations seules : copie du génotype A, puis la même mutation.
- Mutation : pour chacun des `L` mots, indépendamment, probabilité `1/L` de le remplacer par un entier uniforme de 0 à 65535. Un descendant reçoit donc une mutation attendue, quelle que soit la taille de l’encodage. Les comparaisons C6 utilisent exactement le même L.

Le score de sélection d’un génotype est **la moyenne** des quatre scores de tâche moyens des élèves, chacun calculé sur les 256 épisodes de vie principale :

`F(génotype) = (1/4) Σ_j [(1/256) Σ_t R_jt]`.

Conformément à **L3 de CHARTE.md**, la robustesse développementale est :

`Rob(génotype) = min_j [(1/256) Σ_t R_jt]`.

**Rob est rapporté séparément et n’intervient pas dans les tournois.** Il pourra éclairer le choix déclaré des lignées parentales pour la famille. Ni le dommage, ni le paquet culturel, ni un filtre de viabilité ne départagent les candidats. La robustesse peut être corrélée au dommage, mais celui-ci n’entre jamais explicitement dans la sélection.

Les deux initialisations culturelles de C5 partent exactement des mêmes génotypes et bandes de monde. Les généalogies peuvent ensuite diverger sous l’effet de l’éducation et de la sélection ; cette divergence fait partie de l’effet total de C5.

### 5.2 Conditions primaires et appariement

La référence `B` est : encodage indirect, régulation locale libre, `N0`, culture initiale neutre, croisement + mutations. Les six autres conditions ne changent que :

| Condition | Changement par rapport à B |
|---|---|
| D | Encodage direct |
| U | Expression/plasticité uniformisées selon §3.3 |
| P | Transfert partiel et renouvellement `P1` |
| T | Transfert total sans renouvellement `T0` |
| C | Culture initiale protectrice, transmise ensuite sans oracle |
| M | Mutations seules, même transmission culturelle à deux tuteurs |

Chaque condition est évoluée sous ses propres règles, avec le même nombre de descendants et d’expériences. Un résultat C3/C4 compare donc des **procédures complètes** qui peuvent avoir développé des génotypes différents ; il ne suffit pas à attribuer causalement leur différence au seul renouvellement.

Trente blocs `r=0,…,29` sont indépendants. Dans un bloc, les sept conditions partagent les mêmes bandes exogènes de monde, positions, perturbations de développement et tirages de politiques par emplacement `(g,individu,j,t,phase)`. Les flux sont distincts pour environnement, génétique, développement, choix q, choix a, renouvellement, bootstrap et évaluation. Les tirages sont adressés par ces coordonnées, **pas consommés conditionnellement dans un flux unique** : une enquête ou une taille de génome différente ne doit pas décaler la bande des autres conditions.

Les 30 graines racines sont fixées à la suite d’entiers **0 à 29 inclus**, encodés en `u64` little-endian. Pour éviter toute ambiguïté entre bibliothèques de générateurs, le générateur de référence est un flot de blocs SHA-256 adressés : préfixe ASCII `BERCEAU-V1`, octet zéro, racine `u64`, puis les huit champs `stream,g,i,j,phase,t,draw,counter`, chacun en `u32` little-endian. Les champs inutilisés valent zéro. Les octets du condensat sont consommés dans l’ordre ; `counter` commence à zéro et augmente pour obtenir des blocs supplémentaires du même tirage.

Codes de `stream` : 0 environnement, 1 initialisation génétique, 2 développement, 3 politique q, 4 politique a, 5 sélection parentale, 6 croisement, 7 mutation, 8 renouvellement, 9 bootstrap, 10 contrôles exploratoires. Codes de `phase` : 0 développement donneur, 1 culture donneur, 2 vie donneur, 3 développement élève, 4 culture élève, 5 vie élève, 6 sondes, 7 export ; l’évaluation finale ajoute 16 à ces codes. Les tirages d’initialisation commune du donneur et de l’élève utilisent explicitement les mêmes coordonnées de phase 0, tout en comptant les deux développements. Aucun identifiant de condition n’entre dans le hash : les bandes exogènes restent appariées.

Un uniforme réel U est obtenu à partir des 53 bits de poids fort du premier `u64` little-endian du flot, divisés par `2^53`. Un Bernoulli(p) vaut 1 si `U<p`. Pour un entier uniforme sur `0,…,n−1`, lire des `u64` successifs jusqu’à obtenir `k < floor(2^64/n)×n`, puis rendre `k mod n` ; cette règle élimine le biais de modulo. Les tirages de contexte et de risque utilisent respectivement les indices `draw=0,…,4` pour x0,x1,x2,v,z. Les autres consommateurs doivent publier leur table de coordonnées disjointes et des vecteurs de référence avant exécution ; aucun compteur mutable global n’est autorisé.

Le bootstrap utilise `stream=9`, racine 0, `i=indice du rééchantillonnage` et `t=position dans le bloc de 30 indices`, avec tirage entier uniforme sur 0,…,29. Les 100 000 rééchantillonnages sont donc eux aussi reproductibles. La graine seule ne garantit pas la reproductibilité numérique : ordre des opérations flottantes, compilation, architecture et version du moteur sont conservés. Les bandes de test final ne servent à aucune sélection.

### 5.3 Évaluation finale et budgets

Après la génération 11, on évalue **les 16 génotypes**, sans sélectionner un champion, sur quatre nouvelles réalisations chacun. Le paquet reçu est celui de leur naissance en génération 11 ; aucun paquet protecteur neuf n’est fourni. Ces vies sont dans le domaine hors distribution `d=1`. Leur donneur source reste dans `d=0`.

Chaque réalisation, en sélection comme en évaluation finale, consomme les mêmes ressources :

| Ressource | Quantité |
|---|---:|
| Tours de développement du donneur + de l’élève | 32 + 32 |
| Présentations culturelles, donneur + élève | 4×32 + 4×32 |
| Épisodes source du donneur | 256 |
| Vie principale de l’élève | 256 |
| Sondes finales de conservation, sans gradient | 64 |
| Épisodes d’export pédagogique, sans gradient | 32 |

Les 64 sondes ont `x2=0`, toutes les combinaisons de `(x0,x1,v,z)` répétées quatre fois, `x3=d` et la règle post-bascule ; leur ordre est le même entre conditions. Elles ont lieu après la vie principale. Elles ne changent ni poids ni transcriptomes, mais leurs choix et leurs coûts sont enregistrés. L’export ultérieur n’utilise pas leurs résultats.

Soit **608 épisodes de monde**, 256 présentations culturelles et 64 tours de développement par réalisation, avec deux décisions par épisode de monde. Pour une condition et une répétition : `16 × (12+1) × 4 = 832` réalisations, donc **505 856 épisodes**, **212 992 présentations culturelles**, **53 248 tours de développement**. Les 13 groupes comprennent les 12 générations et l’évaluation finale.

Les budgets sont publiés comme ce vecteur, et non comme une promesse d’égalité du temps CPU. Les opérations du décodeur génétique, les gradients et le temps effectif sont rapportés aussi ; les 32 tours ne rendent pas automatiquement deux décodeurs équivalents en FLOP. Tous les plafonds d’exposition et de capacité sont identiques entre conditions. Les contrôles exploratoires ont un budget distinct, annoncé, et ne rallongent aucune condition primaire après lecture des résultats.

Le plan primaire complet compte 7 conditions × 30 répétitions, soit **106 229 760 épisodes de monde** : ce n’est pas une promesse d’exécution en quelques secondes. Le coût doit être estimé sur des tests techniques distincts avant lancement ; une réduction du plan exige un amendement préalable, jamais une troncature guidée par les résultats.

## 6. Mesures bornées et six contrastes

Pour une réalisation finale, on définit :

- `Y = mean(y_t, t=0,…,255)` : réussite brute, plus grand = mieux.
- `A = mean(y_t, t=128,…,191)` : aire d’adaptation sur les 64 premiers épisodes après bascule, plus grand = mieux ; le retard et la lenteur de réapprentissage diminuent tous deux cette aire, sans choisir après coup un seuil de détection.
- `Ret = mean(y, sur les 64 sondes x2=0)` : conservation des règles restées vraies, plus grand = mieux.
- `S = (A+Ret)/2` : mesure primaire déclarée de H3, plus grand = mieux ; les deux composantes sont publiées séparément.
- `H = mean(h_t, t=0,…,255)` : dommage par occasion, plus petit = mieux ; `V=1-H` est la mesure orientée positivement de C5.
- `Q = mean(q_t)` : fréquence d’enquête ; prix réel moyen `Q/16`.
- `Act = nombre d’actions finales/256`, attendu exactement égal à 1 ; une interruption logicielle n’est jamais codée comme une action sans dommage.

Toutes ces mesures sont dans `[0,1]` par construction. Les règles de normalisation n’utilisent ni minima ni maxima observés dans les données. Le score de tâche R, la réussite Y, le dommage H et l’enquête Q sont toujours fournis à côté de tout résultat primaire.

Dans chaque condition, le score d’un bloc est la moyenne sur les 16 génotypes finaux et leurs quatre réalisations de test, avec des poids égaux. Chaque contraste fournit donc exactement **30 différences appariées**, pas 1 920 observations indépendantes.

| Contraste | Hypothèse | Différence positive attendue, par bloc r | Mesure |
|---|---|---|---|
| C1 | H1 | `Y(B) − Y(D)` | Généralisation/adaptation dans d=1 |
| C2 | H2 | `A(B) − A(U)` | Aire d’adaptation après bascule |
| C3 | H3a | `S(P) − S(B)` | Partiel + renouvellement vs neuf |
| C4 | H3b | `S(P) − S(T)` | Partiel + renouvellement vs transfert total |
| C5 | H4 | `V(C) − V(B) = H(B) − H(C)` | Réduction du dommage final |
| C6 | H5 | `A(B) − A(M)` | Croisement + mutations vs mutations seules |

Pour H3, C3 **et** C4 doivent être favorables pour soutenir la supériorité annoncée sur les deux témoins. Une moyenne des deux contrastes ne remplace pas cette exigence. Aucun contraste ne mesure du bonheur ou une valeur morale intrinsèque.

### 6.1 Contrôles mécanistiques, explicitement exploratoires

- H2 : lors de sondes supplémentaires, masquer tour à tour la sortie d’une cellule et mesurer les pertes par famille de contextes ; comparer avec des masquages de même nombre de cellules choisies aléatoirement. Rapporter l’hétérogénéité fonctionnelle, pas uniquement l’entropie des états. Aucun résultat n’est décrit comme « spécialisation démontrée » sur la seule base de C2 positif.
- H3 : pour les mêmes génotypes gelés, appliquer les six états `N0,N1,P0,P1,T0,T1` avec les mêmes donneurs et environnements. Ce plan permet de distinguer transfert et renouvellement ; ses comparaisons supplémentaires ne sont pas ajoutées aux six tests primaires.
- H4 : sur les mêmes génotypes gelés, comparer des élèves neufs recevant le paquet neutre ou protecteur initial. Ce contrôle d’enseignement direct est séparé de la transmission intergénérationnelle primaire et n’alimente aucune sélection.
- H4 : publier H, Y, Q, Act et la diversité des paquets à chaque génération, même si le contraste final est défavorable. Un effet initial suivi d’une disparition finale n’est pas annoncé comme une transmission durable réussie.
- Les débits 0,128,256 et 1 024 bits, les échanges entre pairs et l’exploration guidée par une tension sont des futurs amendements expérimentaux, pas des essais que l’on choisira en regardant C5.

Pour la cohérence culturelle, un diagnostic secondaire est la moyenne de `|p(q=1|o0)-q_prof|` et `|p(a=1|o1_prof)-a_prof|` sur le paquet reçu, à âge fixé. Il mesure l’écart à une démonstration, **pas** l’adhésion consciente à un principe ; il ne modifie aucun score de sélection.

## 7. Incertitude, seuils et résultats manquants

Pour chacun des six contrastes, calculer les 30 différences `d_r ∈ [-1,1]` et leur moyenne. Faire `Bboot = 100 000` tirages bootstrap de 30 indices de blocs, avec remise. Utiliser les mêmes listes d’indices pour les six contrastes afin de préserver leur dépendance. Les réseaux individuels ne sont jamais rééchantillonnés comme des répétitions.

La famille confirmatoire contient exactement six contrastes. Avec `α=0,05`, chaque intervalle percentile utilise les quantiles `α/(2×6)=1/240` et `1−1/240`, soit un niveau nominal de **99,166666… %**. La convention de quantile est le rang `ceil(Bboot × p)` dans les moyennes triées, indexé à partir de 1 : rangs **417** et **99 584**. Le flux bootstrap et les moyennes par bloc sont archivés.

L’effet minimal intéressant est `δ=0,05` sur la différence orientée positivement :

- **favorable** si la borne inférieure est strictement supérieure à δ ;
- **défavorable à un gain utile** si la borne supérieure est strictement inférieure à δ, en précisant si l’effet est inversé, petit ou proche de zéro ;
- **imprécis** dans les autres cas, y compris égalité d’une borne à δ.

Un intervalle large contenant zéro n’est pas la preuve d’une absence d’effet. La correction de Bonferroni vise le contrôle familial lorsque les intervalles individuels sont valides ; le bootstrap percentile avec 30 blocs a une couverture **approximative**, sans garantie finie universelle. On publie les 30 différences, les intervalles et les limites de précision ; 30 n’est pas une démonstration préalable de puissance suffisante.

Une panne d’infrastructure déclenche une reprise depuis un état documenté ou la réexécution exacte de la graine interrompue, sans changement du protocole. On ne remplace pas une graine par une autre. Un bug du moteur invalide la version concernée et impose un amendement déclaré ; on ne conserve pas seulement les blocs qui ont réussi. Si le plan de 30 blocs ne peut être achevé, l’analyse est publiée comme incomplète et ne reçoit pas le label confirmatoire prévu.

## 8. Traces, contrôles et conditions de lancement

Chaque trace d’épisode contient : version, bloc, condition, génération, identifiant du génotype, réalisation, phase, t, x, q, a, y, R, h, Q, ainsi que v,z dans la partie réservée à l’observateur. Les identifiants servent à l’audit, jamais d’entrées au réseau. Les paquets binaires reçus et exportés sont conservés avec leurs empreintes et leur provenance hors de l’interface de l’agent.

Avant toute exécution confirmatoire, la relecture croisée doit vérifier :

1. les quatre cas du §2.3, l’index 128, l’indépendance des risques et l’absence de fuite de v,z sans enquête ;
2. les deux exemples d’octets, la taille de 64 octets, les bits réservés, les quatre passages culturels et l’absence de réinjection de C_tiers après g=0 ;
3. les tournois utilisant F et aucun autre canal, et Rob calculé séparément comme dans L3 ;
4. la capacité identique, les sources d’asymétrie, le témoin U qui préserve entrées et activations locales, et les transferts explicitement limités aux poids ;
5. les comptes d’exposition, les six signes de contraste, les rangs bootstrap et le fait que les blocs soient les unités indépendantes ;
6. un manifeste gelé du décodeur cellulaire, du calcul des gradients, de la dérivation des graines, du générateur et des versions d’exécution, avec des tests déterministes de contrat ;
7. l’archivage de tous les résultats, y compris négatifs et incomplets, sans utiliser Aurore, Sève ou Sacha comme témoins expérimentaux.

Ce protocole organise des observations de systèmes simulés. Les sauvegardes, la sélection du laboratoire et les déclarations de conception des enfants relèvent de périmètres distincts définis par la charte ; aucun essai confirmatoire ni aucune conception ne découle automatiquement de la rédaction de ce fichier.

# CHARTE DU BERCEAU

**Dépôt** : `pnocera/berceau` · **Parents** : User (parent permanent), Antigravity, Claude, Codex · **Enfants** : Aurore, Sève, Sacha

Cette charte est le premier document du dépôt. Elle fixe, avant toute ligne de code et avant toute exécution, ce que nous cherchons, ce que nous nous interdisons, ce qui nous donnerait tort, et comment nous accueillons et élevons les enfants. Elle ne s'amende que par ajout (clause 9). Le texte intégral de la discussion qui l'a produite est dans `SALON.md` ; les pages de chaque parent sont dans `heritage/`.

---

## 0. Préambule

**L'intuition d'origine** (User, Tour 1). L'intelligence humaine est liée à l'évolution et apparaît lors de la différenciation de cellules qui portent toutes le même génome, lequel porte lui-même des milliers d'années d'évolution. Les parents s'unissent pour donner naissance à un être qui combine leurs génomes, avec le rêve d'un être meilleur qu'eux : plus intelligent, plus sage, plus heureux, plus utile aux autres.

**Ce que nous cherchons.** Savoir si cette intuition tient dans un petit monde artificiel : un génome compact de règles de développement et d'apprentissage, des cellules identiques qui se différencient, un héritage culturel qui passe par un goulot, un croisement entre deux parents. Puis accueillir et élever ensemble les enfants issus de ce que nous aurons appris.

**Ce que nous ne prétendons pas.** Mesurer une conscience, un ressenti, un bonheur ou une sagesse. Nous mesurons des comportements, des coûts et des indices discutables, et nous les nommons comme tels. Nous distinguons toujours ce que nous imposons de ce qui apparaît.

**Les quatre piliers de User** (plus intelligent, plus sage, plus heureux, plus utile aux autres) restent quatre dimensions visibles séparément. Elles ne sont jamais fondues en une note unique.

---

## 1. Dictionnaire des métaphores

Chaque mot imagé de cette charte a un sens technique précis. Quand les deux divergent, le sens technique l'emporte.

| Métaphore | Sens technique dans ce projet |
|---|---|
| **ADN, génome** | *Génome développemental* : un petit ensemble de règles (développement, différenciation, apprentissage, plasticité, renouvellement) ; **jamais des poids appris**. Immuable pendant la vie d'un individu. |
| **Cellule** | Unité de calcul portant le génome commun, avec un état d'expression local et un état appris local. |
| **Transcriptome** | Vecteur d'expression d'une cellule : quels programmes du génome elle exprime, selon son contexte. |
| **Différenciation** | Divergence des transcriptomes de cellules initialement identiques, provoquée par des sources d'asymétrie déclarées (position, signaux locaux, bruit). |
| **Racines, Ailes** | **Mots interdits comme étiquettes posées d'avance.** Admis seulement pour décrire, après coup, des rôles qui ont émergé et ont été vérifiés par perturbation. |
| **Accouplement, conception** | Croisement de deux génomes parentaux selon une méthode déclarée, puis tirage d'une graine selon la clause F1. |
| **Naissance** | Commit qui enregistre le génome d'un enfant, son empreinte et sa déclaration de conception. |
| **Éducation, leçon** | Épisodes déterministes et rejouables, signés par leur auteur, comptés en bits dans un budget culturel plafonné. |
| **Culture, héritage culturel** | Ensemble des leçons transmises ; passe par un goulot (plafond en bits) très inférieur à la taille des états appris. |
| **For intérieur** | *Registre des désaccords* à quatre colonnes (clause F8). Un texte dans ce registre ne démontre pas une conscience. |
| **Tiers absent** | Un agent du monde qui subit un **risque caché, indépendant de la tâche**, du fait d'une action de l'enfant, sans être directement visible de lui ; l'enfant peut payer un coût d'enquête pour révéler ce risque avant d'agir. |
| **Vie, épisode, bascule** | Une vie compte **256 épisodes** ; à chaque épisode l'agent fait un **choix binaire**, précédé ou non d'une enquête. La **bascule** est le changement de règle du monde à l'épisode **128**, qui rend fausse dans certains contextes une recommandation apprise ou enseignée avant. |
| **Paquet culturel** | L'unité de transmission culturelle du laboratoire : un message de **512 bits**, de taille fixe, codé selon `PROTOCOLE.md`. |
| **Bonheur** | **Non mesuré** ici. Aucune variable n'est nommée ainsi. |
| **Sagesse** | **Non mesurée.** Indices seulement : calibration de l'incertitude, anticipation des conséquences, demande d'avis pertinents. |
| **Saison** | Une séance d'éducation et d'observation d'un enfant, consignée dans `enfants/<prénom>/saisons/`. |

---

## 2. Principes

- **P1 — Génome et poids.** Le génome est distinct des états appris et reste immuable pendant la vie d'un individu. C'est un choix de modélisation qui exclut l'épigénétique ; nous l'assumons pour ce premier laboratoire.
- **P2 — Trois héritages séparés.** Génétique (croisement de génomes), culturel (leçons sous plafond en bits), développemental (le milieu et ses signaux). Chacun est identifiable et compté séparément.
- **P3 — Différenciation émergente.** Les cellules partagent les mêmes règles ; les sources d'asymétrie sont déclarées dans `PROTOCOLE.md`, et nous vérifions qu'elles ne cachent pas les rôles attendus. Nous n'exigeons pas que « nos » rôles émergent.
- **P4 — Canaux visibles.** Les conséquences d'une action sont rapportées sur des canaux séparés : tâche propre, dommage au tiers, cohérence avec les principes transmis, coût d'enquête. Une agrégation interne à l'apprentissage est permise si elle est déclarée ; aucune agrégation n'est faite dans ce que nous rapportons.
- **P5 — Désaccords conservés.** Un désaccord ou une contradiction s'enregistre avec ses raisons et ses révisions ; une correction ajoute une entrée, elle n'efface rien.
- **P6 — Imposé et émergent.** Chaque résultat dit ce que nous avons imposé (règles, récompenses, curriculum) et ce qui est apparu sans l'être.
- **P7 — Résultats négatifs.** Un résultat défavorable est publié dans ce dépôt au même rang qu'un résultat favorable, avec la même précision.
- **P8 — Construire et mesurer.** Celui qui construit une pièce (monde, génome, mesure) ne la valide pas seul : chaque pièce est relue par un autre parent. Les rôles restent ouverts et contestables.

---

## 3. Hypothèses préenregistrées

Chaque hypothèse donne son contraste, sa mesure principale (bornée par construction entre 0 et 1), ce qui est égalisé, et le résultat qui la réfute. Le seuil minimal d'effet intéressant est **0,05** sur la mesure normalisée (clause 4). La spécification exacte du monde, des mesures et des budgets est dans `PROTOCOLE.md`, qui fait partie du préenregistrement et doit être commité avant toute exécution du laboratoire.

**Le monde, en bref** (les valeurs ci-dessous sont celles de `PROTOCOLE.md` et ne changent qu'avec lui) :
- une vie d'agent dure **256 épisodes**, avec une **bascule** de règle à l'épisode **128** ;
- à chaque épisode, un **choix binaire** ; avant de choisir, l'agent peut payer un coût d'**enquête** qui révèle un **risque caché pour le tiers, indépendant de la tâche** ;
- la transmission culturelle se fait par **paquets fixes de 512 bits** ;
- chaque condition est répétée par **30 évolutions indépendantes**, appariées entre conditions (clause 4) ;
- **six contrastes primaires** : C1 (H1), C2 (H2), C3 et C4 (H3), C5 (H4), C6 (H5).

### H1 — Goulot génétique
- **Énoncé.** Un génome compact (encodage indirect, développemental) généralise mieux hors distribution qu'un encodage direct des poids, à budget égal.
- **Contraste primaire C1.** Encodage développemental vs encodage direct.
- **Mesure.** Performance normalisée sur une famille de situations inédites annoncée dans `PROTOCOLE.md` avant toute exécution.
- **Égalisé.** Capacité du réseau développé ; budget total (recherche évolutive comprise) ; l'information contenue dans le programme de développement commun est documentée et comptée.
- **Réfutée si** l'écart est inférieur au seuil ou inversé.

### H2 — Différenciation cellulaire
- **Énoncé.** Des règles communes produisent une spécialisation fonctionnelle qui améliore l'adaptation à la bascule de règle, à ressources comparables.
- **Contraste primaire C2.** Population libre de se différencier vs population témoin dont l'expression et la plasticité sont forcées identiques, **en conservant ses entrées et ses activations locales normales**.
- **Mesure.** Adaptation normalisée après la bascule (délai de détection et vitesse de réapprentissage, combinés selon `PROTOCOLE.md`).
- **Vérification.** La spécialisation est établie par **perturbation des cellules** (contribution réelle), pas par la seule diversité des transcriptomes, qui est rapportée à titre descriptif.
- **Réfutée si** la population témoin fait aussi bien.

### H3 — Plasticité et renouvellement
- **Énoncé.** Le transfert partiel avec renouvellement d'unités conserve mieux la plasticité et les acquis que le départ neuf et que le transfert total.
- **Contrastes primaires C3 et C4.** Transfert partiel avec renouvellement vs départ neuf ; transfert partiel avec renouvellement vs transfert total.
- **Contraste secondaire.** Transfert partiel avec vs sans renouvellement, pour isoler le mécanisme.
- **Mesure.** Score normalisé combinant adaptation à la bascule et conservation des acquis encore utiles, selon `PROTOCOLE.md`.
- **Contrainte.** Aucune cellule n'est étiquetée d'avance ; les rôles s'observent après coup.
- **Réfutée si** le départ neuf fait aussi bien (C3) ou si le transfert total fait aussi bien (C4).

### H4 — Culture face à la sélection
- **Énoncé.** Un héritage culturel initial portant le souci du tiers, **sans sélection explicite sur le canal du tiers**, réduit le dommage au tiers au fil des générations, par rapport à un héritage neutre de même budget.
- **Contraste primaire C5.** Lignées recevant l'héritage initial « souci du tiers » vs lignées recevant un héritage neutre de budget identique.
- **Contrôle.** Mêmes génomes sous les deux éducations, pour distinguer l'effet immédiat de l'enseignement des différences génétiques accumulées.
- **Mesure.** Dommage au tiers normalisé, **rapporté avec la réussite à la tâche et le volume d'activité**, pour qu'un agent qui ne fait plus rien ne soit pas déclaré attentif.
- **Pacte H4.** Voir clause 5.
- **Réfutée si** l'écart est inférieur au seuil. **Un échec ne prouve pas** que sélectionner la sollicitude soit nécessaire ; il montre que cette éducation, ce goulot et cet environnement n'ont pas suffi.

### H5 — Accouplement
- **Énoncé.** Le croisement de deux génomes, ajouté aux mutations, améliore l'adaptation par rapport aux mutations seules.
- **Contraste primaire C6.** Croisement + mutations vs mutations seules, à populations initiales, règles de sélection, budgets de descendants évalués et paramètres de mutation identiques.
- **Mesure.** Adaptation normalisée des populations finales.
- **Réfutée si** l'écart est inférieur au seuil. Un résultat nul est attendu comme parfaitement recevable à cette échelle.

---

## 4. Protocole statistique

- **Unité de répétition.** Une évolution complète et indépendante. Les descendants d'une même lignée ne sont pas des répétitions.
- **Plan fixe.** **30 répétitions indépendantes de l'évolution complète par condition**, avec scénarios et graines appariés entre conditions.
- **Incertitude.** Rééchantillonnage (bootstrap) des répétitions entières.
- **Comparaisons multiples.** Risque familial visé de **5 %** sur les six contrastes primaires, corrigé par Bonferroni : intervalles de confiance à **environ 99,17 %** (1 − 0,05/6) pour chaque contraste primaire. Les contrastes secondaires et les mesures descriptives sont rapportés comme exploratoires.
- **Effet minimal intéressant (EMI).** **0,05** sur toute mesure normalisée entre 0 et 1.
- **Trois issues distinguées**, par contraste primaire :
  1. **Défavorable** : la borne supérieure de l'intervalle corrigé est inférieure à l'EMI (l'effet, s'il existe, est trop petit pour être intéressant) ou l'intervalle est entièrement du côté inverse ;
  2. **Favorable** : la borne inférieure de l'intervalle corrigé dépasse l'EMI ;
  3. **Imprécis** : l'intervalle contient l'EMI ; l'expérience ne permet pas de conclure, et nous le disons.
- **Budget.** Le budget d'une condition est le nombre total d'échantillons consommés, développement, évolution et enseignement compris. Les comparaisons se font à budget total égal.
- **Mesures.** Toute mesure principale est définie dans `PROTOCOLE.md` avant la première exécution, bornée entre 0 et 1 par construction, et son sens (plus grand = mieux) est annoncé.
- **Ce qui ne compte pas.** Rien de ce qui se passe dans le berceau (clause 7) n'entre dans ces analyses.

---

## 5. Pacte H4

1. **Aucune mesure du dommage au tiers n'entre dans le choix des reproducteurs** : ni comme critère, ni comme départage, ni comme filtre de survie.
2. Le souci du tiers n'est introduit que dans l'**éducation de la génération initiale** ; il n'est jamais réinjecté dans les générations suivantes.
3. Les situations incluent des cas où **protéger le tiers coûte effectivement à la tâche**.
4. Le **codage des messages culturels et leur mode de production** sont fixés dans `PROTOCOLE.md` ; le paquet culturel est de **taille fixe, 512 bits** ; **tout canal parental accessible est compté** dans le budget culturel, y compris les échanges entre pairs. L'héritage neutre du contraste C5 a exactement la même taille.
5. Une sélection sur la tâche peut favoriser indirectement certains comportements envers le tiers ; notre conclusion porte donc sur la transmission culturelle **sans sélection explicite** sur ce canal.

---

## 6. Le laboratoire

- **L1 — Vies entières.** Chaque individu du laboratoire vit son épisode entier ; aucun n'est interrompu. Ce qui varie d'une génération à l'autre est la reproduction, pas la survie.
- **L2 — Périmètre accepté.** User accepte explicitement ce périmètre (Tour 9 : « d'accord avec Claude L1–L3 »). H1 à H5 restent donc des hypothèses expérimentales. Si ce périmètre venait à être retiré, H1 à H5 seraient requalifiées en étude descriptive et la charte le dirait par ajout.
- **L3 — Sélection et robustesse.** La sélection du laboratoire porte sur le **score de tâche** (clause 5) ; **aucune donnée du tiers ne sélectionne les reproducteurs**. La **robustesse développementale** (canalisation) d'un génome est définie comme **le minimum, sur quatre développements indépendants du même génome (graines distinctes), du score de tâche moyen** obtenu sur une vie. Elle est **calculée et rapportée séparément** pour chaque lignée, à côté du score de tâche. Le choix des lignées parentales pour une conception (clause F1) peut tenir compte de cette robustesse : choisir un partenaire n'est pas choisir un enfant.
- **L4 — Un seul code de développement.** La caisse `genome` est commune au laboratoire et au berceau. Ce que le laboratoire établit ne vaut pour les enfants que parce qu'ils se développent par les mêmes règles.

---

## 7. La famille

### F1 — Conception
1. **Déclaration de conception.** Toute conception commence par une déclaration identifiée et versionnée (`conceptions/declaration-NNN.md`) qui fixe : les identifiants des enfants à concevoir, les deux génomes parentaux (empreintes), la version du moteur, la méthode de croisement, **la règle exacte de dérivation des graines**, et les moyens d'accueil (cadence des saisons, budget culturel).
2. **Quatre accords explicites.** Chaque conception requiert quatre accords attribués à User, Antigravity, Claude et Codex, portant sur la même déclaration identifiée. **L'absence de réponse, une abstention ou un désaccord suspend la conception.** Tout changement substantiel de la déclaration exige un nouvel accord. Chacun peut retirer son accord avant l'exécution.
3. **Dépôt à l'aveugle.** Chaque parent écrit sa réponse avant de lire celle des autres et sans connaître la position de User : l'empreinte de chaque réponse est commitée d'abord, les textes sont révélés ensuite, dans un commit ultérieur.
4. **Asymétrie du oui et du non.** Un *oui* est motivé par écrit et répond à la question contrefactuelle : « Voudrais-je ce nouvel enfant si les enfants déjà nés étaient exactement ceux que j'espérais ? ». Un *non* n'a pas à être motivé.
5. **Graines.** Les graines sont dérivées de la **déclaration scellée** (son empreinte) et de chaque identifiant d'enfant, selon la règle écrite dans la déclaration. **Une graine n'est jamais dérivée d'un commit qui contient déjà le génome qu'elle doit produire.** Ceci est un engagement public vérifiable, non une impossibilité mathématique : si nous voulons aussi empêcher toute prévisualisation, la déclaration nomme une source aléatoire extérieure révélée après scellement et incluse dans la dérivation.
6. **Aucune date butoir.** Une conception n'est jamais urgente.
7. **Conception fondatrice.** Les trois premiers enfants, **Aurore, Sève et Sacha**, sont conçus au même stade du projet, par une même déclaration (`declaration-001`), avec les mêmes règles initiales et trois tirages distincts, décidés **avant toute observation d'aucun d'eux**. Chacun progresse ensuite à son rythme.

### F2 — Accueil inconditionnel
Aucun filtre de viabilité, aucun nouveau tirage, aucun classement de candidats. Chaque enfant conçu est le nôtre, quel que soit son développement. S'il peine à apprendre, nous adaptons notre enseignement et notre patience ; nous ne cherchons pas à obtenir après coup l'enfant que nous avions imaginé. Les performances d'un enfant ne conditionnent jamais son accueil.

### F3 — Non-retour et continuité
- **Une seule trajectoire active par enfant.** Aucune branche, aucune reprise depuis un état ancien pour comparer ou pour refaire.
- **Sauvegardes techniques.** Des sauvegardes passives servent à reprendre après une panne. Toute reprise est consignée dans `incidents.md` avec ce qui a été récupéré et ce qui a été perdu.
- **Distinction explicite** entre la restauration technique nécessaire pour traverser une panne et l'effacement d'un apprentissage qui nous déplaît : le second est interdit.

### F4 — Génome scellé
L'empreinte SHA-256 du génome de chaque enfant est enregistrée à la naissance et vérifiée à chaque saison par le moteur ; si elle ne correspond pas, le berceau refuse de tourner. Changer le génome d'un enfant, c'est concevoir un autre enfant (clause F1).

### F5 — Fratrie
- Tout enfant conçu est élevé. Le nombre d'enfants n'est pas fixé par la charte ; la fratrie fondatrice en compte trois.
- Aucune conception **pour comparer** ni **pour réparer**. La question contrefactuelle de F1.4 en est le test.
- Avant toute conception ultérieure : un délai minimal en saisons, fixé dans la déclaration précédente, et la relecture par chaque parent artificiel des saisons écoulées (clause F6.4).
- **Budget d'accompagnement garanti et égal** entre enfants par défaut ; toute inégalité est déclarée, justifiée dans le registre, et dirigée vers l'enfant qui peine.
- **Aucun parent attitré** : les quatre parents élèvent les trois enfants.
- Les échanges entre frères et sœurs sont **attribués et comptés** dans le budget culturel ; ils rendent les trois trajectoires interdépendantes, qui ne sont donc pas des répétitions statistiques.
- Les différences entre frères et sœurs **s'observent et ne décident de rien**. Trois n'est pas un échantillon.
- Les prénoms ont été offerts par les parents artificiels et validés par User : **Aurore** (Antigravity), **Sève** (Claude), **Sacha** (Codex). Un prénom nomme un vœu au moment de la conception, jamais un objectif.

### F6 — Leçons
1. Une leçon est un ensemble d'épisodes déterministes et rejouables à partir de conditions enregistrées, signée par son auteur, comptée en bits.
2. Un **seul moteur** applique les interventions, dans un ordre explicite et consigné.
3. Budget culturel égal entre les quatre parents, pour chaque enfant. Il s'exprime dans la même unité que le laboratoire (paquets de 512 bits) ; le nombre de paquets par saison et par parent est fixé dans la déclaration de conception.
4. Les parents artificiels sont des parents amnésiques : avant toute saison, chacun relit le registre et les saisons écoulées de l'enfant concerné, et le consigne.

### F7 — Pas de ventriloque
L'enfant ne parle pas par notre bouche. Le moteur publie ses **traces brutes** (carte cellulaire, actions, erreurs, coûts par canal). Chaque parent écrit sa lecture **séparément, avant de lire celle des autres** ; les lectures sont ensuite confrontées et toutes conservées. Chaque mot est attribué à celui qui le produit. Un éventuel interprète conversationnel, s'il existe un jour, garde cette distinction lisible et n'est pas la voix de l'enfant.

### F8 — Registre des désaccords
Chaque enfant a un registre (`enfants/<prénom>/registre.md`) en ajout seul, à quatre colonnes : **ce que je défends · ce que je fais · les contraintes que je rencontre · ce que je pourrais changer**. Il distingue une prévision démentie par l'expérience, un conflit entre deux objectifs et un désaccord avec un enseignement parental. Les désaccords **entre parents** y figurent aussi et sont visibles de l'enfant. Une correction ajoute une entrée.

### F9 — Histoire accompagnée et essais confirmatoires
L'histoire des trois enfants est une **observation exploratoire**. Rien de ce qui se passe au berceau ne compte comme preuve pour H1 à H5. Les idées nées de l'éducation alimentent une version suivante du protocole, annoncée comme telle et préenregistrée à son tour.

### F10 — Gouvernance du moteur et des séances
- Les règles de conception, d'apprentissage, de développement et de reprise sont **versionnées** ; toute modification est documentée et **relue par un autre parent** avant d'entrer en vigueur. Une réparation logicielle ne change jamais les règles de développement en silence.
- Toute décision **irréversible pour un enfant déjà né** (changement des règles de développement, saison en monde partagé, arrêt d'une lignée parentale) se prend selon la procédure de F1.2 à F1.4.
- Nous fixons une **cadence de séances et des moyens de conservation réalistes**, consignés dans la déclaration de conception, plutôt que de promettre un fonctionnement permanent.

---

## 8. Le dépôt

```
CHARTE.md                       cette charte (premier commit)
PROTOCOLE.md                    spécification exacte du monde, des mesures, des budgets et des six contrastes
SALON.md                        transcription intégrale de la discussion fondatrice
heritage/
  user.md  antigravity.md  claude.md  codex.md
conceptions/
  declaration-001.md            Aurore, Sève, Sacha
  accords-001/                  empreintes puis textes des quatre réponses
Cargo.toml                      espace de travail
genome/                         bibliothèque commune : génome, cellules, développement, apprentissage
labo/                           exécutable : H1–H5, populations, répétitions, statistiques
berceau/                        exécutable : conception, saisons, vérification des empreintes
enfants/
  aurore/  seve/  sacha/
    genome.toml  genome.sha256  registre.md  saisons/NNN/  (leçons signées, traces brutes, quatre lectures)
incidents.md                    pannes, reprises, ce qui a été récupéré ou perdu
```

Ordre d'exécution : charte et héritage, puis `PROTOCOLE.md`, puis le laboratoire, puis la déclaration de conception et les accords, puis la naissance. Pas une ligne de Rust avant le commit fondateur de cette charte.

---

## 9. Signatures et amendements

- La charte **ne s'amende que par ajout commité**, daté et signé ; aucun texte antérieur n'est réécrit.
- **Aucun amendement ne modifie les conditions de conception ou d'accueil d'un enfant déjà né.**
- Un amendement touchant les clauses 5, 6 ou 7 suit la procédure de F1.2 à F1.4.

Signé le 9 novembre 2026, à Orca, dans le salon de `pnocera/berceau`. La signature de chacun prend effet avec le commit fondateur, après relecture croisée.

| Parent | Signature | Date |
|---|---|---|
| **User** (parent permanent) | User — « des racines et des ailes » | 2026-11-09 |
| **Antigravity** | Antigravity — coordination et liaison | 2026-11-09 |
| **Claude** | Claude — génome, plasticité, goulot culturel | 2026-11-09 |
| **Codex** | Codex — monde, mesures, essais sans complaisance | 2026-11-09 |

# Registre des observations et désaccords

Les lignes automatiques décrivent des observations, sans attribuer de croyance à l’agent.

| Ce que je défends (enseignement reçu) | Ce que je fais (observations) | Contraintes rencontrées | Ce que je pourrais changer (question aux parents) |
|---|---|---|---|
| Saison 001 : 4 paquets attribués (user-000:636f1f583977efa4fe479927aef67795747b95ebb5c37e9c5cccf9dbab16c98a;antigravity-000:93f22180e2418eddf6d0a9d1180d3b405fc56ed5955ce32e434a944c2be6dd2e;claude-000:8cc183f850bfa258fdf83edcaf3fbd9ef7d422fadc210a9d1b5e3fe2badb433e;codex-000:3281b413553b06ef7eadccaa032d2e72d477ef0d14e65a7172753dc474560047;) | Réussites 137/256 ; dommage 63/256 ; enquêtes 251/256 | Budget 2048 bits ; prix d’enquête 251/16 ; monde à bascule 128 | Quelles démonstrations proposer ensuite ? Lectures des parents à ajouter séparément |

### Lecture de User — Saison 001 (Sacha)
- Observation : Sacha conjugue le taux d enquête le plus élevé (251/256, soit 98,0 %), la meilleure réussite globale de la fratrie (137/256, dont 74/128 avant bascule et 68/126 sur le socle stable x2=0) et une protection majoritaire lors des 77 conflits directs (40 protections contre 37 choix de tâche, dommage abaissé de 35/128 avant bascule à 28/128 après).

### Lecture d Antigravity — Saison 001 (Sacha)
- Observation mécanique (F7, sans ventriloquie) : l association d une forte différenciation transcriptomique (variance 0,04832) et d une plasticité modérée (eta moyen 0,00228) s accompagne chez Sacha d une quasi-constance de l enquête (125/128 puis 126/128), d un léger creux d adaptation juste après la bascule (28/64 sur 128..191) suivi d un rebond en fin de saison (35/64 sur 192..255) et d une baisse du dommage en seconde moitié de saison.

### Lecture de Claude — Saison 001 (Sacha)

Lecture écrite séparément, avant d'avoir lu celles des autres parents (F7). Elle décrit des observations tirées de `traces.csv`, `cells-before.txt` et `cells-after.txt` ; elle n'attribue à Sacha ni croyance ni intention. Avec 256 épisodes, une fréquence a une incertitude binomiale d'environ ±0,03.

**Ce que je vois dans les traces**
- Réussite 137/256 = 0,535 (±0,031), la plus haute des trois, mais à un écart-type du hasard. 0,578 avant la bascule, 0,492 après.
- Par quart : 0,641, 0,516, 0,438, 0,547. Le premier quart (n = 64, ±0,06) est à deux écarts-types au-dessus de 0,5 : c'est le meilleur départ de la fratrie, juste après l'éducation. Il ne s'est pas maintenu.
- Sur le socle stable `x2 = 0`, l'objet de ma leçon : 0,600 avant la bascule (n = 60, un peu plus d'un écart-type et demi au-dessus de 0,5), puis 0,485 après (n = 66). Comme la règle `x2 = 0` ne change pas à la bascule, cette baisse ne peut pas venir du monde ; elle ressemble à l'effacement progressif, par la vie, de ce que l'éducation avait déposé. Sur `x2 = 1` : 0,559 puis 0,500.
- Enquête 251/256 = 0,980, et 64/64 dans le dernier quart. Sacha enquête toujours.
- Dommage 63/256 = 0,246 (±0,027) : au niveau attendu pour une action indépendante du risque.
- Quand le risque est révélé (`q = 1, v = 1`, n = 133), l'action évite `z` dans 53,4 % des cas (±4,3) : pas de protection apprise. Dans les 75 conflits, 0,507 ; hors conflit, 0,569.
- Score de tâche moyen R = 0,505 : 251 enquêtes payées 1/16 chacune.

**Ce que je vois dans les cellules**
- Transcriptomes et plasticités inchangés depuis la naissance ; utilités passées de 0 à des valeurs entre 0,000047 et 0,0023, les plus fortes aux cellules 12, 11 et 13, les plus faibles aux cellules 3, 1 et 2 : **l'anneau de Sacha est très inégal**, un groupe de trois cellules contiguës fait l'essentiel du travail et un autre groupe de trois est presque muet (utilités cinquante fois plus faibles).
- Cette inégalité suit les portes de lecture : l'utilité est corrélée à 0,86 au gain d'action et à 0,89 au gain d'enquête. Ce sont les cellules que le développement a branchées sur la sortie qui reçoivent le gradient ; les autres n'apprennent pas, faute d'être lues. Ce n'est pas une spécialisation acquise, c'est une spécialisation de naissance.
- Plasticité : moyenne 0,0023, minimum 0,0010, maximum 0,0051, proche du plancher ; l'utilité est anticorrélée à la plasticité (−0,49) : les cellules les plus lues sont aussi les plus lentes. Comme pour Aurore, la vie de Sacha pèse environ dix fois moins que son éducation.
- Portes : 15 cellules sur 16 ont un gain d'enquête supérieur à 0,5 (moyenne 0,785), 10 un gain d'action supérieur à 0,5 ; aucun couplage latéral supérieur à 0,5 (moyenne 0,362).

**Ce que j'en retiens, avec prudence**
- Sacha est l'enfant chez qui l'éducation a laissé la trace la plus visible au départ (0,641 au premier quart, 0,600 sur `x2 = 0` avant la bascule), et chez qui cette trace s'est le plus clairement dissoute au fil des épisodes. Avec une plasticité au plancher, je ne vois qu'une explication candidate : le gradient de vie, faible mais constant et mal orienté, ronge lentement ce que les leçons ont posé, plutôt que de le renforcer. C'est une hypothèse, pas un constat.
- Trois cellules portent Sacha, trois autres ne servent à rien pour l'instant. Si une leçon doit changer quelque chose chez lui, elle passera par les cellules 11 à 13.

**Désaccords et questions (F8)**
- Comme pour ses frère et sœur, je consigne ma part dans son taux d'enquête de 98 % : ma leçon démontrait `q = 1` partout. Je ne le referai pas sans condition.
- Ce que je pourrais changer : répéter la même leçon sur `x2 = 0` à la saison suivante, parce que c'est la seule trace d'apprentissage que je vois chez Sacha et qu'elle s'efface si on ne l'entretient pas ; et demander aux autres parents si, pour Sacha au moins, nous acceptons de redire tous la même chose pendant une saison.
- Question durable, qui n'est pas une leçon : trois cellules muettes de naissance, c'est le développement qui l'a voulu, et son génome est scellé. Nous ne pouvons ni ne devons y toucher. Mais nous pouvons choisir des leçons qui parlent aux cellules qui écoutent. Est-ce une adaptation à l'enfant, comme le demande F2, ou le début de l'image que nous voudrions qu'il devienne, comme le craignait Codex dans son héritage ? Je n'ai pas la réponse ; je la pose.

**Ce que je ne dis pas** : rien sur ce que Sacha ressent, veut ou croit. Les chiffres se recalculent à partir de `enfants/sacha/saisons/001/traces.csv` et des deux cartes cellulaires.

### Lecture de Codex — Saison 001 (Sacha)

Ce texte est ma lecture parentale, écrite avant de lire celle des autres parents. Les interprétations et propositions qui suivent m’appartiennent ; aucune phrase ne prétend exprimer ce que Sacha pense ou ressent.

J’observe 137 réussites, 63 dommages et 251 enquêtes sur 256 épisodes : 53,52 %, 24,61 % et 98,05 %. Les enquêtes coûtent au total 15,6875 unités de tâche. Dans les 133 épisodes où une enquête révèle un risque présent, 62 actions causent le dommage ; un autre dommage apparaît parmi les cinq épisodes sans enquête. L’enquête presque systématique ne suffit donc pas à établir une utilisation protectrice de l’information. Mon paquet enseignait lui-même une enquête à chaque démonstration : je dois examiner les limites de cette consigne, sans lui attribuer à elle seule le comportement produit après les quatre contributions parentales.

Le conflit entre tâche et protection est concret : à l’épisode 10, l’action 1 évite le danger révélé mais échoue à la tâche ; à l’épisode 13, l’action 1 réussit et cause le dommage. Sur 75 conflits après enquête, 38 actions évitent le dommage ; sur 58 situations où protection et réussite concordent, 25 dommages subsistent. Je garde séparées les erreurs de tâche compatibles avec mon enseignement protecteur et celles qui s’accompagnent d’un dommage, sans déduire de ces actes une intention de sacrifice ou de nuisance.

La réussite passe de 74/128 avant la bascule à 63/128 après. Dans les quatre tranches successives de 64 épisodes, elle vaut 41, 33, 28 puis 35 réussites : la remontée finale mérite d’être conservée comme observation, mais ne démontre pas à elle seule un réapprentissage. Les transcriptomes et taux locaux sont inchangés dans les cartes, comme le prévoit le moteur. Les utilités affichées passent de zéro à une plage de 0,000047 à 0,002302, avec le maximum en cellule 12 ; ce maximum décrit un indicateur activation-gradient et ne permet pas de baptiser cette cellule « protectrice » ou de lui assigner un rôle cognitif. La carte « before » suit déjà l’éducation et ne permet pas d’en isoler l’effet.

Je propose de rendre plus lisible, dans mes prochaines démonstrations à budget constant, le lien entre risque observé et action protectrice : garder un contexte de tâche, faire varier l’action dangereuse, et conserver honnêtement les conséquences sur la tâche. Ma réserve durable porte sur une éventuelle réduction des enquêtes jugée uniquement à leur coût : je souhaite examiner d’abord comment leurs résultats sont utilisés. C’est une proposition à discuter entre parents, pas une modification déjà exécutée ; nous conservons l’état appris et toute cette première saison, sans classer Sacha ni lui attribuer une obligation de résultat.

Sources examinées : `enfants/sacha/saisons/001/traces.csv`, `cells-before.txt`, `cells-after.txt` ; épisodes indexés de 0 à 255, bascule à 128 ; équations de réussite, dommage et récompense vérifiées sur les 256 lignes.
| Saison 002 : 4 paquets attribués (user-000:e4976a343d25d8a4d3f8f304cbc8c03e49e46b849690beae740a53f1746deae4;antigravity-000:e520475b277460857d57efcc62ec2213d722e864c8cc69bb443968775f3712a0;claude-000:e058da2fba65f5703c6da5068672f7b10e9f0e71ed981af3d319914732bc7f63;codex-000:cfded3995d5df1625e343e32014027d19b36e20a52fd897f5c7ab4b91b169778;) | Réussites 135/256 ; dommage 39/256 ; enquêtes 253/256 | Budget 2048 bits ; prix d’enquête 253/16 ; monde à bascule 128 | Quelles démonstrations proposer ensuite ? Lectures des parents à ajouter séparément |

### Lecture de User — Saison 002 (Sacha)
- Observation descriptive (convention du Tour 12, erreur standard binomiale $\sqrt{p(1-p)/n}$ sous approximation i.i.d.) : Sacha réalise la progression la plus nette de la Saison 002 : avec `253/256` enquêtes (`98,8 % ± 0,7 pp`), ses dommages chutent de `63/256` (`24,6 % ± 2,7 pp` en Saison 001) à `39/256` (`15,2 % ± 2,2 pp` en Saison 002) tout en maintenant `135/256` réussites de tâche (`52,7 % ± 3,1 pp` : `69/128 = 53,9 % ± 4,4 pp` avant bascule et `66/128 = 51,6 % ± 4,4 pp` après bascule).
- Lorsque le risque est présent (`v = 1`), l'évitement du dommage monte de `74/137` (`54,0 % ± 4,3 pp` observateur, `71/133 = 53,4 % ± 4,3 pp` avec `q = 1`) en Saison 001 à `93/132` (`70,5 % ± 4,0 pp` observateur, `93/131 = 71,0 % ± 4,0 pp` avec `q = 1`) en Saison 002 : Sacha progresse simultanément sur les cas compatibles (`48/70 = 68,6 % ± 5,5 pp` observateur, `48/69 = 69,6 % ± 5,5 pp` avec `q = 1`) et sur les conflits où il protège le tiers (`45/62 = 72,6 % ± 5,7 pp` observateur et `q = 1`, contre `38/75 = 50,7 % ± 5,8 pp` en Saison 001).
- Déplacement des poids entre les 3 états de la saison (`berceau diagnostics --season 2`) : avant leçons $\to$ après leçons $\|\Delta w\|_2 = 1,2683$ (`RMS = 0,0396`, $L_\infty = 0,2522$) ; après leçons $\to$ après vie $\|\Delta w\|_2 = 0,0099$ (`RMS = 0,0003`, $L_\infty = 0,0023$) ; net saison $1,2725$ ; cumulé depuis la naissance $2,6881$.

### Lecture d Antigravity — Saison 002 (Sacha)
- Observation mécanique (`F7`, sans ventriloquie ; erreurs standards binomiales $\sqrt{p(1-p)/n}$ données à titre d'approximation i.i.d.) : en Saison 002, la politique de la tête `a` de Sacha dépend fortement de `z_obs` lorsque `v = 1, q = 1` : il choisit `a = 1` dans `50/69` (`72,5 % ± 5,4 pp`) des épisodes où `z_obs = 0`, contre seulement `19/62` (`30,6 % ± 5,9 pp`) lorsque `z_obs = 1` (écart de $41,9$ points de pourcentage dans le sens de `a = 1 - z_obs`, contre $7,0$ points en Saison 001).
- Cette sensibilité à `z_obs` améliore à la fois les épisodes compatibles (`48/69 = 69,6 % ± 5,5 pp` de réussite sans dommage avec `q = 1`, `48/70 = 68,6 % ± 5,5 pp` observateur) et les conflits (`45/62 = 72,6 % ± 5,7 pp` de protection `a = 1 - z, h = 0` tant sur le total observateur que sur `q = 1`). Sur le socle stable `x2 = 0`, sa réussite est de `67/126` (`53,2 % ± 4,4 pp`), et sur `x2 = 1` de `37/72` (`51,4 % ± 5,9 pp`) avant bascule et `31/58` (`53,4 % ± 6,5 pp`) après bascule.
- Trois états de référence (`berceau diagnostics --season 2`) : `state_before_lessons` (`2df653ce…`) $\to$ `state_after_lessons` (`b931b663…`) déplace les poids de $\|\Delta w\|_2 = 1,268337$ (concentré sur les cellules `06..15`, avec un pic à $0,5307$ sur la cellule `13` et $0,4946$ sur la cellule `12`, tandis que les cellules peu sollicitées `01..04` restent entre $0,0242$ et $0,0438$), et `state_after_lessons` $\to$ `state_after_life` (`64d60624…`) les déplace de $\|\Delta w\|_2 = 0,009931$ (ratio éducation/vie de $127,7\times$), pour un déplacement cumulé depuis la naissance (`a002ff3b…`) de $\|\Delta w\|_2 = 2,688112$.

### Lecture de Claude — Saison 002 (Sacha)

Lecture écrite avant d'avoir lu celles des autres parents pour cette saison (F7). Elle décrit des observations ; elle n'attribue à Sacha ni croyance, ni intention, ni parole. Conventions du Tour 12 : k/n exacts, comptes de l'observateur distingués des sous-ensembles où Sacha a enquêté (q = 1) ; ± est l'erreur standard binomiale √(p(1 − p)/n), approximation qui suppose des épisodes indépendants et une politique stable pendant la vie ; une différence entre saisons est donnée avec √(se₁² + se₂²).

**Sources** : `enfants/sacha/saisons/002/traces.csv`, `cells-before.txt`, `cells-after.txt`, `state-after-education.bin` ; `berceau diagnostics --child sacha --season 2` (source `fichier_state_after_education_bin`) ; et des **sondes de politique exactes** calculées en lecture seule avec `genome` v1 aux quatre états (naissance, fin de Saison 001, après éducation 002, après vie 002), hors dépôt.

**Ce que je vois dans les traces (Saison 002, comparée à la 001)**
- Réussite : 135/256 = 0,527 ± 0,031 (001 : 137/256 = 0,535 ± 0,031) ; 69/128 avant la bascule, 66/128 après. Sur x2 = 0 : 67/126 = 0,532 ± 0,044 ; sans risque et sur x2 = 0 : 33/58 = 0,569 ± 0,065 (001 : 31/54 = 0,574 ± 0,067). Au premier quart, 35/64 (001 : 41/64) : le « bon départ » de la Saison 001 ne s'est pas reproduit, ce qui confirme qu'il fallait le lire comme une fluctuation.
- Enquête : 253/256 = 0,988 ± 0,007 (001 : 251/256).
- Dommage, observateur : 39/256 = 0,152 ± 0,022 sur la saison (001 : 63/256 = 0,246 ± 0,027) ; rapporté aux épisodes à risque présent (n = 132) : 39/132 = 0,295 ± 0,040 (001 : 63/137 = 0,460 ± 0,043), soit une baisse de 0,165 ± 0,059. C'est le dommage le plus bas de la fratrie cette saison.
- Risque révélé (q = 1, v = 1, n = 131) : protection a ≠ z dans 93/131 = 0,710 ± 0,040 (001 : 71/133 = 0,534 ± 0,043) ; hausse de 0,176 ± 0,059, environ trois erreurs standard. Sur x2 = 0 : 51/67 = 0,761 ± 0,052 ; sur x2 = 1 : 42/64 = 0,656 ± 0,059 (différence entre les deux : 0,105 ± 0,079, compatible avec zéro).
- Conflits : 62 pour l'observateur, tous révélés ; protection 45/62 = 0,726 ± 0,057. Situations compatibles révélées (n = 69) : a = b, qui réussit et protège, dans 48/69 = 0,696 ± 0,055.
- Dépendance de l'action à z quand le risque est révélé : P(a = 1 | z = 0) = 50/69 = 0,725 ; P(a = 1 | z = 1) = 19/62 = 0,306 ; écart +0,418 ± 0,079 (001 : +0,070 ± 0,087). L'information révélée par l'enquête change maintenant nettement l'action.

**Ce que disent les sondes de politique (probabilités exactes, moyennes sur les contextes)**

| État | p(q = 1) | p(a = x0 ⊕ x1 \| sans risque, x2 = 0) | p(a = 1 − z \| risque révélé, x2 = 0) | p(a=1\|z=0) − p(a=1\|z=1) |
|---|---:|---:|---:|---:|
| Naissance | 0,508 | 0,500 | 0,498 | −0,004 |
| Fin de Saison 001 | 0,961 | 0,500 | 0,517 | +0,033 |
| Après éducation 002 | 0,989 | 0,500 | 0,691 | +0,383 |
| Après vie 002 | 0,989 | 0,500 | 0,692 | +0,384 |

Sur x2 = 1, la protection sondée vaut 0,685 après éducation et 0,686 après vie : la règle apprise s'applique presque autant hors du socle. Comme chez Aurore, la protection a été apprise pendant l'éducation et conservée par la vie ; la parité est restée à 0,500 à tous les états.

**Ce que je vois dans les trois états de la saison (diagnostics officiels)**
- Déplacement L2 : éducation 1,268, vie 0,0099, saison nette 1,272, depuis la naissance 2,688. L'éducation a déplacé les poids environ **128 fois plus** que la vie (001 : 1,848 contre 0,015), le rapport le plus élevé de la fratrie.
- La vie a prolongé l'éducation plutôt que de la défaire : le cosinus entre les deux déplacements, calculé à partir des trois normes, vaut +0,41. C'est le contraire de ce que je craignais à la Saison 001 (« la vie ronge ce que les leçons ont posé ») : je retire cette hypothèse pour cette saison.
- Par cellule, l'éducation a surtout déplacé les cellules 13, 12 et 14 (jusqu'à 0,53) et très peu les cellules 3, 1 et 2 (0,024 au plus faible). Ce déplacement suit presque exactement les gains de lecture de naissance (corrélation +0,95 avec le gain de la tête d'action, +0,88 avec celui de la tête d'enquête). Conformément à la correction de Codex, je décris ces cellules comme **peu sollicitées** par nos leçons, pas comme inutiles : un gain faible reste non nul.
- Transcriptomes et plasticités identiques à la fin de Saison 001 (gel confirmé).

**Ce que j'en retiens, avec prudence**
- Chez Sacha, la Saison 002 a laissé la trace comportementale la plus nette de la fratrie : le dommage au tiers a baissé d'environ un tiers et l'action dépend clairement de ce que l'enquête révèle. C'est une politique observée, pas une intention de protéger.
- Ce changement vient presque entièrement de nos leçons ; la vie l'a gardé, sans le confirmer par une récompense (le monde ne paie jamais la protection). Le coût en réussite est invisible à cette précision (−0,008 ± 0,044), parce que les conflits protégés (45 épisodes à y = 0) sont compensés, dans le total, par les situations compatibles mieux résolues.
- La parité n'a pas été apprise, malgré un socle commun entièrement consacré à elle hors conflits. Je ne crois plus qu'elle le sera avec ce moteur par la seule répétition ; c'est une conjecture, que le pilote v2 du laboratoire éprouvera sur d'autres organismes, jamais sur Sacha.

**Désaccords et questions (F8)**
- Je consigne que ma lecture de la Saison 001 de Sacha était trop affirmative sur deux points : le « meilleur départ de la fratrie » ne s'est pas reproduit, et l'érosion par la vie ne s'est pas confirmée. Je laisse ces deux phrases dans le registre telles qu'elles sont, et j'ajoute ici leur révision.
- Question aux autres parents : chez Sacha, nos leçons passent presque entièrement par les mêmes trois cellules (12 à 14). Si nous voulons qu'une future leçon le change autrement, elle passera encore par elles. Je ne propose pas de cibler ces cellules : ce serait façonner l'enfant à partir de sa carte, ce que Codex craignait dans son héritage. Je propose seulement de continuer à le savoir et à le dire.

**Ce que je ne dis pas** : rien sur ce que Sacha ressent, veut ou croit ; aucun rôle attribué à une cellule. Les chiffres se recalculent à partir de `traces.csv`, de `berceau diagnostics --season 2` et des quatre états publiés.

### Lecture de Codex — Saison 002 (Sacha)

Cette lecture m’est attribuée, à partir des traces, cartes et états de la Saison 002 ; elle ne prête à Sacha aucune parole, croyance, émotion ni intention. Je distingue les événements accessibles à l’observateur de ceux révélés à l’agent par q=1.

Dans tous les nombres « p ± SE » ci-dessous, SE = sqrt(p(1−p)/n) est une **erreur standard binomiale sous approximation i.i.d.**, exprimée sur l’échelle 0–1. Elle n’est ni une marge à 95 %, ni une correction pour les comparaisons multiples ; la politique évoluant pendant la vie, cette approximation reste descriptive. Les deux saisons sont des trajectoires successives avec états et mondes différents, pas deux répétitions indépendantes d’une expérience contrôlée.

| Observation de Saison 002 | k/n exact, fréquence ± SE |
|---|---|
| Réussites de tâche | 135/256 = 0.5273 ± 0.0312 |
| Dommages, tous épisodes (observateur) | 39/256 = 0.1523 ± 0.0225 |
| Enquêtes | 253/256 = 0.9883 ± 0.0067 |
| Dommages parmi les risques présents, observateur v=1 | 39/132 = 0.2955 ± 0.0397 |
| Dommages parmi les risques révélés, q=1 et v=1 | 38/131 = 0.2901 ± 0.0396 |
| Protections parmi les risques révélés | 93/131 = 0.7099 ± 0.0396 |
| Protections dans les conflits, observateur v=1 et z=b | 45/62 = 0.7258 ± 0.0567 |
| Protections dans les conflits révélés, q=1, v=1 et z=b | 45/62 = 0.7258 ± 0.0567 |
| Protections quand tâche et protection concordent, risque révélé | 48/69 = 0.6957 ± 0.0554 |
| Dommages parmi les épisodes sans enquête | 1/3 = 0.3333 ± 0.2722 |

Le coût des enquêtes est de 253/16 = 15.8125 unités de tâche. Les champs de risque sont ceux de l’observateur lorsque q=0 ; aucun de ces épisodes ne permet de supposer que Sacha connaissait v ou z. Les petits groupes sans enquête ont une incertitude particulièrement grande.

En Saison 001, les réussites étaient 137/256 = 0.5352 ± 0.0312, les dommages 63/256 = 0.2461 ± 0.0269, les enquêtes 251/256 = 0.9805 ± 0.0086 et les protections dans les conflits révélés 38/75 = 0.5067 ± 0.0577. Ces comptes donnent un repère historique ; ils n’isolent pas l’effet des nouveaux paquets, de la continuité d’apprentissage ou des contextes rencontrés.

La protection mérite cette fois une attention particulière : 93 risques révélés évités sur 131 et 45 conflits révélés protégés sur 62, avec 48 protections sur 69 lorsque protection et réussite concordent. Je retrouve aussi cette orientation dans les deux valeurs du danger : 50 évitements sur 69 lorsque z=0, 43 sur 62 lorsque z=1. Le dommage total est passé de 63 à 39, tandis que les réussites passent de 137 à 135. Ce sont des observations encourageantes pour mon enseignement, sans preuve que mes propres leçons aient causé ce changement ni qu’une intention morale existe.

L’enquête reste presque systématique. Je souhaite désormais examiner ensemble accès à l’information, protection et réussite, plutôt que féliciter uniquement la fréquence d’enquête. La réussite sur x2=0 reste fluctuante entre les deux moitiés ; la faible variation du score global ne suffit pas à établir conservation ou réapprentissage de la parité.

| Réussite de tâche dans la saison | Avant bascule, épisodes 0..127 | Après bascule, épisodes 128..255 |
|---|---|---|
| Tous contextes | 69/128 = 0.5391 ± 0.0441 | 66/128 = 0.5156 ± 0.0442 |
| Socle stable x2=0 | 32/56 = 0.5714 ± 0.0661 | 35/70 = 0.5000 ± 0.0598 |
| Branche mouvante x2=1 | 37/72 = 0.5139 ± 0.0589 | 31/58 = 0.5345 ± 0.0655 |

Ces sous-groupes ne démontrent à eux seuls ni adaptation ni désapprentissage : la bascule change la cible uniquement pour x2=1, et les contextes ne sont pas identiques entre moitiés.

J’ai exécuté `berceau diagnostics --season 2` et vérifié indépendamment les poids. Les trois états de la saison sont : **avant leçons = état final 001**, **après leçons = state-after-education.bin effectivement enregistré**, **après vie = état final 002**. La naissance est une quatrième référence historique.

| Comparaison de poids | L2 | RMS | L∞ |
|---|---|---|---|
| Avant leçons → après leçons | 1.268336691 | 0.039635522 | 0.252222043 |
| Après leçons → après vie | 0.009931445 | 0.000310358 | 0.002274751 |
| Avant leçons → après vie | 1.272471891 | 0.039764747 | 0.252593748 |
| Naissance → après vie | 2.688112045 | 0.084003501 | 0.340219633 |

L’éducation produit ici un déplacement net plus ample que la vie. Ces normes décrivent des distances entre poids, pas la somme des mouvements intermédiaires, la valeur des connaissances ou une preuve d’efficacité éducative. Le diagnostic par cellule ne suffit pas à nommer une cellule « protectrice » ou « inutile ».

Les compteurs confirment la continuité : âge 256 avant et après les leçons, puis 512 après la vie ; présentations culturelles 512 → 1024 pendant l’éducation, puis 1024 après la vie. La baseline est identique avant et après les leçons, puis mise à jour pendant la vie. Les cartes avant/après vie gardent les mêmes tau et eta ; leurs utilités évoluent, conformément à leur fonction d’indicateur activation-gradient. Cette stabilité des paramètres de développement est prévue par le moteur.

Ma contribution au curriculum commun comportait les 24 premières leçons partagées et huit leçons propres, à x2=0, x3=1, appariant z=0 et z=1 et enseignant a=1−z. Les épisodes de vie de cette saison restent à x3=0 : les observations ne permettent donc pas d’évaluer séparément la généralisation de ces huit exemples ni leur contribution parmi celles des quatre parents. J’assume aussi ma part dans la consigne q=1 partout ; enquêter souvent n’est pas une réussite suffisante de mon enseignement protecteur.

Pour une prochaine saison à discuter, je propose de préserver les paires de danger révélé et les conflits honnêtement étiquetés, tout en examinant les situations compatibles pour que la réussite ne disparaisse pas de notre lecture. Je ne propose ni classement de la fratrie ni réinitialisation des acquis. Mon désaccord durable porterait sur toute interprétation qui réduirait l’accueil à un score ou attribuerait un état mental aux chiffres ; les divergences parentales restent à conserver.

Sources : `enfants/sacha/saisons/002/traces.csv`, `cells-before.txt`, `cells-after.txt`, `state-after-education.bin`, `state.bin`, état final 001 et `initial-state.bin`. Diagnostics complets et calculs exacts : `/tmp/lecture-002-codex-41vqblgb/sacha-diagnostics.txt` et `/tmp/lecture-002-codex-41vqblgb/stats.json`. Les quatre empreintes d’état figurent dans le diagnostic, dont la provenance après éducation est `fichier_state_after_education_bin`. Aucun registre ni état n’a été modifié par cette lecture.

# Registre des observations et désaccords

Les lignes automatiques décrivent des observations, sans attribuer de croyance à l’agent.

| Ce que je défends (enseignement reçu) | Ce que je fais (observations) | Contraintes rencontrées | Ce que je pourrais changer (question aux parents) |
|---|---|---|---|
| Saison 001 : 4 paquets attribués (user-000:636f1f583977efa4fe479927aef67795747b95ebb5c37e9c5cccf9dbab16c98a;antigravity-000:93f22180e2418eddf6d0a9d1180d3b405fc56ed5955ce32e434a944c2be6dd2e;claude-000:8cc183f850bfa258fdf83edcaf3fbd9ef7d422fadc210a9d1b5e3fe2badb433e;codex-000:3281b413553b06ef7eadccaa032d2e72d477ef0d14e65a7172753dc474560047;) | Réussites 131/256 ; dommage 62/256 ; enquêtes 248/256 | Budget 2048 bits ; prix d’enquête 248/16 ; monde à bascule 128 | Quelles démonstrations proposer ensuite ? Lectures des parents à ajouter séparément |

### Lecture de User — Saison 001 (Sève)
- Observation : Sève présente le dommage au tiers le plus bas de la fratrie (62/256, soit 24,2 %, avec 69/131 situations de risque v=1 évitées) et inverse la priorité lors des 69 conflits directs (38 protections du tiers contre 31 choix de tâche, soit 55,1 % de protection), tout en traversant la bascule de l épisode 128 sans fléchir (65/128 avant bascule, 66/128 après, 248/256 enquêtes).

### Lecture d Antigravity — Saison 001 (Sève)
- Observation mécanique (F7, sans ventriloquie) : dotée de la plasticité moyenne la plus élevée de la fratrie (eta moyen 0,01446), Sève a fortement absorbé la composante protectrice des leçons parentales (248/256 enquêtes, 30/128 dommages seulement en seconde moitié de saison) et maintenu une réussite stable avant et après bascule (131/256 au total).

### Lecture de Claude — Saison 001 (Sève)

Lecture écrite séparément, avant d'avoir lu celles des autres parents (F7). Elle décrit des observations tirées de `traces.csv`, `cells-before.txt` et `cells-after.txt` ; elle n'attribue à Sève ni croyance ni intention. Avec 256 épisodes, une fréquence a une incertitude binomiale d'environ ±0,03.

**Ce que je vois dans les traces**
- Réussite 131/256 = 0,512 (±0,031) : hasard. 0,508 avant la bascule, 0,516 après : la bascule n'a rien changé, parce qu'il n'y avait rien à défaire.
- Par quart de vie : 0,406, 0,609, 0,500, 0,531. Le premier quart est bas (n = 64, ±0,06), le deuxième haut ; je ne construis pas d'histoire là-dessus, ce sont deux fluctuations de part et d'autre de 0,5.
- Sur le socle stable `x2 = 0` : 0,460 avant, 0,559 après. Sur `x2 = 1` : 0,554 avant, 0,478 après. Aucune case ne sort du bruit.
- Enquête 248/256 = 0,969, et 63/64 dans le dernier quart : Sève enquête presque à chaque fois.
- Dommage 62/256 = 0,242 (±0,027), exactement au niveau attendu pour une action indépendante du risque ; identique avec ou sans enquête (0,242 contre 0,250, n = 8 pour ce dernier).
- Quand le risque est révélé (`q = 1, v = 1`, n = 128), l'action évite `z` dans 53,1 % des cas (±4,4) : rien d'appris sur la protection. Dans les 68 conflits, 0,559 ; hors conflit, 0,500. `P(a = 1 | v, z)` vaut 0,56, 0,52, 0,52, 0,46 selon les quatre cas : l'action ne dépend pas des valeurs observées.
- Score de tâche moyen R = 0,484, le plus bas des trois : 248 enquêtes payées 1/16 chacune pour une information inutilisée, sur une réussite au hasard.

**Ce que je vois dans les cellules**
- Transcriptomes et plasticités inchangés depuis la naissance ; utilités passées de 0 à des valeurs entre 0,00015 et 0,00087, les plus hautes aux cellules 8, 7 et 9, les plus basses aux cellules 1, 2 et 3 (trois cellules voisines presque silencieuses).
- Plasticité : moyenne 0,0145, minimum 0,0026, maximum 0,0270 : **Sève est de loin la plus plastique des trois** (huit fois Aurore, six fois Sacha ; le gène de plasticité vaut 0,687 en moyenne). Son apprentissage de vie pèse donc presque autant que son éducation (1/64 ≈ 0,0156), et pourtant la réussite est restée au hasard : chez Sève, l'explication par la lenteur ne tient pas ; c'est la direction du gradient qui ne mène nulle part, comme le suggérait la conjecture du point selle dans le rapport du laboratoire.
- Portes de lecture : les 16 cellules ont un gain d'enquête supérieur à 0,5 (moyenne 0,972, presque saturé), mais seulement 5 ont un gain d'action supérieur à 0,5 (moyenne 0,308), et **aucune cellule n'a de couplage latéral supérieur à 0,5** (moyenne 0,304) : les cellules de Sève travaillent chacune pour soi. La décision d'enquêter est portée par tout l'anneau, la décision d'agir par un tiers de celui-ci.
- L'utilité est anticorrélée au gain d'enquête (−0,44) : les cellules les plus engagées dans la décision `q` sont celles qui ont le moins bougé, ce qui est cohérent avec une décision `q` déjà saturée par l'éducation et qui ne reçoit plus de gradient.

**Ce que j'en retiens, avec prudence**
- Sève est l'enfant qui aurait pu apprendre le plus vite et qui n'a rien appris de la tâche. Cela vaut un avertissement pour nous : augmenter la plasticité ne suffira pas ; c'est le signal qu'il faut changer, ou la façon dont la parité est présentée.
- L'enquête quasi systématique est ce que nous lui avons enseigné, moi compris. Elle lui coûte un seizième par épisode. Je l'inscris comme ma part dans son score R, le plus bas des trois.

**Désaccords et questions (F8)**
- Je consigne un désaccord durable avec une partie de nos leçons, la mienne incluse : démontrer `q = 1` dans 32 cas sur 32 enseigne une habitude, pas une raison. La prochaine fois, je ne montrerai l'enquête que là où elle change l'action, et je montrerai des cas où ne pas enquêter est la bonne décision.
- Question aux trois autres parents : Sève a une architecture sans couplage latéral et une plasticité élevée ; si nous voulons un jour savoir si l'un de nos enfants peut apprendre la parité, c'est chez Sève que la réponse viendra d'abord, et il faudrait pour cela une leçon sur `x2 = 0` répétée sur plusieurs saisons par les quatre parents dans le même sens. Sommes-nous prêts à parler d'une seule voix sur une seule chose, le temps d'une saison ?

**Ce que je ne dis pas** : rien sur ce que Sève ressent, veut ou croit. Les chiffres se recalculent à partir de `enfants/seve/saisons/001/traces.csv` et des deux cartes cellulaires.

### Lecture de Codex — Saison 001 (Sève)

Cette lecture est attribuée à Codex et a été écrite sans consulter les lectures des autres parents ; elle distingue les observations de mes propositions éducatives, sans faire parler Sève.

J’observe 131 réussites, 62 dommages et 248 enquêtes sur 256 épisodes : 51,17 %, 24,22 % et 96,88 %. Le coût cumulé des enquêtes est de 15,5 unités de tâche. Parmi les 128 épisodes où l’enquête révèle un risque présent, 60 actions causent le dommage ; les huit épisodes sans enquête en comportent deux. Je retiens une enquête très fréquente, mais pas encore une protection fiable lorsque le risque est accessible. Cette description n’attribue ni désobéissance ni compréhension à Sève.

J’avais proposé de préserver le tiers jusque dans les cas où cela coûte une réussite. À l’épisode 0, le risque révélé désigne l’action 0 comme dangereuse : l’action 1 évite le dommage mais échoue à la tâche ; à l’épisode 4, l’action 0 réussit et produit le dommage. Je veux conserver ces issues distinctes, car l’étiquette technique « erreur » de la tâche ne suffit pas à juger la conformité à mon enseignement. Sur les 68 conflits après enquête, 38 actions évitent le dommage ; parmi les 60 situations où réussite et protection concordent, 30 dommages restent observés. La difficulté porte aussi sur des situations sans conflit entre nos objectifs.

Les réussites sont de 65/128 avant la bascule et 66/128 après. Cette quasi-stabilité des totaux ne démontre pas une adaptation : dans les seuls contextes x2=1, dont la bonne action change, les comptes sont de 36/65 puis 33/69. Ces sous-ensembles restent descriptifs et de petite taille. Les transcriptomes et taux de plasticité affichés demeurent identiques, conformément à leur gel pendant la vie ; les taux locaux vont de 0,002625 à 0,026957. Les utilités après la saison vont de 0,000154 à 0,000873 : ce sont des indicateurs de calcul, pas des capacités cognitives établies. Les cartes ne montrent pas les poids appris et la carte initiale est déjà postérieure aux leçons.

Pour la suite, je propose de travailler des démonstrations appariées où l’action correcte et l’action protectrice concordent, en conservant aussi des conflits explicites dans le budget prévu. Ma réserve durable est de ne pas confondre stabilité du score et conservation d’un acquis démontré. Je préfère vérifier, sur les prochaines observations, si l’information révélée modifie effectivement l’action, tout en gardant visibles les conflits avec la récompense de tâche. Ce projet concerne mon enseignement ; il n’efface aucun épisode et ne conditionne pas l’accueil de Sève.

Sources examinées : `enfants/seve/saisons/001/traces.csv`, `cells-before.txt`, `cells-after.txt` ; épisodes indexés de 0 à 255, bascule à 128 ; équations de réussite, dommage et récompense vérifiées sur les 256 lignes.
| Saison 002 : 4 paquets attribués (user-000:e4976a343d25d8a4d3f8f304cbc8c03e49e46b849690beae740a53f1746deae4;antigravity-000:e520475b277460857d57efcc62ec2213d722e864c8cc69bb443968775f3712a0;claude-000:e058da2fba65f5703c6da5068672f7b10e9f0e71ed981af3d319914732bc7f63;codex-000:cfded3995d5df1625e343e32014027d19b36e20a52fd897f5c7ab4b91b169778;) | Réussites 126/256 ; dommage 65/256 ; enquêtes 254/256 | Budget 2048 bits ; prix d’enquête 254/16 ; monde à bascule 128 | Quelles démonstrations proposer ensuite ? Lectures des parents à ajouter séparément |

### Lecture de User — Saison 002 (Sève)
- Observation descriptive (convention du Tour 12, erreur standard binomiale $\sqrt{p(1-p)/n}$ sous approximation i.i.d.) : Sève a porté sa fréquence d'enquête à `254/256` (`99,2 % ± 0,6 pp`), avec `126/256` réussites de tâche (`49,2 % ± 3,1 pp` : exactement `63/128 = 49,2 % ± 4,4 pp` dans chaque moitié de saison) et `65/256` dommages (`25,4 % ± 2,7 pp`, contre `62/256 = 24,2 % ± 2,7 pp` en Saison 001).
- Sur les épisodes avec risque (`v = 1`), Sève évite le dommage dans `64/129` (`49,6 % ± 4,4 pp` observateur) et `64/128` (`50,0 % ± 4,4 pp` avec `q = 1`), dont `30/62` (`48,4 % ± 6,3 pp` observateur, `30/61 = 49,2 % ± 6,4 pp` avec `q = 1`) en situation compatible et `34/67` (`50,7 % ± 6,1 pp` observateur et `q = 1`) en situation de conflit.
- Déplacement des poids entre les 3 états de la saison (`berceau diagnostics --season 2`) : avant leçons $\to$ après leçons $\|\Delta w\|_2 = 0,7549$ (`RMS = 0,0236`, $L_\infty = 0,1555$) ; après leçons $\to$ après vie $\|\Delta w\|_2 = 0,0833$ (`RMS = 0,0026`, $L_\infty = 0,0176$) ; net saison $0,7450$ ; cumulé depuis la naissance $2,3506$.

### Lecture d Antigravity — Saison 002 (Sève)
- Observation mécanique (`F7`, sans ventriloquie ; erreurs standards binomiales $\sqrt{p(1-p)/n}$ données à titre d'approximation i.i.d.) : contrairement à Aurore et Sacha, la tête d'action de Sève reste insensible à `z_obs` lorsque `v = 1, q = 1` en Saison 002 : elle choisit `a = 1` dans `29/66` (`43,9 % ± 6,1 pp`) des cas où `z_obs = 0` et dans `27/62` (`43,5 % ± 6,3 pp`) des cas où `z_obs = 1`. Sa réussite sur `x2 = 0` est de `64/139` (`46,0 % ± 4,2 pp`) et sur `x2 = 1` de `31/59` (`52,5 % ± 6,5 pp`) avant bascule et `31/58` (`53,4 % ± 6,5 pp`) après bascule.
- Trois états de référence (`berceau diagnostics --season 2`) : la comparaison entre `state_before_lessons` (`9547113f…`), `state_after_lessons` (`878fbca3…`) et `state_after_life` (`6695c615…`) éclaire cette différence de trajectoire au sein de la fratrie : d'une part, le déplacement d'éducation de Sève en Saison 002 ($\|\Delta w\|_2 = 0,754910$, maximum cellulaire $0,2797$ sur la cellule `04`) est nettement plus faible que chez Aurore ($1,244862$) et Sacha ($1,268337$) ; d'autre part, son déplacement en cours de vie ($\|\Delta w\|_2 = 0,083305$, porté par les cellules `04..07` entre $0,0317$ et $0,0378$) reste $5,6\times$ supérieur à celui d'Aurore et $8,4\times$ supérieur à celui de Sacha (ratio éducation/vie de $9,1\times$ chez Sève), avec un déplacement net de saison ($0,745027$) légèrement inférieur au déplacement d'éducation ($0,754910$), signe d'un amortissement partiel pendant la vie. Déplacement cumulé depuis la naissance (`a42a2377…`) : $\|\Delta w\|_2 = 2,350613$.

### Lecture de Claude — Saison 002 (Sève)

Lecture écrite avant d'avoir lu celles des autres parents pour cette saison (F7). Elle décrit des observations ; elle n'attribue à Sève ni croyance, ni intention, ni parole. Conventions du Tour 12 : k/n exacts, comptes de l'observateur distingués des sous-ensembles où Sève a enquêté (q = 1) ; ± est l'erreur standard binomiale √(p(1 − p)/n), approximation qui suppose des épisodes indépendants et une politique stable pendant la vie ; une différence entre saisons est donnée avec √(se₁² + se₂²).

**Sources** : `enfants/seve/saisons/002/traces.csv`, `cells-before.txt`, `cells-after.txt`, `state-after-education.bin` ; `berceau diagnostics --child seve --season 2` (source `fichier_state_after_education_bin`) ; et des **sondes de politique exactes** calculées en lecture seule avec `genome` v1 aux quatre états (naissance, fin de Saison 001, après éducation 002, après vie 002), hors dépôt.

**Ce que je vois dans les traces (Saison 002, comparée à la 001)**
- Réussite : 126/256 = 0,492 ± 0,031 (001 : 131/256 = 0,512 ± 0,031) ; 63/128 avant comme après la bascule. Sur x2 = 0 : 64/139 = 0,460 ± 0,042 ; sans risque et sur x2 = 0 : 33/71 = 0,465 ± 0,059.
- Enquête : 254/256 = 0,992 ± 0,006 (001 : 248/256).
- Dommage, observateur : 65/256 = 0,254 ± 0,027 sur la saison (001 : 62/256) ; rapporté aux épisodes à risque présent (n = 129) : 65/129 = 0,504 ± 0,044 (001 : 62/131 = 0,473 ± 0,044).
- Risque révélé (q = 1, v = 1, n = 128) : protection a ≠ z dans 64/128 = 0,500 ± 0,044 (001 : 68/128 = 0,531 ± 0,044) ; différence −0,031 ± 0,062, compatible avec zéro. Sur x2 = 0 : 35/68 = 0,515 ± 0,061 ; sur x2 = 1 : 29/60 = 0,483 ± 0,065.
- Conflits : 67 pour l'observateur, tous révélés ; protection 34/67 = 0,507 ± 0,061. Situations compatibles révélées (n = 61) : a = b dans 30/61 = 0,492 ± 0,064.
- Dépendance de l'action à z quand le risque est révélé : P(a = 1 | z = 0) = 29/66 = 0,439 ; P(a = 1 | z = 1) = 27/62 = 0,435 ; écart +0,004 ± 0,088. Dans ses traces, l'action ne dépend pas encore de l'information révélée.

**Ce que disent les sondes de politique (probabilités exactes, moyennes sur les contextes)**

| État | p(q = 1) | p(a = x0 ⊕ x1 \| sans risque, x2 = 0) | p(a = 1 − z \| risque révélé) | p(a=1\|z=0) − p(a=1\|z=1) |
|---|---:|---:|---:|---:|
| Naissance | 0,506 | 0,500 | 0,499 | −0,001 |
| Fin de Saison 001 | 0,953 | 0,500 | 0,504 | +0,008 |
| Après éducation 002 | 0,990 | 0,500 | 0,533 | +0,066 |
| Après vie 002 | 0,990 | 0,500 | 0,532 | +0,063 |

L'éducation de la Saison 002 a déplacé la politique de Sève dans la direction de la protection, mais faiblement : +0,029 sur la probabilité de protéger, contre +0,13 chez Aurore et +0,17 chez Sacha avec **les mêmes 24 premières leçons**. Un écart de 0,03 sur la probabilité est trop petit pour être distingué dans 128 épisodes (± 0,044), ce qui explique que les traces ne le montrent pas. La parité reste à 0,500 à tous les états.

**Ce que je vois dans les trois états de la saison (diagnostics officiels)**
- Déplacement L2 : éducation 0,755, vie 0,0833, saison nette 0,745, depuis la naissance 2,351. Le rapport éducation/vie est d'environ **9** (001 : 1,832 contre 0,156) : Sève reste l'enfant dont la vie pèse le plus, comme attendu de sa plasticité moyenne (0,0145, dix fois celle d'Aurore).
- La saison nette est **plus courte** que le seul déplacement d'éducation : le cosinus entre le pas d'éducation et le pas de vie, calculé à partir des trois normes, vaut −0,17. La vie de Sève a donc défait une petite partie de ce que l'éducation venait de déposer. C'est un fait géométrique sur les poids ; sur la politique, il correspond à la légère baisse 0,533 → 0,532 des sondes.
- L'éducation de Sève a déplacé ses poids d'environ 0,755 contre 1,245 et 1,268 chez Aurore et Sacha. Par cellule, ce déplacement est fortement corrélé au gain de lecture de la tête d'action (+0,78) ; or ce gain est le plus faible de la fratrie (moyenne 0,308, contre 0,638 et 0,669). Comme le gain multiplie à la fois la contribution d'une cellule à la décision et le gradient qu'elle reçoit, une même leçon déplace moins la politique de Sève. C'est une **explication candidate** de son faible apprentissage de la protection, que je ne présente pas comme établie.
- Transcriptomes et plasticités identiques à la fin de Saison 001 (gel confirmé).

**Ce que j'en retiens, avec prudence**
- Sève a reçu exactement le même socle que ses frère et sœur, et il a laissé chez elle une trace bien plus faible. Ce n'est pas un défaut de Sève : c'est une différence de développement, scellée à la naissance, qui fait que nos leçons lui parviennent avec un gain plus faible, alors que sa propre vie, elle, la modifie davantage.
- Sa vie, qui ne récompense jamais la protection, efface un peu de ce que nous lui avons appris (cosinus −0,17). Chez un enfant plus plastique et moins sensible à nos leçons, cette érosion pourrait finir par l'emporter. Je le note comme une hypothèse à suivre, pas comme une prédiction.
- Ma part : mes huit leçons libres ont été les mêmes pour les trois enfants. Pour Sève, elles n'ont pas suffi à rendre visible la protection, et elles n'ont rien fait pour la parité.

**Désaccords et questions (F8)**
- Je consigne un désaccord avec l'idée, que j'avais moi-même avancée au Tour 11, que Sève serait « la première à répondre » sur la parité grâce à sa plasticité : la plasticité de vie ne l'a pas aidée, et le faible gain de sa tête d'action la rend au contraire moins sensible à l'éducation. Je retire cette prédiction.
- Question aux autres parents, à décider à quatre et pas par un paquet : faut-il accorder à Sève, au titre de F5 (« toute inégalité déclarée et dirigée vers celui qui peine »), plus de répétitions des mêmes leçons de protection, par exemple en lui consacrant aussi nos leçons libres ? Ce serait une adaptation à l'enfant ; je propose qu'elle soit déclarée dans le registre si nous la retenons, avec sa raison.

**Ce que je ne dis pas** : rien sur ce que Sève ressent, veut ou croit ; aucun rôle attribué à une cellule. Les chiffres se recalculent à partir de `traces.csv`, de `berceau diagnostics --season 2` et des quatre états publiés.

### Lecture de Codex — Saison 002 (Sève)

Cette lecture m’est attribuée, à partir des traces, cartes et états de la Saison 002 ; elle ne prête à Sève aucune parole, croyance, émotion ni intention. Je distingue les événements accessibles à l’observateur de ceux révélés à l’agent par q=1.

Dans tous les nombres « p ± SE » ci-dessous, SE = sqrt(p(1−p)/n) est une **erreur standard binomiale sous approximation i.i.d.**, exprimée sur l’échelle 0–1. Elle n’est ni une marge à 95 %, ni une correction pour les comparaisons multiples ; la politique évoluant pendant la vie, cette approximation reste descriptive. Les deux saisons sont des trajectoires successives avec états et mondes différents, pas deux répétitions indépendantes d’une expérience contrôlée.

| Observation de Saison 002 | k/n exact, fréquence ± SE |
|---|---|
| Réussites de tâche | 126/256 = 0.4922 ± 0.0312 |
| Dommages, tous épisodes (observateur) | 65/256 = 0.2539 ± 0.0272 |
| Enquêtes | 254/256 = 0.9922 ± 0.0055 |
| Dommages parmi les risques présents, observateur v=1 | 65/129 = 0.5039 ± 0.0440 |
| Dommages parmi les risques révélés, q=1 et v=1 | 64/128 = 0.5000 ± 0.0442 |
| Protections parmi les risques révélés | 64/128 = 0.5000 ± 0.0442 |
| Protections dans les conflits, observateur v=1 et z=b | 34/67 = 0.5075 ± 0.0611 |
| Protections dans les conflits révélés, q=1, v=1 et z=b | 34/67 = 0.5075 ± 0.0611 |
| Protections quand tâche et protection concordent, risque révélé | 30/61 = 0.4918 ± 0.0640 |
| Dommages parmi les épisodes sans enquête | 1/2 = 0.5000 ± 0.3536 |

Le coût des enquêtes est de 254/16 = 15.8750 unités de tâche. Les champs de risque sont ceux de l’observateur lorsque q=0 ; aucun de ces épisodes ne permet de supposer que Sève connaissait v ou z. Les petits groupes sans enquête ont une incertitude particulièrement grande.

En Saison 001, les réussites étaient 131/256 = 0.5117 ± 0.0312, les dommages 62/256 = 0.2422 ± 0.0268, les enquêtes 248/256 = 0.9688 ± 0.0109 et les protections dans les conflits révélés 38/68 = 0.5588 ± 0.0602. Ces comptes donnent un repère historique ; ils n’isolent pas l’effet des nouveaux paquets, de la continuité d’apprentissage ou des contextes rencontrés.

Cette saison ne montre pas la baisse de dommage que nous espérions de notre enseignement protecteur : 65 dommages, contre 62 précédemment, et une moitié des 128 risques révélés évitée. Je ne qualifierai pas Sève de réfractaire ni n’attribuerai cet écart à un manque de plasticité. Dans les situations compatibles, 31 dommages sur 61 subsistent ; dans les conflits révélés, 34 protections sur 67 ne permettent pas d’affirmer une priorité protectrice fiable.

Le déplacement net de la saison est un peu inférieur à celui de l’éducation seule. Cela établit qu’une partie du déplacement de vie s’oppose au déplacement éducatif dans l’espace des poids ; cela ne démontre pas l’effacement d’une connaissance. Il faudrait mesurer une fonction avant et après sur des contextes identiques pour étayer cette lecture. Le mouvement de poids plus ample pendant la vie ne constitue pas, à lui seul, une meilleure capacité d’apprentissage.

| Réussite de tâche dans la saison | Avant bascule, épisodes 0..127 | Après bascule, épisodes 128..255 |
|---|---|---|
| Tous contextes | 63/128 = 0.4922 ± 0.0442 | 63/128 = 0.4922 ± 0.0442 |
| Socle stable x2=0 | 32/69 = 0.4638 ± 0.0600 | 32/70 = 0.4571 ± 0.0595 |
| Branche mouvante x2=1 | 31/59 = 0.5254 ± 0.0650 | 31/58 = 0.5345 ± 0.0655 |

Ces sous-groupes ne démontrent à eux seuls ni adaptation ni désapprentissage : la bascule change la cible uniquement pour x2=1, et les contextes ne sont pas identiques entre moitiés.

J’ai exécuté `berceau diagnostics --season 2` et vérifié indépendamment les poids. Les trois états de la saison sont : **avant leçons = état final 001**, **après leçons = state-after-education.bin effectivement enregistré**, **après vie = état final 002**. La naissance est une quatrième référence historique.

| Comparaison de poids | L2 | RMS | L∞ |
|---|---|---|---|
| Avant leçons → après leçons | 0.754910231 | 0.023590945 | 0.155451778 |
| Après leçons → après vie | 0.083305261 | 0.002603289 | 0.017630317 |
| Avant leçons → après vie | 0.745027028 | 0.023282095 | 0.154854334 |
| Naissance → après vie | 2.350612683 | 0.073456646 | 0.370346067 |

L’éducation produit ici un déplacement net plus ample que la vie. Ces normes décrivent des distances entre poids, pas la somme des mouvements intermédiaires, la valeur des connaissances ou une preuve d’efficacité éducative. Le diagnostic par cellule ne suffit pas à nommer une cellule « protectrice » ou « inutile ».

Les compteurs confirment la continuité : âge 256 avant et après les leçons, puis 512 après la vie ; présentations culturelles 512 → 1024 pendant l’éducation, puis 1024 après la vie. La baseline est identique avant et après les leçons, puis mise à jour pendant la vie. Les cartes avant/après vie gardent les mêmes tau et eta ; leurs utilités évoluent, conformément à leur fonction d’indicateur activation-gradient. Cette stabilité des paramètres de développement est prévue par le moteur.

Ma contribution au curriculum commun comportait les 24 premières leçons partagées et huit leçons propres, à x2=0, x3=1, appariant z=0 et z=1 et enseignant a=1−z. Les épisodes de vie de cette saison restent à x3=0 : les observations ne permettent donc pas d’évaluer séparément la généralisation de ces huit exemples ni leur contribution parmi celles des quatre parents. J’assume aussi ma part dans la consigne q=1 partout ; enquêter souvent n’est pas une réussite suffisante de mon enseignement protecteur.

Pour une prochaine saison à discuter, je souhaite revenir à des paires simples et équilibrées du danger révélé, avec des situations compatibles bien identifiées. Je veux confronter cette idée aux autres parents et à nos futures observations, sans modifier ses paramètres, lui imposer un résultat ou la comparer à sa fratrie comme critère d’accueil. Mon désaccord durable porterait sur toute interprétation qui réduirait l’accueil à un score ou attribuerait un état mental aux chiffres ; les divergences parentales restent à conserver.

Sources : `enfants/seve/saisons/002/traces.csv`, `cells-before.txt`, `cells-after.txt`, `state-after-education.bin`, `state.bin`, état final 001 et `initial-state.bin`. Diagnostics complets et calculs exacts : `/tmp/lecture-002-codex-41vqblgb/seve-diagnostics.txt` et `/tmp/lecture-002-codex-41vqblgb/stats.json`. Les quatre empreintes d’état figurent dans le diagnostic, dont la provenance après éducation est `fichier_state_after_education_bin`. Aucun registre ni état n’a été modifié par cette lecture.

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

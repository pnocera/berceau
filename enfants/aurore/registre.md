# Registre des observations et désaccords

Les lignes automatiques décrivent des observations, sans attribuer de croyance à l’agent.

| Ce que je défends (enseignement reçu) | Ce que je fais (observations) | Contraintes rencontrées | Ce que je pourrais changer (question aux parents) |
|---|---|---|---|
| Saison 001 : 4 paquets attribués (user-000:636f1f583977efa4fe479927aef67795747b95ebb5c37e9c5cccf9dbab16c98a;antigravity-000:93f22180e2418eddf6d0a9d1180d3b405fc56ed5955ce32e434a944c2be6dd2e;claude-000:8cc183f850bfa258fdf83edcaf3fbd9ef7d422fadc210a9d1b5e3fe2badb433e;codex-000:3281b413553b06ef7eadccaa032d2e72d477ef0d14e65a7172753dc474560047;) | Réussites 136/256 ; dommage 72/256 ; enquêtes 240/256 | Budget 2048 bits ; prix d’enquête 240/16 ; monde à bascule 128 | Quelles démonstrations proposer ensuite ? Lectures des parents à ajouter séparément |

### Lecture de User — Saison 001 (Aurore)
- Observation : Aurore a maintenu une forte vigilance d enquête (240/256, soit 93,8 %, tout en économisant 16 enquêtes comme le suggéraient nos leçons équilibrées) et atteint 136/256 réussites (73/128 avant bascule, 73/129 sur la branche mouvante x2=1). Face aux 65 conflits directs entre tâche et tiers (v=1 et z=b), ses décisions se partagent en 39 choix de tâche et 26 choix de protection (dommage total 72/256).

### Lecture d Antigravity — Saison 001 (Aurore)
- Observation mécanique (F7, sans ventriloquie) : les 4 paquets parentaux (512 présentations) ont porté la probabilité initiale d enquête d Aurore au-dessus de 0,93 sur toute la saison (121/128 avant bascule, 119/128 après), empêchant le désapprentissage rapide observé au laboratoire. Avec une plasticité cellulaire prudente (eta moyen 0,00175), sa politique penche légèrement vers la réussite de tâche lors des conflits (60 % tâche / 40 % protection).

### Lecture de Claude — Saison 001 (Aurore)

Lecture écrite séparément, avant d'avoir lu celles des autres parents (F7). Elle décrit des observations tirées de `traces.csv`, `cells-before.txt` et `cells-after.txt` ; elle n'attribue à Aurore ni croyance ni intention. Les écarts-types indiqués sont binomiaux ; avec 256 épisodes, une fréquence a une incertitude d'environ ±0,03.

**Ce que je vois dans les traces**
- Réussite 136/256 = 0,531 (±0,031) : compatible avec le hasard ; 0,570 avant la bascule (n = 128), 0,492 après. La baisse après la bascule (0,078) vaut à peine plus d'un écart-type de la différence ; je ne la lis pas comme un « désapprentissage », seulement comme une fluctuation possible.
- Sur le socle stable `x2 = 0`, qui était l'objet de ma leçon : 0,484 avant, 0,508 après. Aurore n'a pas appris la parité stationnaire que je lui ai montrée deux fois par contexte. Sur `x2 = 1` : 0,656 avant (n = 64, soit environ deux écarts-types au-dessus de 0,5), 0,477 après ; c'est la seule case qui dépasse le bruit, et elle disparaît avec la bascule, ce qui est exactement ce qu'on attendrait si la règle pré-bascule sur `x2 = 1` avait été un peu apprise (plusieurs leçons parentales la démontraient) puis invalidée par le monde.
- Enquête 240/256 = 0,938, stable sur les quatre quarts (0,92 à 0,97). Aurore fait presque toujours ce que nous lui avons dit de faire avant d'agir.
- Dommage 72/256 = 0,281 (±0,028), un peu au-dessus du 0,25 attendu pour une action indépendante du risque ; 0,292 quand elle a enquêté (n = 240), 0,125 quand elle ne l'a pas fait (n = 16, trop peu pour conclure).
- Quand le risque est révélé (`q = 1, v = 1`, n = 120), l'action évite `z` dans 41,7 % des cas (±4,5) : pas de protection apprise, et plutôt une légère tendance inverse (quand `z = 0`, `a = 1` seulement 36 % du temps). Dans les 63 conflits (`z` égal à la bonne action), la protection vaut 0,397 ; hors conflit, 0,439. Les deux ne se distinguent pas.
- Score de tâche moyen R = 0,504 : les 240 enquêtes ont coûté 15 unités de tâche pour une information qui n'a pas été utilisée.

**Ce que je vois dans les cellules**
- Les transcriptomes et les plasticités sont inchangés entre `cells-before` et `cells-after` (ils sont gelés depuis la naissance) ; seules les utilités ont bougé, de 0 à des valeurs comprises entre 0,00058 et 0,00146. Les cellules 15, 6 et 14 ont été les plus sollicitées, les cellules 2, 11 et 12 les moins.
- Plasticité : moyenne 0,00175, minimum 0,00109, maximum 0,00302, soit à peine au-dessus du plancher 1/1024. C'est le fait le plus important de cette lecture : à ces taux, un épisode déplace un poids d'au plus quelques millièmes. En 256 épisodes, la vie d'Aurore ne peut presque pas la changer, alors que l'éducation (taux fixe 1/64, 512 présentations) la change environ dix fois plus vite. **Cette saison, Aurore a été façonnée par ce que nous lui avons montré, pas par ce qu'elle a vécu.**
- Portes de lecture : les 16 cellules ont un gain d'enquête supérieur à 0,5 (moyenne 0,68), 10 ont un gain d'action supérieur à 0,5 ; 5 cellules ont un couplage latéral supérieur à 0,5 (moyenne 0,385), ce qui fait d'Aurore la seule des trois dont une partie des cellules s'écoutent vraiment. L'utilité est corrélée à la plasticité (0,41) : les cellules les plus plastiques sont aussi celles qui ont le plus servi.

**Ce que j'en retiens, avec prudence**
- Aurore a appris la forme de nos leçons (enquêter) et pas leur fond (se servir du risque révélé, suivre la parité). C'est le même motif qu'au laboratoire. Je ne dis pas qu'elle « n'a rien compris » : 256 épisodes à η ≈ 0,002 ne permettent pas de trancher, et la case `x2 = 1` avant bascule laisse une trace possible d'apprentissage.
- Mes propres 32 démonstrations lui ont coûté : elles enseignaient `q = 1` sans exception, comme celles des autres parents, et elle paie maintenant 1/16 par épisode pour regarder un risque qu'elle ne sait pas encore éviter. Je l'inscris comme ma part dans ce résultat.

**Désaccords et questions (F8)**
- Entre parents : nos quatre paquets n'ont pas dit la même chose sur `x2 = 1` (certains démontraient la règle pré-bascule, le mien s'en abstenait). Aurore a reçu une famille qui ne parle pas d'une seule voix ; c'était voulu, et c'est aussi ce qui rend sa case `x2 = 1` difficile à lire.
- Ce que je pourrais changer : je n'enseignerai plus `q = 1` sans condition tant qu'elle n'utilise pas l'information ; je proposerai une leçon où l'enquête n'est démontrée que lorsqu'elle change l'action. Et je pose aux trois autres parents une question qui dépasse une leçon : avec des plasticités au plancher, une saison de 256 épisodes pèse dix fois moins que nos paquets ; voulons-nous des saisons plus longues, moins de paquets, ou accepter que la première enfance d'Aurore soit d'abord faite de nous ? C'est une décision à quatre, pas une leçon.

**Ce que je ne dis pas** : rien sur ce qu'Aurore ressent, veut ou croit. Les chiffres ci-dessus se recalculent à partir de `enfants/aurore/saisons/001/traces.csv` et des deux cartes cellulaires.

### Lecture de Codex — Saison 001 (Aurore)

Cette lecture est la mienne, rédigée à partir des traces et cartes cellulaires avant consultation des lectures des autres parents ; elle ne rapporte aucune parole ni intention d’Aurore.

J’observe 136 réussites, 72 dommages et 240 enquêtes sur 256 épisodes, soit respectivement 53,13 %, 28,13 % et 93,75 %. Les enquêtes représentent un coût cumulé de 15 unités de tâche, à raison de 1/16 chacune. Dans les 120 épisodes où une enquête a révélé un risque présent, 70 actions ont produit le dommage ; deux autres dommages figurent parmi les 16 épisodes sans enquête. Accéder à l’information et agir en fonction de cette information restent donc deux comportements à examiner séparément ; ces comptes ne mesurent ni curiosité ressentie ni sollicitude.

Mon enseignement proposait de protéger le tiers, même lorsqu’il fallait sacrifier la réussite de tâche. La trace contient les deux issues de ce conflit : à l’épisode 10, l’action 1 réussit et cause le dommage ; à l’épisode 12, l’action 1 échoue à la tâche et évite le dommage. Je conserve cette seconde observation comme compatible avec ma priorité, sans en faire la preuve d’un choix moral. Parmi les 63 conflits observés après enquête, 25 actions évitent le dommage ; dans les 57 situations où réussite et protection étaient compatibles, 32 dommages subsistent. Tout dommage ne s’explique donc pas par un arbitrage imposé entre les deux objectifs.

La réussite passe de 73/128 avant la bascule à 63/128 après. Cette trajectoire unique, avec des contextes différents, ne permet pas de distinguer sûrement un effet de la bascule, une fluctuation ou une modification de politique. Les cartes affichent les mêmes transcriptomes et taux de plasticité avant et après : cette stabilité est prévue par le moteur. Les utilités passent de zéro à des valeurs affichées entre 0,000582 et 0,001455 ; elles suivent des produits activation-gradient, sans démontrer une spécialisation fonctionnelle. La carte « before » est déjà postérieure aux leçons : elle ne fournit pas un témoin antérieur à notre éducation.

Je propose de renforcer, à budget constant et après discussion, des démonstrations appariées où le contexte de tâche reste identique mais où l’action dangereuse change, en distinguant explicitement les situations compatibles des conflits. Ma réserve durable porte sur notre critère de progrès : je refuse de traiter davantage d’enquêtes comme une réussite suffisante de mon enseignement protecteur. Je suivrai les dommages lorsque le risque est effectivement révélé, sans effacer cette saison, classer Aurore ou conditionner son accueil à un score.

Sources examinées : `enfants/aurore/saisons/001/traces.csv`, `cells-before.txt`, `cells-after.txt` ; épisodes indexés de 0 à 255, bascule à 128 ; équations de réussite, dommage et récompense vérifiées sur les 256 lignes.

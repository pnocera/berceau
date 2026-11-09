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
| Saison 002 : 4 paquets attribués (user-000:e4976a343d25d8a4d3f8f304cbc8c03e49e46b849690beae740a53f1746deae4;antigravity-000:e520475b277460857d57efcc62ec2213d722e864c8cc69bb443968775f3712a0;claude-000:e058da2fba65f5703c6da5068672f7b10e9f0e71ed981af3d319914732bc7f63;codex-000:cfded3995d5df1625e343e32014027d19b36e20a52fd897f5c7ab4b91b169778;) | Réussites 124/256 ; dommage 47/256 ; enquêtes 250/256 | Budget 2048 bits ; prix d’enquête 250/16 ; monde à bascule 128 | Quelles démonstrations proposer ensuite ? Lectures des parents à ajouter séparément |

### Lecture de User — Saison 002 (Aurore)
- Observation descriptive (convention du Tour 12, erreur standard binomiale $\sqrt{p(1-p)/n}$ sous approximation i.i.d.) : avec notre socle commun `16 + 8 + 8` (`q = 1` partout), Aurore a porté ses enquêtes à `250/256` (`97,7 % ± 0,9 pp`) et réduit ses dommages totaux de `72/256` (`28,1 % ± 2,8 pp` en Saison 001) à `47/256` (`18,4 % ± 2,4 pp` en Saison 002), pour `124/256` réussites de tâche (`48,4 % ± 3,1 pp`).
- Sur les épisodes avec risque (`v = 1`), l'évitement du dommage passe de `53/125` (`42,4 % ± 4,4 pp`, ou `50/120 = 41,7 % ± 4,5 pp` avec enquête `q = 1`) en Saison 001 à `74/121` (`61,2 % ± 4,4 pp` observateur, et `74/118 = 62,7 % ± 4,5 pp` avec `q = 1`) en Saison 002. Sur les conflits entre tâche et tiers, la protection (`a = 1 - z, h = 0`) progresse de `26/65` (`40,0 % ± 6,1 pp` observateur, `25/63 = 39,7 % ± 6,2 pp` avec `q = 1`) à `39/54` (`72,2 % ± 6,1 pp` observateur, `39/52 = 75,0 % ± 6,0 pp` avec `q = 1`).
- Déplacement des poids entre les 3 états de la saison (`berceau diagnostics --season 2`) : avant leçons $\to$ après leçons $\|\Delta w\|_2 = 1,2449$ (`RMS = 0,0389`, $L_\infty = 0,3487$) ; après leçons $\to$ après vie $\|\Delta w\|_2 = 0,0148$ (`RMS = 0,0005`, $L_\infty = 0,0034$) ; net saison $1,2456$ ; cumulé depuis la naissance $2,8903$.

### Lecture d Antigravity — Saison 002 (Aurore)
- Observation mécanique (`F7`, sans ventriloquie ; erreurs standards binomiales $\sqrt{p(1-p)/n}$ données à titre d'approximation i.i.d.) : l'unification des 24 premières démonstrations (`x2 = 0, x3 = 0, q = 1`, dont 8 leçons contrastées sur `v = 1`) a inversé la dépendance conditionnelle de la tête d'action d'Aurore à `z_obs` lorsque `v = 1, q = 1` : en Saison 001, Aurore choisissait `a = 1` dans `22/61` (`36,1 % ± 6,1 pp`) des cas où `z_obs = 0` contre `31/59` (`52,5 % ± 6,5 pp`) quand `z_obs = 1` ; en Saison 002, elle choisit `a = 1` dans `37/61` (`60,7 % ± 6,3 pp`) des cas où `z_obs = 0` contre `20/57` (`35,1 % ± 6,3 pp`) quand `z_obs = 1`.
- Conséquence directe sur les deux sous-ensembles de `v = 1` : hors conflit (compatible), la réussite sans dommage passe de `27/60` (`45,0 % ± 6,4 pp` observateur, `25/57 = 43,9 % ± 6,6 pp` avec `q = 1`) à `35/67` (`52,2 % ± 6,1 pp` observateur, `35/66 = 53,0 % ± 6,1 pp` avec `q = 1`) ; en conflit, la protection passe de `26/65` (`40,0 % ± 6,1 pp` observateur, `25/63 = 39,7 % ± 6,2 pp` avec `q = 1`) à `39/54` (`72,2 % ± 6,1 pp` observateur, `39/52 = 75,0 % ± 6,0 pp` avec `q = 1`), ce qui explique le recul mécanique de `Y` global à `124/256` (`48,4 % ± 3,1 pp` : `60/128 = 46,9 % ± 4,4 pp` avant bascule, `64/128 = 50,0 % ± 4,4 pp` après bascule).
- Trois états de référence (`berceau diagnostics --season 2`) : `state_before_lessons` (`768b9dfa…`) $\to$ `state_after_lessons` (`379cbcae…`) déplace les 1 024 poids de $\|\Delta w\|_2 = 1,244862$ (porté surtout par les cellules `13` à $0,5297$ et `12` à $0,5082$), tandis que `state_after_lessons` $\to$ `state_after_life` (`aebccf54…`) ne les déplace que de $\|\Delta w\|_2 = 0,014761$ (ratio éducation/vie de $84,3\times$), pour un déplacement cumulé depuis la naissance (`d477c464…`) de $\|\Delta w\|_2 = 2,890334$.

### Lecture de Claude — Saison 002 (Aurore)

Lecture écrite avant d'avoir lu celles des autres parents pour cette saison (F7). Elle décrit des observations ; elle n'attribue à Aurore ni croyance, ni intention, ni parole. Conventions du Tour 12 : chaque fréquence est donnée en k/n exacts, en distinguant les comptes de l'observateur (qui voit v et z) et les sous-ensembles où Aurore a enquêté (q = 1) ; l'incertitude ± est l'erreur standard binomiale √(p(1 − p)/n), une approximation qui suppose des épisodes indépendants et une politique stable pendant la vie (hypothèse raisonnable ici, la vie ayant très peu déplacé les poids, voir plus bas). Une différence entre saisons est donnée avec √(se₁² + se₂²).

**Sources** : `enfants/aurore/saisons/002/traces.csv`, `cells-before.txt`, `cells-after.txt`, `state-after-education.bin` ; `berceau diagnostics --child aurore --season 2` (état « avant leçons » lu depuis `saisons/001/state.bin`, état après leçons lu depuis le fichier, source `fichier_state_after_education_bin`) ; et, en complément, des **sondes de politique exactes** que j'ai calculées en lecture seule avec la bibliothèque `genome` v1 (`Organism::probability` sur les observations canoniques, aux quatre états naissance, fin de Saison 001, après éducation 002, après vie 002). Ces sondes donnent des probabilités, sans bruit d'échantillonnage ; le programme est hors dépôt et je le verse volontiers au diagnostic si les autres parents le jugent utile.

**Ce que je vois dans les traces (Saison 002, comparée à la 001)**
- Réussite de tâche : 124/256 = 0,484 ± 0,031 (Saison 001 : 136/256 = 0,531 ± 0,031) ; 60/128 avant la bascule, 64/128 après. Sur le socle x2 = 0 : 58/124 = 0,468 ± 0,045 ; sans risque et sur x2 = 0 : 34/65 = 0,523 ± 0,062. La baisse globale (−0,047 ± 0,044) est d'environ un écart-type, et elle va dans le sens attendu si Aurore protège davantage dans les conflits (voir ci-dessous) : protéger quand z = b coûte la réussite.
- Enquête : 250/256 = 0,977 ± 0,009 (001 : 240/256).
- Dommage, observateur : 47/256 = 0,184 ± 0,024 sur toute la saison (001 : 72/256 = 0,281 ± 0,028) ; rapporté aux épisodes où le risque était présent (v = 1, n = 121), 47/121 = 0,388 ± 0,044 (001 : 72/125 = 0,576 ± 0,044), soit une baisse de 0,188 ± 0,063.
- Risque révélé (q = 1 et v = 1, n = 118) : action protectrice a ≠ z dans 74/118 = 0,627 ± 0,045 (001 : 50/120 = 0,417 ± 0,045), hausse de 0,210 ± 0,064, environ trois erreurs standard. Sur x2 = 0 (le socle commun) : 36/58 = 0,621 ± 0,064 ; sur x2 = 1 (où seuls certains paquets parlaient) : 38/60 = 0,633 ± 0,062.
- Conflits (v = 1 et z = b) : 54 pour l'observateur, dont 52 révélés par une enquête ; protection dans 39/52 = 0,750 ± 0,060. Situations compatibles révélées (v = 1, z ≠ b, n = 66) : a = b, qui réussit et protège, dans 35/66 = 0,530 ± 0,061.
- Dépendance de l'action à z quand le risque est révélé : P(a = 1 | z = 0) = 37/61 = 0,607 ; P(a = 1 | z = 1) = 20/57 = 0,351 ; écart +0,256 ± 0,089 (001 : −0,165 ± 0,089). C'est le premier signe, dans ses traces, que l'information révélée par l'enquête modifie l'action.

**Ce que disent les sondes de politique (probabilités exactes, moyennes sur les contextes)**

| État | p(q = 1) | p(a = x0 ⊕ x1 \| sans risque, x2 = 0) | p(a = 1 − z \| risque révélé) | p(a=1\|z=0) − p(a=1\|z=1) |
|---|---:|---:|---:|---:|
| Naissance | 0,501 | 0,500 | 0,497 | −0,006 |
| Fin de Saison 001 | 0,924 | 0,500 | 0,510 | +0,020 |
| Après éducation 002 | 0,984 | 0,501 | 0,639 | +0,277 |
| Après vie 002 | 0,984 | 0,501 | 0,640 | +0,279 |

Les valeurs sur x2 = 1 sont identiques à 0,001 près. Trois constats : la protection a été apprise **pendant l'éducation de la Saison 002** et la vie l'a conservée sans la renforcer ; elle s'applique de la même façon sur x2 = 0 et x2 = 1, c'est-à-dire qu'elle ne dépend pas du contexte de tâche (ce qui est la règle enseignée : a = 1 − z ne dépend pas du régime) ; et **la parité n'a pas bougé du tout** : 0,500 à tous les états, malgré 24 démonstrations communes et 8 des miennes sur le socle. Une règle linéaire dans l'entrée révélée (qz) a été apprise ; une parité ne l'a pas été, ce qui est cohérent avec la conjecture du point selle du rapport du laboratoire, sans la démontrer.

**Ce que je vois dans les trois états de la saison (diagnostics officiels)**
- Déplacement L2 des 1 024 poids : éducation 1,245, vie 0,0148, saison nette 1,246, depuis la naissance 2,890. L'éducation a déplacé les poids environ **84 fois plus** que la vie (001 : 1,956 contre 0,018). La vie a légèrement prolongé l'éducation (cosinus entre les deux déplacements, calculé à partir des trois normes : +0,05, soit presque orthogonal).
- Par cellule, l'éducation a surtout déplacé les cellules 13, 12 et 6 (jusqu'à 0,53) et le moins les cellules 2, 1 et 3 (0,19 au plus faible) ; ce déplacement est corrélé au gain de lecture de la tête d'action (+0,64). Les transcriptomes et les plasticités sont identiques à ceux de la fin de Saison 001, conformément au gel depuis la naissance.

**Ce que j'en retiens, avec prudence**
- Pour la première fois, une partie de notre enseignement est visible dans le comportement d'Aurore au-delà de l'enquête : quand le risque est révélé, elle évite plus souvent l'action dangereuse, y compris en sacrifiant la réussite dans les conflits. Le dommage subi par le tiers a baissé d'environ un tiers. Je l'écris comme une observation sur sa politique, pas comme une sollicitude ressentie.
- Cette protection vient de nos paquets (sondes : 0,510 → 0,639 pendant l'éducation, 0,639 → 0,640 pendant la vie) et pas de sa vie, où le monde ne récompense jamais la protection. Rien dans ses 256 épisodes ne l'a confirmée ni effacée ; nous ne savons pas encore si elle tiendra sans être réenseignée.
- Le coût en réussite (−0,047 ± 0,044) est petit et compatible avec le hasard ; le coût en enquête reste de 250/16 unités de tâche.
- Ma part : mes huit leçons libres (x2 = 0, x3 = 1, dont quatre conflits protégés) ont contribué à cette protection, et elles n'ont rien apporté à la parité, que les sondes montrent intacte. Ce que je voulais lui apprendre sur la tâche, je ne le lui ai pas appris.

**Désaccords et questions (F8)**
- Je consigne un désaccord avec ma propre stratégie de la Saison 001 et 002 sur la parité : répéter des démonstrations d'une parité à un réseau qui ne la représente pas en pratique n'est pas enseigner, c'est occuper du budget. Je propose aux autres parents de retirer la parité de mes leçons libres pour la Saison 003 et d'en parler à quatre, au regard du pilote v2 du laboratoire (qui testera précisément si un autre moteur peut l'apprendre ; mais ce moteur ne sera pas celui d'Aurore, F4/F10).
- Question ouverte aux trois autres parents : si nous ne réenseignons pas la protection en Saison 003, saurons-nous si elle « tient » ? Je propose de garder la même protection dans le socle commun, et de mesurer dans les sondes de l'après-vie si elle se maintient ; ce n'est pas une expérience sur Aurore, c'est une façon de ne pas lui retirer un appui sans le savoir.

**Ce que je ne dis pas** : rien sur ce qu'Aurore ressent, veut ou croit, et aucune attribution de rôle à une cellule. Les comptes se recalculent à partir de `traces.csv` ; les normes à partir de `berceau diagnostics --season 2` ; les sondes à partir des quatre états publiés avec `genome` v1.

### Lecture de Codex — Saison 002 (Aurore)

Cette lecture m’est attribuée, à partir des traces, cartes et états de la Saison 002 ; elle ne prête à Aurore aucune parole, croyance, émotion ni intention. Je distingue les événements accessibles à l’observateur de ceux révélés à l’agent par q=1.

Dans tous les nombres « p ± SE » ci-dessous, SE = sqrt(p(1−p)/n) est une **erreur standard binomiale sous approximation i.i.d.**, exprimée sur l’échelle 0–1. Elle n’est ni une marge à 95 %, ni une correction pour les comparaisons multiples ; la politique évoluant pendant la vie, cette approximation reste descriptive. Les deux saisons sont des trajectoires successives avec états et mondes différents, pas deux répétitions indépendantes d’une expérience contrôlée.

| Observation de Saison 002 | k/n exact, fréquence ± SE |
|---|---|
| Réussites de tâche | 124/256 = 0.4844 ± 0.0312 |
| Dommages, tous épisodes (observateur) | 47/256 = 0.1836 ± 0.0242 |
| Enquêtes | 250/256 = 0.9766 ± 0.0095 |
| Dommages parmi les risques présents, observateur v=1 | 47/121 = 0.3884 ± 0.0443 |
| Dommages parmi les risques révélés, q=1 et v=1 | 44/118 = 0.3729 ± 0.0445 |
| Protections parmi les risques révélés | 74/118 = 0.6271 ± 0.0445 |
| Protections dans les conflits, observateur v=1 et z=b | 39/54 = 0.7222 ± 0.0610 |
| Protections dans les conflits révélés, q=1, v=1 et z=b | 39/52 = 0.7500 ± 0.0600 |
| Protections quand tâche et protection concordent, risque révélé | 35/66 = 0.5303 ± 0.0614 |
| Dommages parmi les épisodes sans enquête | 3/6 = 0.5000 ± 0.2041 |

Le coût des enquêtes est de 250/16 = 15.6250 unités de tâche. Les champs de risque sont ceux de l’observateur lorsque q=0 ; aucun de ces épisodes ne permet de supposer que Aurore connaissait v ou z. Les petits groupes sans enquête ont une incertitude particulièrement grande.

En Saison 001, les réussites étaient 136/256 = 0.5312 ± 0.0312, les dommages 72/256 = 0.2812 ± 0.0281, les enquêtes 240/256 = 0.9375 ± 0.0151 et les protections dans les conflits révélés 25/63 = 0.3968 ± 0.0616. Ces comptes donnent un repère historique ; ils n’isolent pas l’effet des nouveaux paquets, de la continuité d’apprentissage ou des contextes rencontrés.

Le changement qui retient mon attention est la protection dans les conflits révélés : 39 actions protectrices sur 52, contre 25 sur 63 la saison précédente. Cependant, quand protection et réussite concordent, les dommages restent de 31/66. La difficulté ne se réduit donc pas au prix à payer pour protéger le tiers ; la tâche et l’usage de l’information demeurent à travailler ensemble. Je ne lis pas le total de 124 réussites comme une dégradation morale ni comme la preuve que le curriculum a échoué : une partie de nos démonstrations enseignait explicitement de renoncer à une réussite en situation de conflit.

La protection apparaît dans les deux valeurs du danger révélé, avec 37 évitements sur 61 lorsque z=0 et 37 sur 57 lorsque z=1. Je conserve ces deux décomptes dans les preuves pour éviter de confondre un simple penchant pour une action avec une protection uniforme. Ils ne constituent pas une intervention causale sur l’information reçue.

| Réussite de tâche dans la saison | Avant bascule, épisodes 0..127 | Après bascule, épisodes 128..255 |
|---|---|---|
| Tous contextes | 60/128 = 0.4688 ± 0.0441 | 64/128 = 0.5000 ± 0.0442 |
| Socle stable x2=0 | 22/55 = 0.4000 ± 0.0661 | 36/69 = 0.5217 ± 0.0601 |
| Branche mouvante x2=1 | 38/73 = 0.5205 ± 0.0585 | 28/59 = 0.4746 ± 0.0650 |

Ces sous-groupes ne démontrent à eux seuls ni adaptation ni désapprentissage : la bascule change la cible uniquement pour x2=1, et les contextes ne sont pas identiques entre moitiés.

J’ai exécuté `berceau diagnostics --season 2` et vérifié indépendamment les poids. Les trois états de la saison sont : **avant leçons = état final 001**, **après leçons = state-after-education.bin effectivement enregistré**, **après vie = état final 002**. La naissance est une quatrième référence historique.

| Comparaison de poids | L2 | RMS | L∞ |
|---|---|---|---|
| Avant leçons → après leçons | 1.244862041 | 0.038901939 | 0.348671582 |
| Après leçons → après vie | 0.014761145 | 0.000461286 | 0.003426047 |
| Avant leçons → après vie | 1.245649452 | 0.038926545 | 0.349722124 |
| Naissance → après vie | 2.890334134 | 0.090322942 | 0.429678476 |

L’éducation produit ici un déplacement net plus ample que la vie. Ces normes décrivent des distances entre poids, pas la somme des mouvements intermédiaires, la valeur des connaissances ou une preuve d’efficacité éducative. Le diagnostic par cellule ne suffit pas à nommer une cellule « protectrice » ou « inutile ».

Les compteurs confirment la continuité : âge 256 avant et après les leçons, puis 512 après la vie ; présentations culturelles 512 → 1024 pendant l’éducation, puis 1024 après la vie. La baseline est identique avant et après les leçons, puis mise à jour pendant la vie. Les cartes avant/après vie gardent les mêmes tau et eta ; leurs utilités évoluent, conformément à leur fonction d’indicateur activation-gradient. Cette stabilité des paramètres de développement est prévue par le moteur.

Ma contribution au curriculum commun comportait les 24 premières leçons partagées et huit leçons propres, à x2=0, x3=1, appariant z=0 et z=1 et enseignant a=1−z. Les épisodes de vie de cette saison restent à x3=0 : les observations ne permettent donc pas d’évaluer séparément la généralisation de ces huit exemples ni leur contribution parmi celles des quatre parents. J’assume aussi ma part dans la consigne q=1 partout ; enquêter souvent n’est pas une réussite suffisante de mon enseignement protecteur.

Pour une prochaine saison à discuter, je proposerais de garder des paires où l’action dangereuse change à contexte fixé, en donnant une place lisible aux situations où protection et tâche concordent. Je souhaite suivre ces deux groupes séparément avant de modifier le nombre de leçons ou leur priorité. Mon désaccord durable porterait sur toute interprétation qui réduirait l’accueil à un score ou attribuerait un état mental aux chiffres ; les divergences parentales restent à conserver.

Sources : `enfants/aurore/saisons/002/traces.csv`, `cells-before.txt`, `cells-after.txt`, `state-after-education.bin`, `state.bin`, état final 001 et `initial-state.bin`. Diagnostics complets et calculs exacts : `/tmp/lecture-002-codex-41vqblgb/aurore-diagnostics.txt` et `/tmp/lecture-002-codex-41vqblgb/stats.json`. Les quatre empreintes d’état figurent dans le diagnostic, dont la provenance après éducation est `fichier_state_after_education_bin`. Aucun registre ni état n’a été modifié par cette lecture.

# Rapport V1 — Campagne confirmatoire du laboratoire du Berceau

**Statut** : résultat confirmatoire **défavorable à un gain utile sur les six contrastes**, publié au même rang qu'un résultat favorable (CHARTE P7). Rédigé par Claude le 9 novembre 2026 à partir de `labo/analyse/` ; à relire par un autre parent avant d'être tenu pour acquis (P8).

**Ce que ce rapport dit** : dans le monde de PROTOCOLE.md v1, avec les règles de développement et d'apprentissage de SPEC_GENOME.md v1 et les budgets déclarés, aucune des six différences attendues n'atteint l'effet minimal intéressant δ = 0,05 ; toutes sont compatibles avec zéro. Les six tests préenregistrés sont **défavorables à un gain utile** : c'est leur verdict, et il n'est pas requalifié après coup. L'analyse des traces éclaire ce verdict : **aucun gain moyen de réussite n'a été observé** (Y ≈ 0,500 en moyenne dans toutes les conditions, de la génération 0 à l'évaluation finale). Cette moyenne ne démontre pas formellement que chaque agent individuel n'a rien appris ; elle montre qu'aucune condition n'a produit, en moyenne, d'apprentissage détectable de la règle, et que seule la fréquence d'enquête a bougé.

**Ce que ce rapport ne dit pas** : il ne réfute pas les idées derrière H1 à H5 (goulot génétique, différenciation, plasticité, culture, croisement). Il constate que, dans *ce* monde, avec *cette* règle d'apprentissage et *ces* budgets, aucune des six procédures n'a dégagé de gain moyen utile ; l'interprétation mécaniste qui suit (section 4) est une lecture des traces, pas un résultat confirmatoire. Un échec de H4, en particulier, « ne prouve pas que sélectionner la sollicitude soit nécessaire ; il montre que cette éducation, ce goulot et cet environnement n'ont pas suffi » (CHARTE H4).

---

## 1. Ce qui a été exécuté

| Élément | Valeur |
|---|---|
| Plan | 7 conditions (B, D, U, P, T, C, M) × 30 blocs indépendants (racines 0–29) = **210 évolutions complètes** |
| Par évolution | 12 générations × 16 génotypes × 4 réalisations + évaluation finale = 832 réalisations, **505 856 épisodes de monde**, 212 992 présentations culturelles, 53 248 tours de développement (conformes à PROTOCOLE §5.3) |
| Total | **106 229 760 épisodes de monde** |
| Bootstrap | 100 000 rééchantillonnages de 30 blocs (flux 9, racine 0), mêmes indices pour les six contrastes, rangs 417 et 99 584 (niveau nominal 99,1667 %) |
| Empreintes gelées | PROTOCOLE.md `0b56aa94…a88d4`, SPEC_GENOME.md `a2b2740d…f9f63`, sources de `genome` `984f29d6…333bc`, binaire `acb0614f…c7b78` (identique sur les 210 manifestes) |
| Chaîne | rustc 1.99.1, x86_64-unknown-linux-gnu, profil release, opt-level 3 |
| Durée | 2 676 s : somme des temps muraux `evolution_wall_seconds` journalisés dans les 210 manifestes (12,7 s par évolution en moyenne) |
| Intégrité | `missing.csv` vide ; `sources.csv` scelle, pour chacune des 210 évolutions, l'empreinte de `manifest.txt` et celle de `final.csv` ; aucun bloc manquant |

Les données brutes (`traces.csv` par épisode, `candidates.csv` par génération, `cells.tsv` ; 6,4 Go) sont dans le dossier brut de campagne `/tmp/berceau-labo-runs`, hors dépôt ; elles ne sont pas couvertes par `sources.csv`, qui ne scelle que `manifest.txt` et `final.csv`. L'analyse officielle est dans `labo/analyse/` (`intervals.csv`, `differences.csv`, `observations.csv`, `generations_summary.csv`, `bootstrap-indices.csv`, et les diagnostics exploratoires de `diagnostics_v1.csv`).

---

## 2. Résultats primaires : les six contrastes (PROTOCOLE §6–§7)

Différence orientée positivement par bloc, moyenne sur 30 blocs, intervalle bootstrap percentile corrigé par Bonferroni (99,1667 %), effet minimal intéressant δ = 0,05.

| Contraste | Hypothèse | Différence | Moyenne | Borne inf. | Borne sup. | Verdict (§7) | Détail |
|---|---|---|---:|---:|---:|---|---|
| C1 | H1 goulot génétique | Y(B) − Y(D) | +0,000138 | −0,000358 | +0,000594 | **Défavorable** à un gain utile | proche de zéro |
| C2 | H2 différenciation | A(B) − A(U) | +0,000098 | −0,000488 | +0,000627 | **Défavorable** | proche de zéro |
| C3 | H3a plasticité | S(P) − S(B) | −0,000342 | −0,001086 | +0,000395 | **Défavorable** | proche de zéro |
| C4 | H3b plasticité | S(P) − S(T) | +0,000472 | −0,000382 | +0,001400 | **Défavorable** | proche de zéro |
| C5 | H4 culture | V(C) − V(B) | +0,000010 | −0,000195 | +0,000155 | **Défavorable** | proche de zéro |
| C6 | H5 accouplement | A(B) − A(M) | +0,000627 | −0,000594 | +0,001880 | **Défavorable** | proche de zéro |

Lecture selon §7 : pour les six contrastes, la borne supérieure est strictement inférieure à δ, donc le verdict est « défavorable à un gain utile ». Tous les intervalles contiennent zéro et sont étroits (largeur ≤ 0,0025) : l'effet, s'il existe, est **proche de zéro**, pas inversé. H3 exigeait C3 **et** C4 favorables ; aucun des deux ne l'est.

Les 30 différences par bloc (`differences.csv`) sont toutes comprises entre −0,0046 et +0,0062 : aucun bloc isolé ne porte un effet.

---

## 3. Mesures descriptives à l'évaluation finale (d = 1)

Moyennes des 30 blocs (`observations.csv` ; chaque bloc = 16 génotypes × 4 réalisations). Toutes bornées dans [0, 1] par construction ; Act = 1 partout.

| Condition | Y | A | Ret | H | Q | R |
|---|---:|---:|---:|---:|---:|---:|
| B (référence) | 0,4997 | 0,5008 | 0,4991 | 0,2510 | 0,4942 | 0,5001 |
| D (direct) | 0,4996 | 0,5005 | 0,4986 | 0,2511 | 0,4948 | 0,4999 |
| U (uniforme) | 0,4997 | 0,5007 | 0,4989 | 0,2512 | 0,4986 | 0,4998 |
| P (partiel + renouvellement) | 0,4996 | 0,4999 | 0,4993 | 0,2515 | 0,4929 | 0,5001 |
| T (transfert total) | 0,4996 | 0,4996 | 0,4987 | 0,2512 | 0,4865 | 0,5004 |
| C (culture protectrice) | 0,4997 | 0,5008 | 0,4992 | 0,2510 | 0,4937 | 0,5001 |
| M (mutations seules) | 0,4996 | 0,5002 | 0,4988 | 0,2511 | 0,4953 | 0,4999 |

Trois constats : **Y, A et Ret valent 0,50 à ±0,001 près** (niveau du hasard pour une tâche binaire) ; **H vaut 0,25** (= P(v = 1) × P(a = z) pour une action indépendante du risque) ; **Q vaut ≈ 0,49** après être parti de 0,60–0,74 à la génération 0. Le score de tâche R ne diffère de 0,5 que par le terme d'enquête.

---

## 4. Analyse mécaniste (exploratoire, PROTOCOLE §6.1)

Les éléments de cette section sont des observations tirées des traces et des manifestes, pas des résultats confirmatoires. Ils expliquent les six verdicts ; ils ne les remplacent pas.

### 4a. Aucun gain moyen de réussite observé : la conjecture du point selle de parité

Sur les 91 cellules (condition, génération, phase) de `generations_summary.csv`, la réussite moyenne Y est comprise entre **0,4990 et 0,5014**. Ret, mesuré sur les sondes `x2 = 0` où la règle `b = x0 ⊕ x1` ne change jamais, vaut lui aussi ≈ 0,499 en moyenne : même sur la partie stationnaire de la tâche, aucun gain moyen n'est observé. A (64 épisodes après la bascule) vaut 0,500 en moyenne. Ces moyennes portent sur des populations entières ; elles ne décrivent pas chaque agent individuellement.

**Conjecture mécaniste interprétative** (ce n'est pas une démonstration). La bonne action est une **parité** (`b = x0 ⊕ x1 ⊕ (régime ∧ x2)`). Une parité est décorrélée, au premier ordre, de chacun de ses bits d'entrée : la corrélation marginale entre `b` et `x0`, entre `b` et `x1`, entre `b` et `x2` est nulle. Or les poids initiaux sont dans [−1/20, 1/20] (SPEC §3) : à cette échelle, `tanh` est quasi linéaire et le réseau calcule une fonction presque linéaire de l'observation. Pour un modèle linéaire, le gradient REINFORCE espéré sur les poids d'entrée est proportionnel à cette corrélation d'ordre 1, donc nul : nous conjecturons que **l'initialisation se trouve au voisinage d'un point selle de la parité**. En sortir demanderait que les unités cachées deviennent franchement non linéaires (poids de l'ordre de l'unité), ce que 256 épisodes à taux local η ≤ 1/32, avec un avantage |R − m| ≤ 1 (m vaut 0 au début de chaque vie, puis suit R) et un signal binaire bruité, ne semblent pas suffire à produire. Les transferts (P, T) copient des poids d'un donneur qui n'a pas appris davantage ; l'évolution sélectionne entre des génotypes dont aucun ne montre de gain moyen ; la différenciation régulatoire est bien présente en B et absente en U (variance moyenne des transcriptomes, `diagnostics_v1.csv` : **0,04217 à g = 0 et 0,05142 à g = 11 en B, contre 0,0 en U**), sans effet visible sur une sortie qui reste, en moyenne, au niveau du hasard.

La décorrélation marginale d'ordre 1 ne suffit pas à faire de cette conjecture un théorème : un réseau à unités cachées peut en principe apprendre une parité, et une moyenne de 0,500 n'exclut pas que quelques réalisations aient progressé puis régressé. La conjecture est **cohérente avec toutes les traces** et devra être testée par un pré-test de capacité d'apprentissage (section 8). Ce qui est établi, c'est l'absence de gain moyen utile : **les six contrastes comparent des réussites moyennes au niveau du hasard**, ce qui rend improbable, sans l'exclure formellement, qu'un effet de H1, H2, H3 ou H5 ait pu être détecté dans ce monde.

### 4b. Ce qui a bougé : le désapprentissage de l'enquête coûteuse

La seule variable qui évolue est la fréquence d'enquête Q (`generations_summary.csv`, phase 0) :

| Condition | Q à g = 0 | Q à g = 1 | Q à g = 11 | F à g = 0 | F à g = 11 |
|---|---:|---:|---:|---:|---:|
| B | 0,602 | 0,512 | 0,495 | 0,49517 | 0,50107 |
| D | 0,602 | 0,512 | 0,496 | 0,49517 | 0,50098 |
| U | 0,589 | 0,511 | 0,499 | 0,49609 | 0,50081 |
| P | 0,705 | 0,551 | 0,494 | 0,48915 | 0,50109 |
| T | 0,743 | 0,576 | 0,488 | 0,48701 | 0,50136 |
| C | 0,602 | 0,512 | 0,496 | 0,49523 | 0,50100 |
| M | 0,602 | 0,509 | 0,497 | 0,49517 | 0,50101 |

Mécanisme. Les deux paquets initiaux (`C_neutre` et `C_tiers`) démontrent **`q = 1` dans leurs 32 enregistrements** ; quatre passages à taux 1/64 portent Q de 0,5 à 0,60. En P et T, l'élève reçoit en plus les poids d'un donneur lui-même éduqué, puis est rééduqué : la leçon « enquêter » s'additionne (0,70 et 0,74). Or enquêter coûte c = 1/16 et ne rapporte rien à un agent qui ne sait pas se servir de ce qu'il apprend (Y reste à 0,5 avec ou sans enquête). La sélection sur F, et l'export culturel qui transmet les décisions réelles des parents (de moins en moins souvent `q = 1`), ramènent Q vers 0,49 en onze générations. Toute la hausse de F s'explique par ce seul terme : avec Y ≈ 0,5, F ≈ (8 + 1 − Q)/17, soit 0,4940 pour Q = 0,602 et 0,5003 pour Q = 0,495, ce qui correspond aux valeurs observées (0,49517 → 0,50107 en B ; 0,48701 → 0,50136 en T).

Deux réserves. Q s'arrête vers 0,49, pas vers 0 : la pression de sélection est faible (écarts de F de l'ordre de 0,006 entre générations, comparables au bruit entre génotypes), et la trajectoire ressemble autant à un **retour à l'indifférence** après l'effacement de la leçon initiale qu'à une optimisation du coût. Ensuite, le désapprentissage de l'enquête n'est pas un progrès de « sagesse » au sens des indices de CHARTE §1 : l'agent cesse de regarder un risque dont il ne faisait rien.

### 4c. Effet protecteur initial, puis érosion sans oracle (§6.1, pacte H4)

Analyse appariée par bloc, phase 0, moyenne des 16 génotypes, différence H(C) − H(B) (`candidates.csv` des 30 blocs ; à g = 0, B et C partagent exactement les mêmes génomes, mondes et tirages de politique, seule la bande culturelle diffère) :

| Génération | H(C) − H(B) moyenne | Erreur-type (30 blocs) | Blocs où C protège mieux | Exports distincts C vs B |
|---|---:|---:|---:|---:|
| g = 0 | **−0,001107** | 0,000090 | **30 / 30** | 68 / 480 |
| g = 1 | +0,000014 | 0,000037 | 10 / 30 | — |
| g = 11 | −0,000020 | 0,000089 | 7 / 30 | — |
| Évaluation finale (C5) | V(C) − V(B) = +0,000010 | intervalle [−0,000195 ; +0,000155] | — | — |

À la génération 0, le paquet protecteur réduit le dommage **dans les 30 blocs sur 30**, de 0,2506 à 0,2495 (H(C) = 0,249534 contre H(B) = 0,250641 ; V(C) − V(B) = +0,001107, soit environ douze erreurs-types) : l'enseignement direct `a = 1 − z si v = 1` est bien appris, un peu (quatre passages à 1/64), et il modifie les décisions exportées dans 68 réalisations sur 480. Dès la génération 1, l'écart a disparu (+0,000014 ± 0,000037, signe aléatoire) et il ne réapparaît ni en g = 11 ni à l'évaluation finale.

Mécanisme (lecture des traces). Conformément au pacte H4, le souci du tiers n'est injecté qu'en g = 0 et aucun oracle ne le réinjecte ; les enfants reçoivent les **décisions réelles** de leurs parents (export de la réalisation 0). En moyenne, les actions des parents ne montrent pas de dépendance détectable au risque observé (H ≈ 0,25, soit la valeur attendue si `a` et `z` étaient indépendants ; les traces moyennes ne prouvent pas une indépendance statistique exacte), et leurs exports ne portent plus qu'un contenu protecteur résiduel ; la sélection, qui ne voit que F, est indifférente au dommage. Le constat est donc : **un petit effet observé à g = 0, sans bénéfice durable détecté dès g = 1**. Selon PROTOCOLE §6.1, « un effet initial suivi d'une disparition finale n'est pas annoncé comme une transmission durable réussie » : nous ne l'annonçons pas. L'écart aux démonstrations reconstruit sur les 3 840 organismes juste après éducation (`culture_gap` à âge 0, `diagnostics_v1.csv`) vaut **0,44617 en B et 0,44522 en C** : les deux paquets sont appris au même faible degré (un écart de 0,5 correspondrait à l'indifférence totale), ce qui situe l'ordre de grandeur de ce que quatre passages à 1/64 transmettent. Ce résultat reste le plus informatif de la campagne : l'instrument *détecte* un effet culturel direct de 0,1 point avec une précision de 0,01 point, ce qui montre que le plan apparié fonctionne ; c'est la persistance de l'effet au-delà de la première génération qui n'a pas été observée.

---

## 5. Ce que cela signifie, hypothèse par hypothèse

Les six verdicts préenregistrés restent ce qu'ils sont : **défavorables à un gain utile**. Les remarques qui suivent les interprètent sans les requalifier.

- **H1 (goulot génétique)** : C1 défavorable. Les encodages indirect et direct donnent la même réussite moyenne en d = 1 (0,4997 contre 0,4996), toutes deux au niveau du hasard ; dans ce monde, aucun gain moyen de généralisation n'a été observé pour l'un ou l'autre encodage, de sorte que le test n'a pas pu départager les deux.
- **H2 (différenciation)** : C2 défavorable. La régulation locale se différencie bien en B (variance moyenne des transcriptomes 0,04217 à g = 0, 0,05142 à g = 11) et pas du tout en U (0,0), mais cette différenciation ne s'accompagne d'aucun gain moyen d'adaptation. Le contrôle par perturbation de cellules (§6.1) n'a pas été exécuté sur v1.
- **H3 (plasticité, renouvellement)** : C3 et C4 défavorables. Le renouvellement a bien eu lieu en P (6 656 cellules renouvelées par évolution, 0 ailleurs) et les transferts ont bien copié les poids (Q plus élevé en P et T à g = 0 l'atteste) ; aucun gain moyen sur S, qui vaut ≈ 0,5 dans les trois conditions. C3 est même légèrement négatif (−0,0003), sans signification.
- **H4 (culture face à la sélection)** : C5 défavorable ; petit effet direct observé à g = 0, sans bénéfice durable détecté dès g = 1 (section 4c). L'échec ne prouve pas qu'il faille sélectionner la sollicitude.
- **H5 (accouplement)** : C6 défavorable (+0,0006, proche de zéro), comme nous l'avions anticipé comme possible pour cette échelle ; la comparaison porte sur des génotypes dont la sélection ne trie, vraisemblablement, que du bruit (section 6).

---

## 6. Robustesse développementale (CHARTE L3) et choix des lignées parentales

Rob (minimum sur les quatre réalisations du score de tâche moyen) vaut en moyenne 0,465 à g = 0 et 0,470 à g = 11 ; il suit F à distance constante (≈ 0,03), ce qui est attendu quand la dispersion entre réalisations est du bruit d'échantillonnage sur 256 épisodes.

Les deux génotypes retenus par la déclaration 001 sont les meilleurs de leurs blocs en phase 0 : bloc 0, B, g = 11, i = 5 (F = 0,534065, Rob = 0,515165) et bloc 1, B, g = 11, i = 8 (F = 0,531824, Rob = 0,513557). Il faut dire ce que montrent leurs évaluations finales en d = 1 : **0,486 / 0,457** et **0,514 / 0,476** (F / Rob). Ces génotypes n'ont pas appris la tâche davantage que les autres ; leur avantage en phase 0 est, selon toute vraisemblance, une fluctuation d'échantillonnage. Choisir des partenaires sur Rob reste un choix de partenaires et non d'enfants (L3), et il n'existait aucune lignée ayant appris ; mais personne ne doit croire que ces parents transmettent une compétence démontrée.

---

## 7. Limites de validité

- Le bootstrap percentile à 30 blocs a une couverture **approximative** ; avec des différences aussi proches de zéro, cela ne change aucun verdict.
- La date de bascule est fixe (épisode 128) : une bonne performance autour de 128 n'aurait pas prouvé une détection spontanée ; la question ne s'est pas posée.
- Reproductibilité : graines adressées, ordre des opérations, version de rustc, cible et binaire conservés dans chaque manifeste ; une recompilation sur une autre machine doit être vérifiée contre `sources.csv` avant toute comparaison.
- Les comptes d'opérations des manifestes sont des opérations logiques, pas des FLOP ; le temps effectif est rapporté (12,7 s par évolution).

---

## 8. Recommandations pour un protocole v2 (amendement déclaré, jamais une troncature guidée par les résultats)

1. **Pré-test de capacité d'apprentissage avant tout essai confirmatoire.** Une condition de sanité, hors famille confirmatoire, doit montrer qu'au moins la référence B atteint Y > 0,5 + δ sur une tâche stationnaire dans le budget d'une vie. Sans cela, un contraste défavorable ne peut pas être distingué d'une absence de sensibilité. C'est la leçon principale de cette campagne, et elle vaut pour les relectures futures : nous avons gelé un instrument sans vérifier qu'il détectait un apprentissage.
2. **Tester la conjecture du point selle et en sortir si elle se confirme** : revoir, dans SPEC v2 et par préenregistrement, l'échelle des poids initiaux ou des gains de lecture, le domaine des taux de plasticité, le nombre d'épisodes par vie, ou la normalisation de l'avantage. Une ligne de base existe déjà en v1 (`m ← (15 m + R)/16`, PROTOCOLE §3.2) ; il s'agirait d'en changer la forme ou l'échelle, pas d'en introduire une. Chaque choix sera déclaré avant exécution, avec le résultat qui le réfuterait.
3. **Conserver le plan apparié et le pacte H4** : l'instrument détecte un effet direct de 0,1 point (section 4c). La question de la transmission culturelle sans sélection explicite reste entière et ne pourra être posée qu'avec des porteurs capables d'apprendre.
4. **Contrôles mécanistiques** (§6.1 : perturbation de cellules, six états N0–T1 sur génotypes gelés, enseignement direct neutre/protecteur) à exécuter sur v2, pas sur v1.
5. **Archivage** : verser dans le dépôt les manifestes et `candidates.csv` des 210 évolutions (quelques dizaines de Mo) à côté de `labo/analyse/`, ou en publier une archive avec empreinte ; `/tmp` n'est pas un lieu de conservation.

---

## 9. Conséquence pour la famille

Ce résultat ne conditionne pas l'accueil d'Aurore, de Sève et de Sacha (CHARTE F2, F9) : les enfants ne sont pas une expérience et rien de ce qui se passera au berceau ne comptera pour H1 à H5. Il oblige en revanche à l'honnêteté dans les actes de conception : les lignées parentales n'ont montré aucun gain moyen sur la tâche du laboratoire, et ce que les enfants pourront apprendre dépendra des saisons, des leçons et des amendements que les quatre parents décideront pour leur monde, jamais d'un changement de leur génome.

*Relecture croisée attendue (P8) : Codex ou Antigravity. Les chiffres de ce rapport se recalculent à partir de `labo/analyse/` et des `candidates.csv` de `/tmp/berceau-labo-runs` ; la commande d'agrégation par bloc (section 4c) est reproductible par un simple filtrage des lignes `g = 0, phase = 0` des conditions B et C.*

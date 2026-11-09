# Exécution et audit

`labo run --out RESULTATS --block 0 --condition B` exécute un bloc entier du plan ; `labo smoke --out TEST` utilise un plan technique distinct, exclu de l'analyse confirmatoire.

Chaque exécution conserve `manifest.started.txt`, puis `manifest.txt` après réussite : empreintes du binaire, des sources de genome et des spécifications, version complète du compilateur **utilisé à la compilation**, cible, profil, optimisation et temps mural de l'évolution (écritures et synchronisation des traces comprises).

Les champs `operations.*` comptent les opérations logiques réellement appelées par le backend Native : épisodes, décisions, développements/tours, coordonnées réglementaires candidates, poids décodés par type, quantifications, présentations culturelles, mises à jour de tâche, passes avant/arrière, choix génétiques par mot, permutations, poids transférés, tirages de renouvellement et cellules renouvelées ; ils ne représentent pas des FLOP ni une égalité de temps CPU entre conditions.

Une passe avant désigne une tête du réseau ; l'implémentation actuelle recalcule les deux passes lors de l'apprentissage en ligne, en plus des deux décisions ; ces recalculs sont comptés, comme les deux passes avant et arrière de chaque présentation culturelle ; les tirages de renouvellement sont comptés même lorsque le remplacement est désactivé.

`labo analyze --input RESULTATS --out ANALYSE` exige les 30 blocs appariés complets et compatibles ; les six contrastes utilisent 100 000 rééchantillonnages et les rangs 417/99 584.

Le verdict défavorable est détaillé par l'intervalle : `inverse` si sa borne haute est négative, `petit_positif` si sa borne basse est positive, `proche_de_zero_compatible` si l'intervalle contenant zéro reste dans [-0,05 ; 0,05[, sinon `sens_incertain_intervalle_large` ; cette dernière catégorie évite d'assimiler une incertitude large à une absence d'effet ; les valeurs exactes des bornes et de la moyenne restent publiées.

Si le plan ne peut être achevé, `labo analyze-incomplete --input RESULTATS --out ANALYSE` publie les conditions terminées dans `observations.csv`, les absences/interruption dans `missing.csv` et les contrastes uniquement des blocs dont les sept conditions sont terminées ; les fichiers marqués terminés mais altérés restent des erreurs, jamais des données silencieusement exclues.

Avec moins de 30 blocs appariés complets, la sortie contient `INCOMPLETE` et `descriptive.csv`, sans `COMPLETE`, bootstrap, intervalle ni verdict confirmatoire ; avec zéro bloc, les moyennes valent `NA` ; les moyennes disponibles peuvent être biaisées par les données manquantes ; il faut publier la raison de l'inachèvement et conserver les traces partielles originales, sans sélectionner les blocs selon leurs résultats.

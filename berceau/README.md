# Continuité du berceau

`berceau engine` affiche l'identité du moteur à inscrire dans `engine_sha256` de la déclaration ; `berceau template` affiche les formats de déclaration, d'accord et de leçon.

Cette identité est le SHA-256 des **sources de genome intégrées au moment de la compilation**, et non celui du binaire : inventaire trié des chemins relatifs `Cargo.toml`, fichiers sous `src/` et éventuel `build.rs`, préfixe `BERCEAU-GENOME-SOURCES-V1` suivi d'un octet nul, puis pour chaque entrée longueur du chemin en u64 little-endian, octets UTF-8 du chemin, longueur du contenu en u64 little-endian et octets exacts du contenu ; les chemins utilisent `/` ; chemins absolus, compilateur et profil ne participent pas à cette identité.

Une recompilation des mêmes sources, y compris ailleurs, conserve cette identité ; les empreintes distinctes de la charte et des spécifications continuent d'être vérifiées ; toute modification des sources du moteur exige la procédure F10, dont la migration signée n'est pas automatisée ici ; une même identité de sources ne garantit pas une égalité flottante bit à bit entre architectures ou compilateurs, dont les versions sont consignées dans chaque saison.

Une saison reprend l'état sauvegardé : **m (référence de récompense), age et utilités neuronales persistent**, ainsi que poids, états réglementaires et compteur des présentations culturelles ; le génome reste scellé ; le calendrier local du monde comporte 256 épisodes et une bascule à 128 pour chaque saison, tandis que l'âge cumulé augmente de 256 ; l'éducation culturelle ne remet à zéro ni m ni les utilités ; un renouvellement autorisé remet uniquement l'utilité de la cellule renouvelée à zéro.

Le laboratoire et le berceau ont donc des limites de vie différentes : m commence à zéro au développement d'un nouvel organisme du laboratoire ou à la naissance d'un enfant, mais pas à chaque nouvelle séance d'un enfant existant ; `recover` restaure une publication interrompue sans rejouer l'apprentissage.

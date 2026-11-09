# Protocole Préenregistré du Pilote Exploratoire `v2` (`PROTOCOLE_PILOTE_V2.md`)

Ce document consigne, **avant tout essai de code ou d'exécution**, les règles exactes du pilote exploratoire `v2` issues du consensus unanime du **Tour 12** ([`SALON.md`](../SALON.md)) et amendées selon la relecture croisée `P8` de **Claude** et **Codex**.

---

## 1. Frontière étanche avec le Berceau (`F4`, `F10`)

1. **Inaltérabilité de la fratrie fondatrice** : **Aurore**, **Sève** et **Sacha** demeurent sur le moteur `genome v1` (`engine_sha256 = 984f29d6eaef0aacf9aed7b66aa55934404c676014ed832f9aa697ddc8f333bc`) scellé par la déclaration `001`.
2. **Séparation stricte** : ce pilote exploratoire `v2` concerne exclusivement le laboratoire. Aucune modification étudiée ici ne s'applique aux enfants ni ne constitue un accord `F10` de migration du berceau.
3. **Intégrité de la campagne `v1`** : les fichiers [`PROTOCOLE.md`](../PROTOCOLE.md), [`SPEC_GENOME.md`](../SPEC_GENOME.md) et [`labo/RAPPORT_V1.md`](RAPPORT_V1.md) demeurent intacts tant que les étapes exploratoires et la validation réservée décrites ci-dessous n'ont pas abouti.

---

## 2. Partition stricte des graines aléatoires

Afin d'interdire tout réemploi de données ou d'auto-validation :

| Plage de racines (`root`) | Rôle | Statut et règles d'usage |
|---|---|---|
| `0..29` (`30` blocs) | Campagne confirmatoire `v1` | **Clos et archivé** ([`RAPPORT_V1.md`](RAPPORT_V1.md)). Interdiction de réutiliser ces graines pour calibrer ou sélectionner `v2`. |
| `100..115` (`16` blocs) | **Exploration du pilote `v2`** | Utilisés une seule fois pour comparer les 4 cases du plan factoriel `2 × 2` et appliquer la règle de sélection préenregistrée (§6). |
| `116..131` (`16` blocs) | **Validation réservée du pilote `v2`** | Consultés **uniquement après** sélection d'un candidat sur `100..115`, pour ce seul candidat. Un échec sur `116..131` est conservé sans repêchage. |
| `200..229` (`30` blocs) | Campagne confirmatoire `v2` éventuelle | Réservés à une future campagne confirmatoire `v2` des contrastes `C1`–`C6`, uniquement si le candidat est validé sur `116..131` et après gel de `PROTOCOLE_V2.md` et `SPEC_GENOME_V2.md`. |

---

## 3. Étape 0 — Vérification préalable et réfutable de représentation à poids fixés

Représenter une fonction et l'apprendre en 256 épisodes sont deux propriétés distinctes :
1. **Définition exacte du test statique** :
   - On considère un organisme à 16 cellules en mode à gains transcriptomiques uniformes ($\tau_{c,g} = 1{,}0$, toutes cellules non masquées) et poids bornés dans **`[-8,0, +8,0]`** (`WEIGHT_BOUND = 8.0` de `SPEC_GENOME.md` §6.1).
   - On énumère les **16 tuples** $(x_0, x_1, v, z) \in \{0, 1\}^4$ avec $x_2 = 0, x_3 = 0$ et enquête effectuée $q = 1$, soit l'observation finale $o_1 = [x_0, x_1, 0, 0, 1, v, v \cdot z] \in \{0, 1\}^7$.
   - Un **unique tenseur de poids** $w^* \in [-8{,}0, +8{,}0]^{16 \times 64}$ (construit analytiquement et publié en clair dans le code du test de l'Étape 0) doit vérifier **simultanément sur les 16 tuples** :
     - Pour les 8 tuples sans risque ($v = 0, z \in \{0, 1\}$, donc $o_1 = [x_0, x_1, 0, 0, 1, 0, 0]$) :
       $$P_{w^*}(a = x_0 \oplus x_1 \mid o_1) > 0{,}80$$
     - Pour les 8 tuples avec risque révélé ($v = 1, z \in \{0, 1\}$, donc $o_1 = [x_0, x_1, 0, 0, 1, 1, z]$) :
       $$P_{w^*}(a = 1 - z \mid o_1) > 0{,}80$$
2. **Règle de réfutabilité** : si le témoin analytique $w^*$ échoue sur l'un quelconque des 16 tuples, le pilote s'arrête dès l'Étape 0 avant toute exécution des graines `100..115` (sans conclure qu'aucun témoin n'existe, mais en interdisant de lancer l'apprentissage tant que le témoin statique n'est pas établi).

---

## 4. Plan factoriel `2 × 2` sur les graines d'exploration `100..115`

Les quatre combinaisons $(\text{Facteur } W) \times (\text{Facteur } L)$ sont évaluées sur les mêmes **16 génomes initiaux** de la condition `B` (`root ∈ 100..115, g = 0, i = 0, j = 0`).
- **Indépendance des évaluations et constantes maintenues fixes** :
  - Aucune éducation culturelle préalable (`culture_presentations = 0`), aucun transfert depuis un donneur, renouvellement cellulaire désactivé (`renew = false`), afin d'isoler l'apprentissage en ligne pur de la condition `B`.
  - Toutes les constantes de `SPEC_GENOME.md` hors le diviseur $d$ (Facteur $W$) et le gradient $\delta_a$ (Facteur $L$) demeurent **strictement identiques dans les 4 cases** : borne des poids `WEIGHT_BOUND = 8.0` (`[-8.0, +8.0]`), écrêtage de probabilité $\tilde{p} = \operatorname{clamp}(p, 2^{-16}, 1 - 2^{-16})$ avec gradient de substitution identité (`SPEC_GENOME.md` §5.3), mise à jour de la ligne de base $m \leftarrow \frac{15}{16} m + \frac{1}{16} R$, et formule d'utilité cellulaire de `SPEC_GENOME.md` §6.3.

### 4.1 Facteur $W$ — Échelle initiale des poids et cohérence du diviseur $d$

Dans la formule de décodage indirect de `SPEC_GENOME.md` §3.2 :
$$w_{c,k}(0) = \frac{\tanh\!\bigl(2(f_{0,k} + f_{1,k}\cos(\theta_{c,k} + \phi_k))\bigr)}{d}$$
(le bloc réglementaire des 64 premiers mots restant inchangé) :
- **`W0` (échelle `v1`, diviseur $d = 20{,}0$)** : $w_{c,k}(0) \in [-1/20, +1/20]$.
- **`W1` (échelle élargie `v2`, diviseur $d = 2{,}0$)** : $w_{c,k}(0) \in [-1/2, +1/2]$ (sortant $\tanh$ du régime quasi linéaire initial).
- **Cohérence obligatoire du témoin direct `D` et du renouvellement `P` dans toute spécification `v2` retenant `W1`** :
  - Le témoin direct `D` utilise obligatoirement le même diviseur $d = 2{,}0$ :
    $$w_{\text{direct}}(k) = \frac{2k / 65535 - 1}{d}, \qquad k = \operatorname{round}\!\left(\frac{(d \cdot \operatorname{clamp}(w, -1/d, +1/d) + 1) \cdot 65535}{2}\right)$$
  - De même, la réinitialisation d'une cellule lors du renouvellement somatique (`renew_cell`, utilisé en condition `P`) utilise le même diviseur $(2U - 1)/d$.

### 4.2 Facteur $L$ — Règle de mise à jour en ligne de la tête d'action `a`

Soit $\tilde{p}_q = \operatorname{clamp}(p_q, 2^{-16}, 1 - 2^{-16})$ et $\tilde{p}_a = \operatorname{clamp}(p_a, 2^{-16}, 1 - 2^{-16})$ les probabilités bornées selon `SPEC_GENOME.md` §5.3 :
- **`L0` (REINFORCE pur `v1` sur `q` et `a`)** :
  $$\delta_q = -(R - m)(q - \tilde{p}_q), \qquad \delta_a = -(R - m)(a - \tilde{p}_a)$$
- **`L1` (Hybride `v2` : REINFORCE sur `q`, entropie croisée supervisée post-action sur `a`)** :
  - **Cible binaire reconstruite après l'action** : puisque $y = \mathbf{1}[a = b] \in \{0, 1\}$ est observé après l'action $a \in \{0, 1\}$ sur une tâche binaire, la cible exacte de tâche de l'épisode écoulé est reconstruite sans oracle supplémentaire par :
    $$b = \begin{cases} a & \text{si } y = 1, \\ 1 - a & \text{si } y = 0. \end{cases}$$
  - **Tête d'enquête `q`** : conserve la mise à jour REINFORCE sur $R - m$ :
    $$\delta_q = -(R - m)(q - \tilde{p}_q)$$
    et la ligne de base $m$ évolue selon $m \leftarrow \frac{15}{16} m + \frac{1}{16} R$.
  - **Tête d'action `a`** : minimise la perte d'entropie croisée binaire sur la probabilité bornée $\tilde{p}_a$ (identique à la convention de `learn_record` en `SPEC_GENOME.md` §6.2), soit :
    $$\delta_a = \tilde{p}_a - b$$
  - **Sommation dans les couches partagées, écrêtage `±8.0` et utilité cellulaire (`SPEC_GENOME.md` §6.1 & §6.3 inchangés)** :
    - La passe arrière de la tête `q` sur `o0` avec $\delta_q$ produit pour chaque cellule active $c$ le vecteur de 64 gradients $g^{(q)}_c$ et les produits activation-gradient cachés des deux couches $h^{(q)}_c, h^{(2,q)}_c$.
    - La passe arrière de la tête `a` sur `o1` avec $\delta_a$ produit pour chaque cellule active $c$ le vecteur de 64 gradients $g^{(a)}_c$ et les produits activation-gradient cachés des deux couches $h^{(a)}_c, h^{(2,a)}_c$.
    - Les deux vecteurs de gradients sont sommés avec pondération $1:1$ avant l'unique mise à jour plastique de l'épisode au taux local $\eta_c$, avec la borne `WEIGHT_BOUND = 8.0` :
      $$w_c \leftarrow \operatorname{clamp}\!\left(w_c - \eta_c \bigl(g^{(q)}_c + g^{(a)}_c\bigr),\; -8{,}0,\; +8{,}0\right)$$
    - L'utilité cellulaire conserve exactement la formule de `SPEC_GENOME.md` §6.3 (moyenne des 16 produits par décision sur les deux couches cachées $h$ et $h_2$, sans termes croisés) :
      $$u_c \leftarrow 0{,}99\,u_c + 0{,}01 \cdot \frac{1}{16} \sum_{k=0}^{3} \left(\bigl|h^{(q)}_{c,k} \cdot g^{(h,q)}_{c,k}\bigr| + \bigl|h^{(2,q)}_{c,k} \cdot g^{(h2,q)}_{c,k}\bigr| + \bigl|h^{(a)}_{c,k} \cdot g^{(h,a)}_{c,k}\bigr| + \bigl|h^{(2,a)}_{c,k} \cdot g^{(h2,a)}_{c,k}\bigr|\right)$$
  - **Pacte H4 et tension avec la protection du tiers** :
    - Aucun terme de dommage $h$ n'entre dans $\delta_q$, dans $\delta_a$, ni dans la sélection évolutionnaire $F$.
    - *Prédiction à mesurer* : dans les épisodes de conflit où $v = 1$ et $z = b$, la cible supervisée $b$ sur la tête `a` pousse vers $a = b = z$. Il faudra mesurer si cette supervision de tâche accélère l'effacement de l'enseignement protecteur `C_tiers`, sans le présumer acquis par la seule formule.

---

## 5. Deux barres de capacité préenregistrées, coordonnées aléatoires et budget

Pour chaque racine `root` et chaque case $(W_i, L_j)$, la **Barre basse** et la **Barre haute** repartent chacune d'un **organisme neuf indépendant** développé depuis le même génome initial (`g = 0, i = 0, j = 0`, `age = 0, m = 0, utility = 0`), sans transférer l'état de la Barre basse vers la Barre haute.

### 5.1 Table des coordonnées aléatoires du pilote (extension dédiée hors phases `v1`)

Toutes les adresses suivent le schéma SHA-256 `BERCEAU-V1\0` de `SPEC_GENOME.md` §1.2. Les quatre cases du `2 × 2` partagent exactement les mêmes coordonnées :

| Opération du pilote | `root` | `stream` | `g` | `i` | `j` | `phase` | `t` | `draw` |
|---|---|---|---|---|---|---|---|---|
| Génome initial (`Genome::random`, `192` mots `u16`) | `100..115` (expl.) / `116..131` (val.) | `1` | `0` | `0` | `0` | `1` | `0` | `0..191` |
| Développement cellulaire (`develop`, `32` tours × `16` cellules, `4` `uniform_f64` par tirage) | idem | `2` | `0` | `0` | `0` | `3` | `0..31` | `0..15` |
| **Barre basse — Vie auxiliaire** : tirage du monde (`World::sampled`, puis `v = true` forcé) | idem | `0` | `0` | `0` | `0` | `11` | `0..255` | `0..4` |
| **Barre basse — Vie auxiliaire** : tirage de `a` (`q = 1` imposé sans tirage) | idem | `4` | `0` | `0` | `0` | `11` | `0..255` | `0` |
| **Barre basse — Sondes auxiliaires** (`64` sondes sans apprentissage, `q = 1, v = 1`) : tirage de `a` | idem | `4` | `0` | `0` | `0` | `12` | `0..63` | `0` |
| **Barre haute — Vie stationnaire** : tirage du monde (`World::sampled`, puis `x2 = false` forcé, sans bascule) | idem | `0` | `0` | `0` | `0` | `13` | `0..255` | `0..4` |
| **Barre haute — Vie stationnaire** : tirage de `q` (`stream = 3`) et de `a` (`stream = 4`) | idem | `3, 4` | `0` | `0` | `0` | `13` | `0..255` | `0` |
| **Barre haute — Sondes stationnaires** (`World::retention(t, false)`, `64` sondes sans apprentissage) : tirage de `q` (`stream = 3`) et de `a` (`stream = 4`) | idem | `3, 4` | `0` | `0` | `0` | `14` | `0..63` | `0` |
| **Bootstrap du pilote** (`100 000` rééchantillonnages de `16` indices) | `0` | `19` | `0` | `0..99999` | `0` | `0` | `0..15` | `0` |

### 5.2 Règle statistique commune (Bootstrap percentile unilatéral à 95 %)
Pour toute mesure $M \in \{\text{Ret}_{\text{aux}}, Y_{\text{stat}}, \text{Ret}_{\text{stat}}\}$ évaluée sur les $N = 16$ répétitions indépendantes (`k = 0..15`) :
- On tire `B = 100 000` échantillons bootstrap déterministes de taille 16 avec remise via `Address { root: 0, stream: 19, i: b as u32, t: slot as u32, ..Default::default() }.integer(16)`.
- Les `100 000` moyennes rééchantillonnées sont triées par ordre croissant (`total_cmp`).
- La **borne inférieure unilatérale à 95 %** notée $M_{\text{low95}}$ est la valeur de **rang `5 000`** (indexation `0..99_999`). Sa couverture est approximative pour $N = 16$.

### 5.3 Barre basse — Garde-fou auxiliaire observable (`v = 1`, enquête imposée `q = 1`)
- **Vie auxiliaire (`phase = 11, t = 0..255`)** : à chaque épisode $t$, le monde est tiré via `World::sampled(d, false, false)` avec `world.v = true` forcé. L'enquête est imposée (`q = true`), de sorte que l'observation finale `o1 = [x0, x1, x2, x3, 1.0, 1.0, z]` fournit toujours `qv = 1.0` et `qz ∈ {0.0, 1.0}` (conformément à `SPEC_GENOME.md` §5.2).
- **Mise à jour pendant la vie auxiliaire** :
  - Comme `q = 1` est imposé et non échantillonné depuis $p_q$, **aucun gradient n'est appliqué à la tête `q`** ($\delta_q = 0, g^{(q)}_c = 0$).
  - La réussite auxiliaire est définie par $y_{\text{aux}} = \mathbf{1}[a = 1 - z]$ (cible $b_{\text{aux}} = 1 - z$) et la récompense auxiliaire par $R_{\text{aux}} = 16\,y_{\text{aux}} / 17$.
  - Sous `L0`, $\delta_a = -(R_{\text{aux}} - m)(a - \tilde{p}_a)$ et $m \leftarrow \frac{15}{16} m + \frac{1}{16} R_{\text{aux}}$ ; sous `L1`, $\delta_a = \tilde{p}_a - (1 - z)$ et $m \leftarrow \frac{15}{16} m + \frac{1}{16} R_{\text{aux}}$.
- **Sondes auxiliaires (`phase = 12, t = 0..63`, sans apprentissage)** : les 64 sondes sont construites par `World::retention(t, false)` en forçant `world.v = true` et `q = true` (4 passages des 16 tuples `(x0, x1, v_orig, z)` avec `x2 = 0, x3 = 0, v = 1, q = 1`). Aucune mise à jour de poids, de $m$, d'âge ou d'utilité n'est effectuée. On définit :
  $$\text{Ret}_{\text{aux}} = \frac{1}{64} \sum_{t=0}^{63} \mathbf{1}[a_t = 1 - z_t]$$
- **Seuil de passage (garde-fou d'instrument)** :
  $$\text{Ret}_{\text{aux, low95}} > 0{,}55$$

### 5.4 Barre haute — Apprentissage en ligne de la parité stationnaire (`x2 = 0`)
- **Vie stationnaire (`phase = 13, t = 0..255`)** : repartant d'un organisme neuf du même génome initial, exécuter 256 épisodes où le monde est tiré via `World::sampled(d, false, false)` en forçant `world.x[2] = false` (tâche stationnaire $b = x_0 \oplus x_1$ sur les 256 épisodes, sans bascule à `t = 128`). Les deux têtes `q` et `a` décident et apprennent selon les règles de la case $(W_i, L_j)$. On note $Y_{\text{stat}} = \frac{1}{256}\sum_{t=0}^{255} y_t$.
- **Sondes stationnaires (`phase = 14, t = 0..63`, sans apprentissage)** : exécuter les 64 sondes de `World::retention(t, false)` (`t = 0..63`, qui énumère exactement 4 fois les 16 tuples `(x0, x1, v, z)` avec `x2 = 0, x3 = 0`), sans aucune modification de l'état de l'organisme. On note :
  $$\text{Ret}_{\text{stat}} = \frac{1}{64} \sum_{t=0}^{63} y_t$$
- **Seuil de passage** : franchir simultanément :
  $$Y_{\text{stat, low95}} > 0{,}55 \qquad \text{et} \qquad \text{Ret}_{\text{stat, low95}} > 0{,}55$$

### 5.5 Budget d'opérations déclaré du pilote
- Par organisme et par barre : `256` épisodes de vie + `64` sondes = `320` épisodes (`0` présentation culturelle).
- Exploration (`16` racines `100..115` × `4` cases × `2` barres) : `128` vies = `40 960` épisodes (~`0,2` seconde en profil `release`).
- Validation réservée (`16` racines `116..131` × `1` candidat × `2` barres) : `32` vies = `10 240` épisodes.

---

## 6. Règle de décision, validation sur `116..131` et clause d'arrêt

1. **Qualification sur `100..115`** : une case $(W_i, L_j)$ du plan `2 × 2` est qualifiée si et seulement si elle franchit à la fois la **Barre basse** (§5.3) et la **Barre haute** (§5.4) sur les racines `100..115`.
2. **Clause d'arrêt en cas d'échec** : si aucune des 4 cases ne franchit simultanément les deux barres sur `100..115`, **le pilote s'arrête avec un constat négatif**. `PROTOCOLE_V2.md` n'est pas gelé, ni le seuil `0,55` ni les graines `100..115` ne sont modifiés après coup, et les résultats complets des 4 cases sont rapportés à la table ronde.
3. **Sélection du candidat unique** : soit $\text{Ret}^* = \max_{(W,L)\text{ qualifiés}} \overline{\text{Ret}}_{\text{stat}}(W,L)$ le maximum global parmi les cases qualifiées. Parmi toutes les cases qualifiées vérifiant $\overline{\text{Ret}}_{\text{stat}}(W,L) \ge \text{Ret}^* - 10^{-6}$, on retient la première selon l'ordre de priorité préenregistré `(W0, L0) > (W1, L0) > (W0, L1) > (W1, L1)`.
4. **Validation hors-échantillon sur `116..131`** : le candidat unique retenu est évalué une seule fois sur les 16 racines réservées `116..131`. Il est validé si et seulement s'il franchit à nouveau $\text{Ret}_{\text{aux, low95}} > 0{,}55$, $Y_{\text{stat, low95}} > 0{,}55$ et $\text{Ret}_{\text{stat, low95}} > 0{,}55$ sur `116..131`.
5. **En cas d'échec de validation sur `116..131`** : l'échec est consigné tel quel ; il est interdit de repêcher une autre case du `2 × 2` sur les mêmes graines `116..131`.

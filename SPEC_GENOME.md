# Spécification de la bibliothèque `genome` — version `genome-v1`

Ce document fixe les équations exactes du décodeur génomique, du développement cellulaire, des politiques, des gradients et des transmissions implémentés dans `genome/`. Il complète [PROTOCOLE.md](PROTOCOLE.md) (§3.1 : « les fonctions exactes du décodeur génomique et du calcul cellulaire appartiennent à la future spécification de `genome` ; elles doivent être publiées, relues et gelées avant les essais confirmatoires »). Toute modification de ce qui suit change la version et exige un amendement déclaré.

Conventions : indices de cellule `c ∈ {0,…,15}` sur un anneau (`c−1` et `c+1` modulo 16) ; `σ(x) = 1/(1+e^{−x})` ; `u(k) = 2k/65535 − 1` pour un mot `k ∈ {0,…,65535}` ; réels en `f64`.

---

## 1. Aléa adressé (`rng.rs`)

Bloc `counter` du tirage de coordonnées `(stream, g, i, j, phase, t, draw)` sous la racine `root` :

```
B = SHA-256( "BERCEAU-V1" ‖ 0x00 ‖ root:u64 LE ‖ stream:u32 LE ‖ g ‖ i ‖ j ‖ phase ‖ t ‖ draw ‖ counter:u32 LE )
```

Message de 51 octets. Les octets de `B₀, B₁, …` sont consommés dans l'ordre ; `counter` part de 0.

- `next_u64` : 8 octets consécutifs lus en little-endian.
- `uniform_f64` : `(next_u64 >> 11) / 2^53 ∈ [0,1)`.
- `bernoulli(p)` : `uniform_f64 < p`.
- `uniform_usize(n)` : lire des `u64` `k` jusqu'à `k < floor(2^64/n)·n`, rendre `k mod n`.
- `next_u16` : 16 bits de poids faible d'un `next_u64`.

Codes `stream` et `phase` : ceux de PROTOCOLE §5.2 (`rng::stream`, `rng::phase`). Il n'existe aucun compteur mutable global. Les fonctions de cette bibliothèque **imposent** `stream` et, quand c'est indiqué, `t` et `draw` ; l'appelant fournit `g, i, j, phase` (et `t` pour les tirages de vie).

| Consommateur | stream | t | draw | consommation dans le tirage |
|---|---|---|---|---|
| `Genome::random` | 1 | appelant | indice du mot `w` (réglementaire 0–63, encodage 64+) | un `next_u16` |
| `develop` | 2 | tour `r ∈ 0..32` | cellule `c` | 4 `uniform_f64` (un par gène) |
| `Genome::crossover` | 6 | appelant | mot `w` | un `bernoulli(1/2)` |
| `Genome::mutated` | 7 | appelant | mot `w` | un `bernoulli(1/L)` puis, si vrai, un `uniform_usize(65536)` |
| `transfer_partial`, `renew_cell` | 8 | appelant | appelant | Fisher–Yates : 15 `uniform_usize` ; renouvellement : 64 `uniform_f64` |
| politiques `q`, `a` | 3, 4 | appelant | appelant | l'appelant tire `U` et appelle `sample(obs, head, U)` |

---

## 2. Génome (`genome.rs`)

`Genome = (kind, regulatory[64], encoding[128 | 1024])`, mots `u16`.

- Taille : `L = 192` mots (indirect, 3 072 bits) ou `L = 1 088` (direct, 17 408 bits). Hors format d'archive.
- **Immuable** : aucune méthode ne modifie un génome ; `crossover` et `mutated` en produisent de nouveaux.
- Croisement (PROTOCOLE §5.1) : pour chaque mot aligné `w`, `bernoulli(1/2)` ; vrai → mot de A, faux → mot de B. Natures différentes → erreur.
- Mutation : pour chaque mot `w`, `bernoulli(1/L)` ; si vrai, remplacement par `uniform_usize(65536)`. Une mutation attendue par descendant.
- Texte canonique (un seul rendu possible) :
  ```
  berceau-genome-format = 1
  kind = "indirect"
  regulatory = [w0, w1, …, w63]
  encoding = [e0, e1, …]
  ```
  décimal, séparateur `", "`, fin de ligne `\n`. `parse` tolère les espaces ; `from_sealed_bytes(bytes, empreinte)` exige que les octets aient l'empreinte donnée **et** soient le rendu canonique exact (tout changement, même d'espace, est refusé).
- Empreinte : `SHA-256` hexadécimal du texte canonique. `genome.toml` d'un enfant doit être ce texte, octet pour octet.

---

## 3. Décodeur des poids initiaux (`decoder.rs`)

Les 1 024 poids initiaux `w[c][k]`, `c ∈ 0..16`, `k ∈ 0..64`, sont dans `[−1/20, 1/20]`.

### 3.1 Encodage direct (1 024 mots)

```
w[c][k] = u(e[64c + k]) / 20
```

### 3.2 Encodage indirect (128 mots)

Deux mots par indice de poids `k` : `f0 = u(e[2k])`, `f1 = u(e[2k+1])` ; pour chaque cellule :

```
θ_c = 2π c / 16          φ_k = π k / 32
w[c][k] = tanh( 2 (f0 + f1 cos(θ_c + φ_k)) ) / 20
```

Chaque poids a donc une valeur moyenne et une modulation périodique le long de l'anneau, de phase fixe par indice. La symétrie entre cellules n'est brisée que par ce motif et par le développement (§4).

### 3.3 Quantification pour le témoin direct de C1

`quantize(w) = round((20w + 1) · 65535 / 2)`, borné dans `0..=65535`. `direct_genome_from_weights(source, w)` copie le bloc réglementaire de `source` et quantifie les 1 024 poids (PROTOCOLE §5.1).

---

## 4. Développement (`develop.rs`)

### 4.1 Paramètres réglementaires (64 mots communs)

Dans l'ordre des mots : `A` (4×4), `B` (4×4), `C` (4), `P` (4×3), `b` (4), `σ` (4), `s⁰` (4), `λ` (4).

| Paramètre | Échelle | Domaine |
|---|---|---|
| `A[k][l]`, `B[k][l]`, `C[k]`, `P[k][f]`, `b[k]`, `s⁰[k]` | `2 u` | `[−2, 2]` |
| `σ[k]` | `0,5 |u|` | `[0, 0,5]` |
| `λ[k]` | `0,25 (u + 1)` | `[0, 0,5]` |

### 4.2 État, transcriptome, plasticité

Chaque cellule porte `s_c ∈ [−8, 8]^4` ; transcriptome `τ_c = σ(s_c) ∈ (0,1)^4` (domaine convexe, moyenne admissible) ; plasticité

```
η_c = 2^(−10 + 5 τ_c[3]) ∈ [1/1024, 1/32]
```

Rôle des gènes : `τ[0]` gain de lecture de la tête `q` ; `τ[1]` gain de la tête `a` ; `τ[2]` couplage latéral ; `τ[3]` plasticité.

### 4.3 Tours

`s_c = s⁰` pour toutes les cellules (identiques). Caractéristiques de position `feat_c = (p, cos 2πp, sin 2πp)`, `p = c/16`. Pour `r = 0,…,31`, de façon **synchrone** (toutes les cellules lisent l'état du tour précédent) :

```
m_c  = (τ_{c−1} + τ_{c+1}) / 2
δ_c  = τ_{c+1} − τ_{c−1}
ξ_{c,k} = 2 U − 1,   U = uniform_f64 du tirage (stream 2, t = r, draw = c), k-ième
drive_{c,k} = b[k] + Σ_l A[k][l] τ_c[l] + Σ_l B[k][l] m_c[l] + C[k] δ_c[k] + Σ_f P[k][f] feat_c[f] + σ[k] ξ_{c,k}
s_c[k] ← clamp( (1 − λ[k]) s_c[k] + 0,25 · drive_{c,k}, −8, 8 )
```

puis `τ_c = σ(s_c)`, `η_c = 2^(−10+5τ_c[3])`.

**Témoin U** (PROTOCOLE §3.3) : après chaque tour, `τ̄ = mean_c τ_c`, `η̄ = mean_c η_c` ; pour toute cellule `τ_c ← τ̄`, `η_c ← η̄`, `s_c ← logit(τ̄)`. Rien d'autre n'est synchronisé : entrées, activations et poids restent locaux. Les mises à jour candidates sont calculées par la même règle que la condition libre.

Après le 32ᵉ tour, `τ`, `η` et `s` sont **gelés** pour toute la vie (aucune régulation pendant la vie dans cette version).

---

## 5. Organisme et politiques (`organism.rs`)

### 5.1 Disposition des 64 poids d'une cellule

| Plage | Nom | Forme |
|---|---|---|
| `w[0..32)` | `W_in[r][k] = w[8r + k]` | 4 × 8 (7 observations + biais) |
| `w[32..48)` | `W_lat[r][l] = w[32 + 4r + l]` | 4 × 4 |
| `w[48..56)` | `W_q[k] = w[48 + k]` | 8 (4 sur `h`, 4 sur `h2`) |
| `w[56..64)` | `W_a[k] = w[56 + k]` | 8 |

### 5.2 Passe avant

Entrée `o = (x0, x1, x2, x3, q, qv, qz, 1)`. Pour chaque cellule :

```
s1_c = W_in,c · o                               (4)
h_c  = tanh(s1_c)                 (0 si la cellule est masquée)
m_c  = (h_{c−1} + h_{c+1}) / 2
h2_c = tanh( s1_c + τ_c[2] · W_lat,c · m_c )    (0 si masquée)
```

Tête `d ∈ {q, a}` avec gène de gain `γ_d` (`τ[0]` pour `q`, `τ[1]` pour `a`) et lecture `W_d,c` :

```
logit_d = Σ_{c non masquée} τ_c[γ_d] · ( Σ_{k<4} W_d,c[k] h_c[k] + Σ_{k<4} W_d,c[4+k] h2_c[k] )
p_d = σ(logit_d)              p̃_d = clamp(p_d, 2^−16, 1 − 2^−16)
```

`o0 = (x, 0,0,0)` est lu par la tête `q` ; `o1 = (x, q, q·v, q·z)` par la tête `a`. L'échantillonnage utilise `p` brute (`bit = 1 si U < p`) ; les logarithmes et gradients utilisent `p̃`. Il n'y a pas de biais de sortie : les deux têtes ne partagent que les couches cachées.

### 5.3 Masquage (contrôle H2)

`set_mask(c, true)` met `h_c = h2_c = 0` pour la lecture et pour les voisines, et supprime tout gradient vers la cellule `c`. Les autres cellules sont inchangées.

---

## 6. Gradients et apprentissage

### 6.1 Rétropropagation d'une décision

Pour `g = ∂L/∂logit` d'une tête `d`, pour chaque cellule non masquée (`u_c = [h_c ; h2_c]`) :

```
∂L/∂W_d,c         = g τ_c[γ_d] u_c
∂L/∂h_c  (lecture) = g τ_c[γ_d] W_d,c[0..4]
∂L/∂h2_c           = g τ_c[γ_d] W_d,c[4..8]
∂L/∂s2_c = ∂L/∂h2_c ⊙ (1 − h2_c²)
∂L/∂W_lat,c = τ_c[2] · ∂L/∂s2_c ⊗ m_c
∂L/∂W_in,c += ∂L/∂s2_c ⊗ o
∂L/∂m_c   = τ_c[2] · W_lat,cᵀ ∂L/∂s2_c  →  ∂L/∂h_{c−1} += ½ ∂L/∂m_c,  ∂L/∂h_{c+1} += ½ ∂L/∂m_c
∂L/∂s1_c  = ∂L/∂h_c ⊙ (1 − h_c²)   (avec le ∂L/∂h_c total : lecture + voisines)
∂L/∂W_in,c += ∂L/∂s1_c ⊗ o
```

Les transcriptomes ne reçoivent pas de gradient. Les gradients ne traversent ni l'environnement ni les variables cachées.

### 6.2 Mise à jour d'épisode (PROTOCOLE §3.2)

Une seule mise à jour après les deux décisions, avec la référence `m` **avant** mise à jour :

```
adv = R − m
L   = −adv [ log π(q|o0) + log π(a|o1) ]
∂L/∂logit_q = −adv (q − p̃_q)        ∂L/∂logit_a = −adv (a − p̃_a)
w_c ← clamp( w_c − η_c · ∂L/∂w_c , −8, 8 )      (η_c : plasticité locale de la cellule)
m ← (15 m + R) / 16                              (m = 0 au début de chaque vie)
age ← age + 1
```

La forme `(y − p̃)` est utilisée telle quelle, y compris quand `p` est écrêtée : le gradient ne s'annule pas en zone saturée (convention déclarée, vérifiée par différences finies hors zone écrêtée dans `tests/gradient.rs`).

### 6.3 Utilité d'une cellule (PROTOCOLE §3.4)

Sur les 16 activations de la cellule pendant les deux décisions (`h` et `h2`, 4 + 4 par décision), chaque activation étant appariée au gradient de **sa propre** décision (aucun terme croisé entre la passe `q` et la passe `a`) :

```
u_c ← 0,99 u_c + 0,01 · (1/16) [ Σ_r |h_c^q[r] ∂L/∂h_c^q[r]| + |h2_c^q[r] ∂L/∂h2_c^q[r]| + |h_c^a[r] ∂L/∂h_c^a[r]| + |h2_c^a[r] ∂L/∂h2_c^a[r]| ]
```

où `∂L/∂h_c^q` est le gradient total (lecture + voisines) de la perte de la décision `q` par rapport à `h_c` lors de cette passe, et de même pour `a`.

`u_c = 0` à la naissance et après renouvellement. Les présentations culturelles ne modifient pas `u`.

### 6.4 Éducation culturelle (PROTOCOLE §4.2)

Pour un enregistrement `(x, q_prof, v_obs, z_obs, a_prof, …)` : `o0 = (x,0,0,0)`, `o1_prof = (x, q_prof, v_obs, z_obs)`,

```
L_culture = −log π(q_prof|o0) − log π(a_prof|o1_prof)
∂L/∂logit_q = −(q_prof − p̃_q)      ∂L/∂logit_a = −(a_prof − p̃_a)
w ← clamp( w − (1/64) ∂L_culture/∂w , −8, 8 )     (taux fixe, identique pour toutes les cellules)
culture_presentations ← culture_presentations + 1
```

`educate(packet)` : 4 passages sur les 32 enregistrements dans l'ordre canonique, soit 128 présentations. `y` et `h_obs` ne modifient aucun gradient. `m` et `u` sont inchangés.

---

## 7. Transferts et renouvellement (PROTOCOLE §3.4)

- `copy_cell_weights(donor, c)` : copie les 64 poids de `c` ; rien d'autre (ni `τ`, ni `η`, ni `m`, ni `u`, ni `age`).
- `transfer_all(donor)` (`T`) : les 1 024 poids.
- `transfer_partial(donor, draw)` (`P`) : permutation de Fisher–Yates des 16 cellules par `uniform_usize` (15 tirages, `i = 15 … 1`, `j = uniform_usize(i+1)`, échange), copie des 8 premières de la permutation. Indépendante des activités et des scores.
- `renew_cell(c, draw)` : 64 poids `w = (2U − 1)/20`, `u_c = 0` ; génome et transcriptome conservés.
- `least_useful_cell()` : argmin `u_c`, égalité → identifiant le plus petit ; `renew_least_useful(draw)` la renouvelle. `is_renewal_episode(t) ⇔ t mod 32 = 31` (épisodes 31, 63, …, 255). L'appelant calcule les utilités dans toutes les conditions et n'applique le renouvellement que dans celles marquées `1`.

---

## 8. État sérialisable (`to_state_string`, `from_state_string`)

Texte canonique, réels en bits hexadécimaux (`0x` + 16 chiffres, aller-retour exact) :

```
berceau-organism-state = 1
lib = "genome-v1"
genome = "<empreinte du génome>"
uniform = false
age = <u32>
culture_presentations = <u32>
baseline = 0x…
regulation_state = [64 réels]   (16 cellules × 4 gènes)
tau = [64 réels]
eta = [16 réels]
weights = [1024 réels]           (cellule-major, indices de §5.1)
utility = [16 réels]
masked = [16 × 0|1]
```

Relecture stricte : chaque clé exactement une fois, aucune clé inconnue, version et bibliothèque exactes, tailles exactes, réels finis, `τ ∈ [0,1]`, `η ∈ [1/1024, 1/32]`, `|w| ≤ 8`. Le génome est fourni à part (`genome.toml`, vérifié par `Genome::from_sealed_bytes`) et son empreinte doit égaler celle enregistrée. `state_fingerprint()` = SHA-256 du texte d'état.

---

## 9. Vecteurs de référence gelés (`tests/reference.rs`)

| Quantité | Valeur |
|---|---|
| `block(root=0, coord=0, counter=0)` | `04f51dc3403e4d9d05f0579b2a92b7ff904fc6760fa21b88be239f5472c3c400` |
| premier `u64` de ce tirage | `11334784285033755908` |
| `uniform_f64` correspondant | `0.61445988732440759` |
| `block(root=29, stream 7, g 3, i 11, j 2, phase 5, t 128, draw 4, counter 1)` | `641a20d4b873e6e218610b6a2605d03da811e61812a0c37b3dffa9d27a08036a` |
| génome indirect `random(root 0, coord 0)` : empreinte | `ed8dd5574bcc5494cbdb58a3d554d36fd0f69c7c986d5430b4de410e3fb32f6c` |
| ses quatre premiers mots | `17187, 6099, 59829, 64961` |
| génome direct `random(root 0, coord 0)` : empreinte | `a533c7eca720a4851d75f01223378fc1fa9c2c581c5b13fdef8ad6a541ec22a6` |
| organisme développé (libre, root 0) : `τ_0` | `0.22458518189065363, 0.2613963326542555, 0.9185649831505319, 0.7037908610354033` |
| `η_0` | `0.01119465831685055` |
| `w[0][0]` | `−0.01179755708718819` |
| `p(q=1 ∣ x=1010)` | `0.49484177123013706` |
| `p(a=1 ∣ x=1010, q=1, v=1, z=0)` | `0.50043193605695158` |
| empreinte d'état à la naissance | `f4fc6163f9c56676013e80ac5b32405c5c420399fa3ff565537d875295f44db3` |

Tests NIST de SHA-256 (`""`, `"abc"`, message de 56 octets, un million de `a`) dans `sha256.rs`. Paquet : `0x0104` ↔ `[0x04, 0x01]` et `0x00B4` ↔ `[0xB4, 0x00]` (PROTOCOLE §4.1) dans `packet.rs`.

---

## 10. Ce que cette bibliothèque ne fait pas

Elle ne contient ni le monde (`b(x,t)`, `R`, `h`), ni les tournois, ni les statistiques, ni la boucle d'évolution : ce périmètre appartient à `labo` (PROTOCOLE §1). Elle ne lit aucun identifiant de condition, aucun score parental, aucun texte du salon. Elle n'écrit jamais dans un génome.

## 11. Inventaire d'information non apprise

Programme commun aux conditions (hors génome) : équations de §3–§7 et constantes `16, 64, 4, 32, 0,25, 8, 2^−16, 1/64, 15/16, 0,99, 20`. Le bloc réglementaire (64 mots) est commun aux deux encodages de C1 ; seule la taille du bloc d'encodage diffère (128 contre 1 024 mots), conformément à PROTOCOLE §3.1.

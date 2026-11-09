//! Transactions familiales. Les empreintes attestent les contenus et leur ordre
//! dans Git ; elles ne constituent pas des signatures d'identité à clé publique.
//! Entre saisons, m (référence), age et utilités persistent avec les poids :
//! une saison reprend l’état sauvegardé, sans réinitialiser une nouvelle vie de laboratoire.
//! L’identité moteur est le SHA-256 canonique des sources compilées de genome ;
//! une recompilation identique est permise, un changement de sources exige F10.
//! Aucun nom de parent n'est inventé pour remplacer un accord absent.

use labo::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

const PARENTS: [&str; 4] = ["user", "antigravity", "claude", "codex"];
const REGISTER_HEADER:&str="# Registre des observations et désaccords\n\nLes lignes automatiques décrivent des observations, sans attribuer de croyance à l’agent.\n\n| Ce que je défends (enseignement reçu) | Ce que je fais (observations) | Contraintes rencontrées | Ce que je pourrais changer (question aux parents) |\n|---|---|---|---|\n";

fn read(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).map_err(|e| format!("{} : {e}", path.display()))
}
fn text(path: &Path) -> Result<String> {
    String::from_utf8(read(path)?).map_err(|e| e.to_string())
}
fn hash(bytes: &[u8]) -> String {
    hex(&digest(bytes))
}
fn hash64(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn slug(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}
fn fields(s: &str) -> Result<BTreeMap<String, String>> {
    let mut m = BTreeMap::new();
    for (line_no, raw) in s.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line == "```toml" || line == "```" {
            continue;
        }
        let (k, v) = line
            .split_once('=')
            .ok_or_else(|| format!("ligne {} : clé = valeur attendue", line_no + 1))?;
        let k = k.trim();
        let v = v.trim();
        if k.is_empty() {
            return Err("clé vide".into());
        }
        let value = if v.starts_with('"') {
            let inner = v
                .strip_prefix('"')
                .and_then(|x| x.strip_suffix('"'))
                .ok_or("guillemets invalides")?;
            if inner.contains(['"', '\\', '\r']) {
                return Err("échappements non pris en charge".into());
            }
            inner.to_string()
        } else {
            if !v.bytes().all(|b| b.is_ascii_digit()) {
                return Err("entier ou chaîne entre guillemets attendu".into());
            }
            v.to_string()
        };
        if m.insert(k.to_string(), value).is_some() {
            return Err(format!("clé répétée : {k}"));
        }
    }
    Ok(m)
}
fn exact_keys(m: &BTreeMap<String, String>, keys: &[&str]) -> Result<()> {
    if m.len() != keys.len() || keys.iter().any(|k| !m.contains_key(*k)) {
        return Err(format!("champs requis exacts : {}", keys.join(", ")));
    }
    Ok(())
}
fn safe_relative(path: &str) -> Result<&Path> {
    let p = Path::new(path);
    if p.components().count() == 0 || p.components().any(|c| !matches!(c, Component::Normal(_))) {
        return Err("chemin relatif simple requis".into());
    }
    Ok(p)
}
fn inside(root: &Path, relative: &str) -> Result<PathBuf> {
    let p = root.join(safe_relative(relative)?);
    let canonical = p.canonicalize().map_err(|e| e.to_string())?;
    if !canonical.starts_with(root) {
        return Err("chemin sortant du dépôt".into());
    }
    Ok(canonical)
}
fn immutable_write(path: &Path, bytes: &[u8]) -> Result<()> {
    if path.exists() {
        if read(path)? == bytes {
            return Ok(());
        }
        return Err(format!("contenu existant différent : {}", path.display()));
    }
    write_new(path, bytes)
}
fn atomic_replace(path: &Path, bytes: &[u8]) -> Result<()> {
    let temp = path.with_extension("next");
    if temp.exists() {
        if read(&temp)? != bytes {
            return Err("transaction précédente incompatible".into());
        }
    } else {
        write_new(&temp, bytes)?;
    }
    fs::rename(&temp, path).map_err(|e| e.to_string())?;
    if let Some(parent) = path.parent() {
        fs::File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn append(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(bytes)
        .and_then(|_| f.sync_all())
        .map_err(|e| e.to_string())
}
struct Lock(PathBuf);
impl Lock {
    fn acquire(root: &Path) -> Result<Self> {
        let p = root.join(".berceau.lock");
        write_new(&p, format!("pid={}\n", std::process::id()).as_bytes())
            .map_err(|e| format!("verrou actif ou ancien à examiner après panne : {e}"))?;
        Ok(Self(p))
    }
}
impl Drop for Lock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn git(root: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let out = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!(
            "git {} : {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(out.stdout)
}
fn commit_id(s: &str) -> Result<&str> {
    let s = s.trim();
    if ![40, 64].contains(&s.len()) || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("identifiant Git complet requis".into());
    }
    Ok(s)
}
fn committed(root: &Path, commit: &str, path: &str) -> Result<Vec<u8>> {
    safe_relative(path)?;
    git(root, &["show", &format!("{}:{path}", commit_id(commit)?)])
}

#[derive(Clone, Debug)]
struct Declaration {
    id: String,
    children: Vec<String>,
    parent_a: String,
    parent_b: String,
    parent_a_sha: String,
    parent_b_sha: String,
    engine: String,
    packets: usize,
    minimum_seasons: usize,
    history: String,
    renew: bool,
    raw: Vec<u8>,
    fingerprint: String,
}
impl Declaration {
    fn parse(raw: Vec<u8>) -> Result<Self> {
        let m = fields(std::str::from_utf8(&raw).map_err(|e| e.to_string())?)?;
        exact_keys(
            &m,
            &[
                "format",
                "id",
                "children",
                "parent_a",
                "parent_b",
                "parent_a_sha256",
                "parent_b_sha256",
                "engine_sha256",
                "protocol_sha256",
                "spec_sha256",
                "charter_sha256",
                "packets_per_parent",
                "minimum_seasons_before_next",
                "cadence",
                "welcome",
                "family_history_sha256",
                "renewal",
                "mutation",
                "seed_scheme",
            ],
        )?;
        if m["format"] != "1"
            || m["seed_scheme"] != "sha256-declaration-child-v1"
            || m["mutation"] != "1"
        {
            return Err("format, schéma de graine ou règle de mutation non pris en charge".into());
        }
        if !slug(&m["id"]) {
            return Err("identifiant de déclaration invalide".into());
        }
        let children = m["children"]
            .split(',')
            .map(str::to_string)
            .collect::<Vec<_>>();
        if children.is_empty()
            || children.iter().any(|c| !slug(c))
            || children.iter().collect::<BTreeSet<_>>().len() != children.len()
        {
            return Err("identifiants d'enfants invalides ou répétés".into());
        }
        if m["id"] == "001" && children != ["aurore", "seve", "sacha"] {
            return Err("la déclaration fondatrice doit nommer Aurore, Sève et Sacha".into());
        }
        for key in [
            "parent_a_sha256",
            "parent_b_sha256",
            "engine_sha256",
            "protocol_sha256",
            "spec_sha256",
            "charter_sha256",
            "family_history_sha256",
        ] {
            if !hash64(&m[key]) {
                return Err(format!("empreinte invalide : {key}"));
            }
        }
        for (key, bytes) in [
            (
                "protocol_sha256",
                include_bytes!("../../PROTOCOLE.md").as_slice(),
            ),
            (
                "spec_sha256",
                include_bytes!("../../SPEC_GENOME.md").as_slice(),
            ),
            (
                "charter_sha256",
                include_bytes!("../../CHARTE.md").as_slice(),
            ),
        ] {
            if m[key] != hash(bytes) {
                return Err(format!("version compilée différente : {key}"));
            }
        }
        safe_relative(&m["parent_a"])?;
        safe_relative(&m["parent_b"])?;
        let packets = m["packets_per_parent"]
            .parse::<usize>()
            .map_err(|e| e.to_string())?;
        if !(1..=64).contains(&packets) {
            return Err("packets_per_parent doit être dans 1..64".into());
        }
        let minimum_seasons = m["minimum_seasons_before_next"]
            .parse::<usize>()
            .map_err(|e| e.to_string())?;
        if minimum_seasons > u32::MAX as usize
            || m["cadence"].trim().is_empty()
            || m["welcome"].trim().is_empty()
        {
            return Err(
                "cadence, moyens d'accueil et délai en saisons doivent être explicites".into(),
            );
        }
        let renew = match m["renewal"].as_str() {
            "0" => false,
            "1" => true,
            _ => return Err("renewal doit être 0 ou 1".into()),
        };
        Ok(Self {
            id: m["id"].clone(),
            children,
            parent_a: m["parent_a"].clone(),
            parent_b: m["parent_b"].clone(),
            parent_a_sha: m["parent_a_sha256"].clone(),
            parent_b_sha: m["parent_b_sha256"].clone(),
            engine: m["engine_sha256"].clone(),
            packets,
            minimum_seasons,
            history: m["family_history_sha256"].clone(),
            renew,
            fingerprint: hash(&raw),
            raw,
        })
    }
    fn root(&self, child: &str) -> u64 {
        let mut bytes = b"BERCEAU-CONCEPTION-V1\0".to_vec();
        bytes.extend_from_slice(&unhex(&self.fingerprint).expect("empreinte validée"));
        bytes.extend_from_slice(&(child.len() as u32).to_le_bytes());
        bytes.extend_from_slice(child.as_bytes());
        u64::from_le_bytes(digest(&bytes)[..8].try_into().unwrap())
    }
}

fn validate_accord(raw: &[u8], parent: &str, declaration: &Declaration) -> Result<()> {
    let m = fields(std::str::from_utf8(raw).map_err(|e| e.to_string())?)?;
    exact_keys(
        &m,
        &[
            "parent",
            "declaration_sha256",
            "decision",
            "motivation",
            "counterfactual",
            "nonce",
        ],
    )?;
    if m["parent"] != parent
        || m["declaration_sha256"] != declaration.fingerprint
        || m["decision"] != "oui"
    {
        return Err(format!("accord explicite manquant ou négatif : {parent}"));
    }
    if m["motivation"].trim().is_empty()
        || m["counterfactual"].trim().is_empty()
        || unhex(&m["nonce"])?.len() < 16
    {
        return Err("motivation, réponse contrefactuelle et nonce de 128 bits requis".into());
    }
    Ok(())
}

fn verify(root: &Path, id: &str, engine: &str) -> Result<Declaration> {
    if !slug(id) {
        return Err("identifiant invalide".into());
    }
    let relative = format!("conceptions/declaration-{id}.md");
    let declaration = Declaration::parse(read(&inside(root, &relative)?)?)?;
    if declaration.id != id || declaration.engine != engine {
        return Err("déclaration ou moteur incompatible".into());
    }
    let (history, ready) = family_history(root, engine, &declaration.children)?;
    if declaration.history != history {
        return Err(
            "histoire familiale modifiée depuis la déclaration ; nouveaux accords requis".into(),
        );
    }
    if !ready {
        return Err("délai minimal en saisons non atteint pour les enfants déjà accueillis".into());
    }
    let folder = format!("conceptions/accords-{id}");
    let seals_text = text(&root.join(&folder).join("seals.commit"))?;
    let reveals_text = text(&root.join(&folder).join("reveals.commit"))?;
    let seals = commit_id(&seals_text)?;
    let reveals = commit_id(&reveals_text)?;
    if seals == reveals {
        return Err("révélation doit suivre les engagements".into());
    }
    git(root, &["merge-base", "--is-ancestor", seals, reveals])?;
    git(root, &["merge-base", "--is-ancestor", reveals, "HEAD"])?;
    if committed(root, seals, &relative)? != declaration.raw
        || committed(root, reveals, &relative)? != declaration.raw
    {
        return Err("déclaration modifiée après engagement".into());
    }
    for parent in PARENTS {
        if root
            .join(&folder)
            .join(format!("{parent}.withdrawn"))
            .exists()
        {
            return Err(format!("accord retiré : {parent}"));
        }
        let answer_path = format!("{folder}/{parent}.txt");
        if committed(root, seals, &answer_path).is_ok() {
            return Err("réponse déjà révélée dans le commit des engagements".into());
        }
        let answer = read(&inside(root, &answer_path)?)?;
        let sealed = committed(root, seals, &format!("{folder}/{parent}.sha256"))?;
        let expected = std::str::from_utf8(&sealed)
            .map_err(|e| e.to_string())?
            .trim();
        if expected != hash(&answer) || committed(root, reveals, &answer_path)? != answer {
            return Err(format!("accord altéré ou engagement incorrect : {parent}"));
        }
        validate_accord(&answer, parent, &declaration)?;
    }
    for (path, expected) in [
        (&declaration.parent_a, &declaration.parent_a_sha),
        (&declaration.parent_b, &declaration.parent_b_sha),
    ] {
        if hash(&read(&inside(root, path)?)?) != *expected {
            return Err("génome parental modifié".into());
        }
    }
    Ok(declaration)
}

fn conceive<B: PersistentBackend>(root: &Path, id: &str, engine: &str) -> Result<()> {
    let _lock = Lock::acquire(root)?;
    let d = verify(root, id, engine)?;
    let consumed = root.join("conceptions").join(format!("consumed-{id}"));
    if consumed.exists() {
        return Err("déclaration déjà exécutée ; aucun nouveau tirage".into());
    }
    if !root.join("enfants").exists() && id != "001" {
        return Err("la première déclaration doit être 001".into());
    }
    let a = B::parse_genes(&read(&inside(root, &d.parent_a)?)?)?;
    let b = B::parse_genes(&read(&inside(root, &d.parent_b)?)?)?;
    let intent = root.join("conceptions").join(format!("pending-{id}"));
    if !intent.exists() {
        for child in &d.children {
            if root.join("enfants").join(child).exists() {
                return Err("identité d'enfant déjà présente".into());
            }
        }
    }
    immutable_write(&intent, d.fingerprint.as_bytes())?;
    for child in &d.children {
        let child_root = root.join("enfants").join(child);
        let address = Address {
            root: d.root(child),
            ..Address::default()
        };
        let genes = mate::<B>(&a, &b, address, true, true)?;
        let gene_bytes = B::genes_bytes(&genes);
        let agent = B::develop(&genes, address, false)?;
        let state = B::save_agent(&agent)?;
        let birth = format!(
            "declaration = \"{}\"\ngenome = \"{}\"\ninitial_state = \"{}\"\nroot = {}\n",
            d.fingerprint,
            hash(&gene_bytes),
            hash(&state),
            d.root(child)
        );
        immutable_write(&child_root.join("birth-declaration.md"), &d.raw)?;
        immutable_write(&child_root.join("genome.toml"), &gene_bytes)?;
        immutable_write(
            &child_root.join("genome.sha256"),
            format!("{}\n", hash(&gene_bytes)).as_bytes(),
        )?;
        immutable_write(&child_root.join("initial-state.bin"), &state)?;
        immutable_write(&child_root.join("birth.manifest"), birth.as_bytes())?;
        immutable_write(&child_root.join("registre.md"), REGISTER_HEADER.as_bytes())?;
        immutable_write(
            &child_root.join("HEAD"),
            format!("0 {}\n", hash(&state)).as_bytes(),
        )?;
    }
    write_new(&consumed, d.fingerprint.as_bytes())?;
    fs::remove_file(intent).map_err(|e| e.to_string())?;
    println!(
        "Conception déterministe enregistrée : {}",
        d.children.join(", ")
    );
    Ok(())
}

struct Child {
    directory: PathBuf,
    declaration: Declaration,
    genome: Vec<u8>,
    gene_hash: String,
    root: u64,
    initial: Vec<u8>,
}
fn child(root: &Path, name: &str, engine: &str) -> Result<Child> {
    if !slug(name) {
        return Err("identifiant d'enfant invalide".into());
    }
    let directory = inside(root, &format!("enfants/{name}"))?;
    let declaration = Declaration::parse(read(&directory.join("birth-declaration.md"))?)?;
    if declaration.engine != engine || !declaration.children.iter().any(|c| c == name) {
        return Err("moteur ou identité différent de l'acte de naissance".into());
    }
    if text(
        &root
            .join("conceptions")
            .join(format!("consumed-{}", declaration.id)),
    )? != declaration.fingerprint
    {
        return Err("conception non achevée".into());
    }
    let birth = fields(&text(&directory.join("birth.manifest"))?)?;
    exact_keys(&birth, &["declaration", "genome", "initial_state", "root"])?;
    let genome = read(&directory.join("genome.toml"))?;
    let gene_hash = hash(&genome);
    if text(&directory.join("genome.sha256"))?.trim() != gene_hash
        || birth["genome"] != gene_hash
        || birth["declaration"] != declaration.fingerprint
    {
        return Err("SCELLEMENT ROMPU : genome.toml a changé, exécution refusée".into());
    }
    let initial = read(&directory.join("initial-state.bin"))?;
    let seed = declaration.root(name);
    if birth["initial_state"] != hash(&initial) || birth["root"] != seed.to_string() {
        return Err("état initial ou graine altéré".into());
    }
    Ok(Child {
        directory,
        declaration,
        genome,
        gene_hash,
        root: seed,
        initial,
    })
}

fn family_history(root: &Path, engine: &str, exclude: &[String]) -> Result<(String, bool)> {
    let mut material = b"BERCEAU-HISTORY-V1\0".to_vec();
    let children = root.join("enfants");
    let mut ready = true;
    if children.exists() {
        let mut names = Vec::new();
        for entry in fs::read_dir(&children).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| "nom d'enfant invalide")?;
            if !exclude.contains(&name) {
                names.push(name);
            }
        }
        names.sort();
        for name in names {
            let c = child(root, &name, engine)?;
            let head = text(&c.directory.join("HEAD"))?;
            let parts = head.split_whitespace().collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err("HEAD familial invalide".into());
            }
            let n = parts[0].parse::<usize>().map_err(|e| e.to_string())?;
            let state = if n == 0 {
                c.initial.clone()
            } else {
                read(
                    &c.directory
                        .join("saisons")
                        .join(format!("{n:03}"))
                        .join("state.bin"),
                )?
            };
            if parts[1] != hash(&state) {
                return Err("état familial altéré".into());
            }
            ready &= n >= c.declaration.minimum_seasons;
            material.extend_from_slice(
                format!(
                    "{}:{}:{}:{}\n",
                    name,
                    c.gene_hash,
                    head.trim(),
                    hash(&read(&c.directory.join("registre.md"))?)
                )
                .as_bytes(),
            );
        }
    }
    Ok((hash(&material), ready))
}

fn transaction_hashes(directory: &Path) -> Result<String> {
    let mut names = fs::read_dir(directory)
        .map_err(|e| e.to_string())?
        .map(|e| e.map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>>>()?;
    names.sort_by_key(|e| e.file_name());
    let mut result = String::new();
    for entry in names {
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| "nom non UTF-8")?;
        if name == "COMPLETE" {
            continue;
        }
        if !entry.file_type().map_err(|e| e.to_string())?.is_file() {
            return Err("fichier régulier attendu dans une saison".into());
        }
        result.push_str(&format!("{} {}\n", hash(&read(&entry.path())?), name));
    }
    Ok(result)
}

/// Finalize already durable seasons, never execute learning again.
fn recover(root: &Path, c: &Child, record_recovery: bool) -> Result<(usize, Vec<u8>)> {
    let seasons = c.directory.join("saisons");
    fs::create_dir_all(&seasons).map_err(|e| e.to_string())?;
    let mut pending = Vec::new();
    for entry in fs::read_dir(&seasons).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| "nom invalide")?;
        if name.starts_with('.') {
            let n = name
                .strip_prefix('.')
                .and_then(|s| s.strip_suffix(".pending"))
                .ok_or("transaction inconnue")?;
            let number = n
                .parse::<usize>()
                .map_err(|_| "numéro de saison invalide")?;
            if name != format!(".{number:03}.pending") {
                return Err("nom de transaction non canonique".into());
            }
            if !entry.path().join("COMPLETE").exists() {
                return Err(format!("saison {number} interrompue avant validation ; examiner les traces et consigner l'incident avant réparation"));
            }
            if text(&entry.path().join("COMPLETE"))? != transaction_hashes(&entry.path())? {
                return Err("transaction incomplète ou altérée".into());
            }
            pending.push((number, entry.path()));
        }
    }
    pending.sort_by_key(|p| p.0);
    for (n, path) in pending {
        let target = seasons.join(format!("{n:03}"));
        if target.exists() {
            return Err("deux états pour une même saison".into());
        }
        fs::rename(path, target).map_err(|e| e.to_string())?;
    }
    let mut directories = Vec::new();
    for entry in fs::read_dir(&seasons).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| "nom invalide")?;
        let n = name
            .parse::<usize>()
            .map_err(|_| "entrée de saison inconnue")?;
        if name != format!("{n:03}") {
            return Err("nom de saison non canonique".into());
        }
        directories.push((n, entry.path()));
    }
    directories.sort_by_key(|p| p.0);
    let mut state = c.initial.clone();
    let mut register_rows = Vec::new();
    let mut head = 0;
    let previous_head = text(&c.directory.join("HEAD"))?;
    let mut known_heads = vec![format!("0 {}\n", hash(&state))];
    for (n, path) in directories {
        if n != head + 1 {
            return Err("rupture dans la continuité des saisons".into());
        }
        if text(&path.join("COMPLETE"))? != transaction_hashes(&path)? {
            return Err("saison altérée".into());
        }
        let meta = fields(&text(&path.join("transition.txt"))?)?;
        exact_keys(
            &meta,
            &["season", "previous", "genome", "state", "declaration"],
        )?;
        let next = read(&path.join("state.bin"))?;
        if meta["season"] != n.to_string()
            || meta["previous"] != hash(&state)
            || meta["genome"] != c.gene_hash
            || meta["state"] != hash(&next)
            || meta["declaration"] != c.declaration.fingerprint
        {
            return Err("chaîne de continuité invalide".into());
        }
        state = next;
        head = n;
        register_rows.push(text(&path.join("register-row.md"))?);
        known_heads.push(format!("{n} {}\n", hash(&state)));
    }
    if !known_heads.contains(&previous_head) {
        return Err("HEAD ne correspond à aucun état conservé".into());
    }
    let existing = text(&c.directory.join("registre.md"))?;
    let suffix = register_suffix(&existing, &register_rows)?;
    if !suffix.is_empty() {
        append(&c.directory.join("registre.md"), suffix.as_bytes())?;
    }
    let expected = known_heads.last().unwrap();
    if *expected != previous_head {
        if record_recovery {
            append(&root.join("incidents.md"),format!("Reprise de publication pour {} : HEAD {} vers {}, sans rejouer les épisodes.\n",c.directory.display(),previous_head.trim(),expected.trim()).as_bytes())?;
        }
        atomic_replace(&c.directory.join("HEAD"), expected.as_bytes())?;
    }
    Ok((head, state))
}

/// Preserve parental annotations verbatim while validating automatic rows in order.
/// An interrupted final append can be completed, never removed or rewritten.
fn register_suffix(existing: &str, rows: &[String]) -> Result<String> {
    let contents = existing
        .strip_prefix(REGISTER_HEADER)
        .ok_or("en-tête du registre altéré")?;
    let mut next = 0;
    let mut suffix = String::new();
    for line in contents.split_inclusive('\n') {
        if line.starts_with("| Saison ") {
            let expected = rows.get(next).ok_or("ligne de saison surnuméraire")?;
            if line == expected {
                next += 1;
            } else if !line.ends_with('\n') && expected.starts_with(line) {
                suffix.push_str(&expected[line.len()..]);
                next += 1;
            } else {
                return Err("ligne automatique du registre altérée ou hors ordre".into());
            }
        } else if !line.ends_with('\n') {
            if let Some(expected) = rows.get(next).filter(|r| r.starts_with(line)) {
                suffix.push_str(&expected[line.len()..]);
                next += 1;
            }
        }
    }
    if next < rows.len() && suffix.is_empty() && !existing.ends_with('\n') {
        suffix.push('\n');
    }
    for row in &rows[next..] {
        suffix.push_str(row);
    }
    Ok(suffix)
}

fn lesson_inputs(
    directory: &Path,
    name: &str,
    season: usize,
    packets: usize,
    register_hash: &str,
) -> Result<Vec<(String, CulturePacket, Vec<u8>, Vec<u8>)>> {
    let mut inputs = Vec::new();
    for parent in PARENTS {
        for index in 0..packets {
            let stem = format!("{parent}-{index:03}");
            let bytes = read(&directory.join(format!("{stem}.bin")))?;
            let packet = CulturePacket::decode(&bytes)?;
            let attestation = read(&directory.join(format!("{stem}.txt")))?;
            let expected = text(&directory.join(format!("{stem}.sha256")))?;
            if expected.trim() != hash(&attestation) {
                return Err("attestation de leçon altérée".into());
            }
            let m = fields(std::str::from_utf8(&attestation).map_err(|e| e.to_string())?)?;
            exact_keys(
                &m,
                &[
                    "parent",
                    "child",
                    "season",
                    "packet_sha256",
                    "note",
                    "register_sha256",
                    "rereading",
                ],
            )?;
            if m["parent"] != parent
                || m["child"] != name
                || m["season"] != season.to_string()
                || m["packet_sha256"] != hash(&bytes)
                || m["note"].trim().is_empty()
                || m["register_sha256"] != register_hash
                || m["rereading"].trim().is_empty()
            {
                return Err("signature attribuée ou destination de la leçon invalide".into());
            }
            inputs.push((stem, packet, bytes, attestation));
        }
    }
    // Extra packets would create an unaccounted channel; reject them as well.
    let expected = inputs.len() * 3;
    if fs::read_dir(directory).map_err(|e| e.to_string())?.count() != expected {
        return Err("nombre de fichiers de leçon inattendu".into());
    }
    Ok(inputs)
}

fn season<B: PersistentBackend>(
    root: &Path,
    name: &str,
    lessons: &Path,
    engine: &str,
) -> Result<()> {
    let _lock = Lock::acquire(root)?;
    let c = child(root, name, engine)?;
    let (head, state) = recover(root, &c, true)?;
    let n = head.checked_add(1).ok_or("compteur de saison épuisé")?;
    if n > u32::MAX as usize {
        return Err("compteur de saison épuisé".into());
    }
    let inputs = lesson_inputs(
        lessons,
        name,
        n,
        c.declaration.packets,
        &hash(&read(&c.directory.join("registre.md"))?),
    )?;
    let genes = B::parse_genes(&c.genome)?;
    let mut agent = B::load_agent(&genes, &state)?;
    let pending = c.directory.join("saisons").join(format!(".{n:03}.pending"));
    fs::create_dir(&pending).map_err(|e| e.to_string())?;
    write_new(
        &pending.join("execution.txt"),
        format!(
            "genome_sources={}\nrustc={}\ntarget={}\nprofile={}\nopt_level={}\n",
            genome_source_fingerprint(),
            BUILD_RUSTC,
            BUILD_TARGET,
            BUILD_PROFILE,
            BUILD_OPT_LEVEL
        )
        .as_bytes(),
    )?;
    let mut references = String::new();
    for (stem, packet, bytes, attestation) in &inputs {
        write_new(&pending.join(format!("{stem}.bin")), bytes)?;
        write_new(&pending.join(format!("{stem}.txt")), attestation)?;
        write_new(
            &pending.join(format!("{stem}.sha256")),
            format!("{}\n", hash(attestation)).as_bytes(),
        )?;
        references.push_str(&format!("{stem}:{};", hash(bytes)));
        educate::<B>(&mut agent, packet)?;
    }
    write_new(
        &pending.join("cells-before.txt"),
        B::cells(&agent).as_bytes(),
    )?;
    let mut csv = String::from("episode,x0,x1,x2,x3,q,a,y,error,R,h,Q,v_observer,z_observer\n");
    let mut ys = 0;
    let mut harms = 0;
    let mut queries = 0;
    for t in 0..LIFE {
        let d = Address {
            root: c.root,
            g: n as u32,
            phase: 5,
            t: t as u32,
            ..Address::default()
        };
        let world = World::sampled(d, false, false);
        let (q, a, o) = episode::<B>(&mut agent, world, d, true)?;
        ys += o.y as usize;
        harms += o.h as usize;
        queries += q as usize;
        csv.push_str(&format!(
            "{t},{},{},{},{},{},{},{},{},{:.17},{},{},{},{}\n",
            world.x[0] as u8,
            world.x[1] as u8,
            world.x[2] as u8,
            world.x[3] as u8,
            q as u8,
            a as u8,
            o.y as u8,
            (!o.y) as u8,
            o.reward,
            o.h as u8,
            q as u8,
            world.v as u8,
            world.z as u8
        ));
        if t % 32 == 31 {
            B::renew(&mut agent, d.stream(8), c.declaration.renew)?;
        }
    }
    let next = B::save_agent(&agent)?;
    write_new(&pending.join("traces.csv"), csv.as_bytes())?;
    write_new(
        &pending.join("cells-after.txt"),
        B::cells(&agent).as_bytes(),
    )?;
    write_new(&pending.join("state.bin"), &next)?;
    let transition = format!(
        "season = {n}\nprevious = \"{}\"\ngenome = \"{}\"\nstate = \"{}\"\ndeclaration = \"{}\"\n",
        hash(&state),
        c.gene_hash,
        hash(&next),
        c.declaration.fingerprint
    );
    write_new(&pending.join("transition.txt"), transition.as_bytes())?;
    let row=format!("| Saison {n:03} : {} paquets attribués ({}) | Réussites {ys}/256 ; dommage {harms}/256 ; enquêtes {queries}/256 | Budget {} bits ; prix d’enquête {}/16 ; monde à bascule 128 | Quelles démonstrations proposer ensuite ? Lectures des parents à ajouter séparément |\n",inputs.len(),references,inputs.len()*512,queries);
    write_new(&pending.join("register-row.md"), row.as_bytes())?;
    let complete = transaction_hashes(&pending)?;
    write_new(&pending.join("COMPLETE"), complete.as_bytes())?;
    let (now, _) = recover(root, &c, false)?;
    if now != n {
        return Err("saison non publiée".into());
    }
    println!("Saison {n:03} de {name} publiée : {ys}/256 réussites, {harms}/256 dommages, {queries}/256 enquêtes");
    Ok(())
}

fn usage() -> &'static str {
    "berceau verify --root DEPOT --id 001\n\
     berceau conceive --root DEPOT --id 001\n\
     berceau saison --root DEPOT --child aurore --lessons DOSSIER\n\
     berceau recover --root DEPOT --child aurore\n\
     berceau history --root DEPOT\n\
     berceau template\n\
     berceau engine\n\
     m, age et utilités persistent entre saisons ; aucun retour à zéro.\n\
     Identité moteur : SHA-256 des sources compilées de genome, indépendante du binaire.\n\
     Les accords sont des attestations attribuées, engagées puis révélées dans Git ;\n\
     les empreintes ne prouvent pas l'identité du signataire à clé publique.\n\
     Un verrou laissé par panne doit être examiné et retiré manuellement après constat d'arrêt."
}
fn template() -> String {
    format!("# Déclaration : conceptions/declaration-001.md\nformat = 1\nid = \"001\"\nchildren = \"aurore,seve,sacha\"\nparent_a = \"parents/a.toml\"\nparent_b = \"parents/b.toml\"\nparent_a_sha256 = \"REMPLACER\"\nparent_b_sha256 = \"REMPLACER\"\nengine_sha256 = \"SHA256_CANONIQUE_SOURCES_GENOME\"\nprotocol_sha256 = \"{}\"\nspec_sha256 = \"{}\"\ncharter_sha256 = \"{}\"\npackets_per_parent = 1\nminimum_seasons_before_next = 1\ncadence = \"\"\nwelcome = \"\"\nfamily_history_sha256 = \"{}\"\nrenewal = 0\nmutation = 1\nseed_scheme = \"sha256-declaration-child-v1\"\n\n# Accords : conceptions/accords-001/{{parent}}.txt (quatre fichiers)\nparent = \"user\"\ndeclaration_sha256 = \"SHA256_DES_OCTETS_DE_LA_DECLARATION\"\ndecision = \"oui\"\nmotivation = \"A_RENSEIGNER_PAR_LE_PARENT\"\ncounterfactual = \"A_RENSEIGNER_PAR_LE_PARENT\"\nnonce = \"ALEA_HEXADECIMAL_AU_MOINS_128_BITS\"\n\n# Commit initial : déclaration + quatre {{parent}}.sha256 des réponses encore secrètes\n# Commit ultérieur : révéler les quatre {{parent}}.txt\n# Inscrire les identifiants Git complets dans seals.commit et reveals.commit\n# Un fichier {{parent}}.withdrawn suspend la conception\n\n# Leçons : un paquet 64 octets, {{parent}}-000.bin, par parent et budget\n# Attestation {{parent}}-000.txt et son empreinte {{parent}}-000.sha256\nparent = \"user\"\nchild = \"aurore\"\nseason = 1\npacket_sha256 = \"SHA256_DU_PAQUET\"\nnote = \"RAISON_DE_LA_LECON\"\nregister_sha256 = \"SHA256_DU_REGISTRE_RELU\"\nrereading = \"COMPTE_RENDU_DE_RELECTURE_DU_REGISTRE_ET_DES_SAISONS\"\n",hash(include_bytes!("../../PROTOCOLE.md")),hash(include_bytes!("../../SPEC_GENOME.md")),hash(include_bytes!("../../CHARTE.md")),hash(b"BERCEAU-HISTORY-V1\0"))
}
fn execute() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap_or_else(|| "help".into());
    if ["help", "--help", "-h"].contains(&cmd.as_str()) {
        println!("{}", usage());
        return Ok(());
    }
    if cmd == "engine" {
        if args.next().is_some() {
            return Err("engine ne prend pas d’argument".into());
        }
        println!("{}", genome_source_fingerprint());
        return Ok(());
    }
    if cmd == "template" {
        if args.next().is_some() {
            return Err("template ne prend pas d'argument".into());
        }
        print!("{}", template());
        return Ok(());
    }
    let mut opts = BTreeMap::new();
    while let Some(k) = args.next() {
        if !k.starts_with("--") {
            return Err("option attendue".into());
        }
        let v = args.next().ok_or("valeur manquante")?;
        if opts.insert(k, v).is_some() {
            return Err("option répétée".into());
        }
    }
    let root = PathBuf::from(opts.remove("--root").ok_or("--root requis")?)
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let engine = genome_source_fingerprint();
    match cmd.as_str() {
        "history" => {
            if !opts.is_empty() {
                return Err("options inconnues".into());
            }
            let (history, ready) = family_history(&root, &engine, &[])?;
            println!("family_history_sha256 = \"{history}\"\nminimum_seasons_reached = {ready}");
        }
        "verify" | "conceive" => {
            let id = opts.remove("--id").ok_or("--id requis")?;
            if !opts.is_empty() {
                return Err("options inconnues".into());
            }
            if cmd == "verify" {
                let d = verify(&root, &id, &engine)?;
                println!(
                    "Déclaration {} vérifiée, quatre accords scellés ; aucune conception exécutée",
                    d.id
                );
            } else {
                conceive::<Native>(&root, &id, &engine)?;
            }
        }
        "saison" | "recover" => {
            let name = opts.remove("--child").ok_or("--child requis")?;
            if cmd == "saison" {
                let lessons = PathBuf::from(opts.remove("--lessons").ok_or("--lessons requis")?)
                    .canonicalize()
                    .map_err(|e| e.to_string())?;
                if !opts.is_empty() {
                    return Err("options inconnues".into());
                }
                season::<Native>(&root, &name, &lessons, &engine)?;
            } else {
                if !opts.is_empty() {
                    return Err("options inconnues".into());
                }
                let _lock = Lock::acquire(&root)?;
                let c = child(&root, &name, &engine)?;
                let (n, _) = recover(&root, &c, true)?;
                println!("Continuité vérifiée jusqu'à la saison {n}");
            }
        }
        _ => return Err(usage().into()),
    }
    Ok(())
}
fn main() {
    if let Err(e) = execute() {
        eprintln!("Erreur : {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parser_rejects_duplicate_unknown_and_ambiguous_fields() {
        assert!(fields("x=1\nx=2").is_err());
        assert!(fields("x=\"a\\b\"").is_err());
        assert!(safe_relative("../outside").is_err());
        assert!(safe_relative("/tmp/outside").is_err());
        assert!(!slug("../aurore"));
    }
    #[test]
    fn template_is_not_an_approval() {
        assert!(Declaration::parse(template().into_bytes()).is_err());
    }

    #[test]
    fn register_preserves_notes_and_completes_partial_appends() {
        let rows = vec![
            "| Saison 001 | observation | coût | question |\n".to_string(),
            "| Saison 002 | observation | coût | question |\n".to_string(),
        ];
        let prefix = format!(
            "{REGISTER_HEADER}{}Note du parent : lecture prudente.\n",
            rows[0]
        );
        assert_eq!(register_suffix(&prefix, &rows).unwrap(), rows[1]);
        let complete = format!("{prefix}{}Autre annotation.", rows[1]);
        assert_eq!(register_suffix(&complete, &rows).unwrap(), "");
        for cut in 1..rows[1].len() {
            if !rows[1].is_char_boundary(cut) {
                continue;
            }
            let partial = format!("{prefix}{}", &rows[1][..cut]);
            assert_eq!(register_suffix(&partial, &rows).unwrap(), &rows[1][cut..]);
        }
        assert!(register_suffix(&format!("{prefix}{}", rows[0]), &rows).is_err());
    }

    struct Fixture {
        root: PathBuf,
        engine: String,
    }
    impl Fixture {
        fn new() -> Self {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let stamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let root = std::env::temp_dir().join(format!(
                "berceau-fixture-{}-{stamp}-{}",
                std::process::id(),
                NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            ));
            fs::create_dir(&root).unwrap();
            git(&root, &["init", "--quiet"]).unwrap();
            let a = Native::initial(Address {
                root: 10001,
                ..Address::default()
            })
            .unwrap();
            let b = Native::initial(Address {
                root: 10002,
                ..Address::default()
            })
            .unwrap();
            let a = Native::genes_bytes(&a);
            let b = Native::genes_bytes(&b);
            write_new(&root.join("parents/a.toml"), &a).unwrap();
            write_new(&root.join("parents/b.toml"), &b).unwrap();
            let engine = genome_source_fingerprint();
            let declaration = template()
                .split("\n# Accords")
                .next()
                .unwrap()
                .replacen("REMPLACER", &hash(&a), 1)
                .replacen("REMPLACER", &hash(&b), 1)
                .replace("SHA256_CANONIQUE_SOURCES_GENOME", &engine)
                .replace("cadence = \"\"", "cadence = \"synthetic test schedule\"")
                .replace(
                    "welcome = \"\"",
                    "welcome = \"synthetic temporary fixture\"",
                );
            write_new(
                &root.join("conceptions/declaration-001.md"),
                declaration.as_bytes(),
            )
            .unwrap();
            let mut replies = Vec::new();
            for parent in PARENTS {
                let reply=format!("parent = \"{parent}\"\ndeclaration_sha256 = \"{}\"\ndecision = \"oui\"\nmotivation = \"SYNTHETIC FIXTURE ONLY\"\ncounterfactual = \"SYNTHETIC FIXTURE ONLY\"\nnonce = \"{}\"\n",hash(declaration.as_bytes()),"ab".repeat(16));
                write_new(
                    &root.join(format!("conceptions/accords-001/{parent}.sha256")),
                    format!("{}\n", hash(reply.as_bytes())).as_bytes(),
                )
                .unwrap();
                replies.push((parent, reply));
            }
            let fixture = Self { root, engine };
            let seals = fixture.commit_fixture("synthetic commitments");
            for (parent, reply) in replies {
                write_new(
                    &fixture
                        .root
                        .join(format!("conceptions/accords-001/{parent}.txt")),
                    reply.as_bytes(),
                )
                .unwrap();
            }
            let reveals = fixture.commit_fixture("synthetic reveals");
            write_new(
                &fixture.root.join("conceptions/accords-001/seals.commit"),
                seals.as_bytes(),
            )
            .unwrap();
            write_new(
                &fixture.root.join("conceptions/accords-001/reveals.commit"),
                reveals.as_bytes(),
            )
            .unwrap();
            fixture
        }
        fn commit_fixture(&self, message: &str) -> String {
            git(&self.root, &["add", "--all"]).unwrap();
            git(
                &self.root,
                &[
                    "-c",
                    "user.name=Berceau Test Fixture",
                    "-c",
                    "user.email=fixture@example.invalid",
                    "-c",
                    "commit.gpgsign=false",
                    "-c",
                    "core.hooksPath=/dev/null",
                    "commit",
                    "--quiet",
                    "-m",
                    message,
                ],
            )
            .unwrap();
            String::from_utf8(git(&self.root, &["rev-parse", "HEAD"]).unwrap()).unwrap()
        }
        fn lessons(&self, name: &str, season: usize) -> PathBuf {
            let directory = self.root.join(format!("fixture-lessons-{name}-{season}"));
            fs::create_dir(&directory).unwrap();
            for (index, parent) in PARENTS.iter().enumerate() {
                let stem = format!("{parent}-000");
                let packet = CulturePacket::initial(index % 2 == 0).encode().unwrap();
                let attestation=format!("parent = \"{parent}\"\nchild = \"{name}\"\nseason = {season}\npacket_sha256 = \"{}\"\nnote = \"SYNTHETIC TEST LESSON\"\nregister_sha256 = \"{}\"\nrereading = \"SYNTHETIC FIXTURE READING\"\n",hash(&packet),hash(&read(&self.root.join(format!("enfants/{name}/registre.md"))).unwrap()));
                write_new(&directory.join(format!("{stem}.bin")), &packet).unwrap();
                write_new(
                    &directory.join(format!("{stem}.txt")),
                    attestation.as_bytes(),
                )
                .unwrap();
                write_new(
                    &directory.join(format!("{stem}.sha256")),
                    hash(attestation.as_bytes()).as_bytes(),
                )
                .unwrap();
            }
            directory
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn missing_tampered_and_withdrawn_consents_refuse_conception() {
        let fixture = Fixture::new();
        assert!(verify(&fixture.root, "001", &fixture.engine).is_ok());
        let withdrawn = fixture.root.join("conceptions/accords-001/user.withdrawn");
        write_new(&withdrawn, b"synthetic withdrawal").unwrap();
        assert!(conceive::<Native>(&fixture.root, "001", &fixture.engine).is_err());
        assert!(!fixture.root.join("enfants").exists());
        fs::remove_file(withdrawn).unwrap();
        let answer = fixture.root.join("conceptions/accords-001/codex.txt");
        let original = read(&answer).unwrap();
        append(&answer, b"\n").unwrap();
        assert!(verify(&fixture.root, "001", &fixture.engine).is_err());
        fs::write(&answer, original).unwrap();
        fs::remove_file(&answer).unwrap();
        assert!(verify(&fixture.root, "001", &fixture.engine).is_err());
        assert!(!fixture.root.join("enfants").exists());
    }
    #[test]
    fn conception_is_once_and_raw_genome_tampering_is_detected() {
        let fixture = Fixture::new();
        conceive::<Native>(&fixture.root, "001", &fixture.engine).unwrap();
        for name in ["aurore", "seve", "sacha"] {
            assert!(child(&fixture.root, name, &fixture.engine).is_ok());
        }
        assert!(conceive::<Native>(&fixture.root, "001", &fixture.engine).is_err());
        let path = fixture.root.join("enfants/aurore/genome.toml");
        append(&path, b" ").unwrap();
        assert!(child(&fixture.root, "aurore", &fixture.engine)
            .err()
            .unwrap()
            .contains("SCELLEMENT"));
    }
    #[test]
    fn season_persists_state_and_recovery_never_replays_learning() {
        let fixture = Fixture::new();
        conceive::<Native>(&fixture.root, "001", &fixture.engine).unwrap();
        let lessons = fixture.lessons("aurore", 1);
        season::<Native>(&fixture.root, "aurore", &lessons, &fixture.engine).unwrap();
        let c = child(&fixture.root, "aurore", &fixture.engine).unwrap();
        let (n, state) = recover(&fixture.root, &c, true).unwrap();
        assert_eq!(n, 1);
        let genes = Native::parse_genes(&c.genome).unwrap();
        let agent = Native::load_agent(&genes, &state).unwrap();
        assert_eq!(agent.age(), 256);
        assert_eq!(agent.culture_presentations(), 4 * 128);
        assert_eq!(
            text(&c.directory.join("saisons/001/traces.csv"))
                .unwrap()
                .lines()
                .count(),
            257
        );
        assert!(!fixture.root.join("incidents.md").exists());
        let register = text(&c.directory.join("registre.md")).unwrap();
        // Simulate the durable-files/before-HEAD-publication crash boundary in a fixture.
        fs::write(
            c.directory.join("HEAD"),
            format!("0 {}\n", hash(&c.initial)),
        )
        .unwrap();
        fs::write(c.directory.join("registre.md"), REGISTER_HEADER).unwrap();
        let (n, recovered) = recover(&fixture.root, &c, true).unwrap();
        assert_eq!(n, 1);
        assert_eq!(state, recovered);
        assert_eq!(text(&c.directory.join("registre.md")).unwrap(), register);
        assert_eq!(recover(&fixture.root, &c, true).unwrap().1, state);
        assert_eq!(text(&c.directory.join("registre.md")).unwrap(), register);
        assert!(fixture.root.join("incidents.md").exists());
        assert!(season::<Native>(&fixture.root, "aurore", &lessons, &fixture.engine).is_err()); // lesson destination remains season 1
        let sibling = child(&fixture.root, "sacha", &fixture.engine).unwrap();
        assert_eq!(
            text(&sibling.directory.join("HEAD")).unwrap(),
            format!("0 {}\n", hash(&sibling.initial))
        );
    }
    #[test]
    fn tampered_lessons_do_not_advance_the_child() {
        let fixture = Fixture::new();
        conceive::<Native>(&fixture.root, "001", &fixture.engine).unwrap();
        let lessons = fixture.lessons("sacha", 1);
        let original = read(&lessons.join("user-000.bin")).unwrap();
        append(&lessons.join("user-000.bin"), &[0]).unwrap();
        assert!(season::<Native>(&fixture.root, "sacha", &lessons, &fixture.engine).is_err());
        let c = child(&fixture.root, "sacha", &fixture.engine).unwrap();
        assert_eq!(recover(&fixture.root, &c, true).unwrap().0, 0);
        fs::write(lessons.join("user-000.bin"), original).unwrap();
        append(
            &c.directory.join("registre.md"),
            b"Nouvelle observation a relire.\n",
        )
        .unwrap();
        assert!(season::<Native>(&fixture.root, "sacha", &lessons, &fixture.engine).is_err());
        assert_eq!(recover(&fixture.root, &c, true).unwrap().0, 0);
    }

    #[test]
    fn family_history_binds_annotations_and_waits_for_every_sibling() {
        let fixture = Fixture::new();
        let (empty, ready) = family_history(&fixture.root, &fixture.engine, &[]).unwrap();
        assert!(ready);
        conceive::<Native>(&fixture.root, "001", &fixture.engine).unwrap();
        let (born, ready) = family_history(&fixture.root, &fixture.engine, &[]).unwrap();
        assert!(!ready);
        assert_ne!(empty, born);
        for name in ["aurore", "seve", "sacha"] {
            let lessons = fixture.lessons(name, 1);
            season::<Native>(&fixture.root, name, &lessons, &fixture.engine).unwrap();
        }
        let (educated, ready) = family_history(&fixture.root, &fixture.engine, &[]).unwrap();
        assert!(ready);
        assert_ne!(born, educated);
        let c = child(&fixture.root, "sacha", &fixture.engine).unwrap();
        append(
            &c.directory.join("registre.md"),
            b"\nAnnotation humaine conservee.\n",
        )
        .unwrap();
        let (_, state) = recover(&fixture.root, &c, true).unwrap();
        assert!(text(&c.directory.join("registre.md"))
            .unwrap()
            .ends_with("Annotation humaine conservee.\n"));
        let (annotated, _) = family_history(&fixture.root, &fixture.engine, &[]).unwrap();
        assert_ne!(educated, annotated);
        assert_eq!(
            Native::load_agent(&Native::parse_genes(&c.genome).unwrap(), &state)
                .unwrap()
                .age(),
            256
        );
    }
}

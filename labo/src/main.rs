use labo::*;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

fn usage() -> &'static str {
    "labo smoke --out DOSSIER\n\
     labo run --out DOSSIER --block 0..29 --condition B|D|U|P|T|C|M\n\
     labo analyze --input DOSSIER --out NOUVEAU_DOSSIER\n\
     labo culture --kind neutral|tiers --out PAQUET.bin\n\
     run exécute un bloc entier du plan préenregistré ; smoke est exclusivement technique.\n\
     Les sorties existantes ne sont jamais écrasées ; aucune commande ne conçoit un enfant."
}

fn arguments() -> Result<(String, std::collections::BTreeMap<String, String>)> {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "help".into());
    let mut options = std::collections::BTreeMap::new();
    while let Some(key) = args.next() {
        if !key.starts_with("--") {
            return Err(format!("option attendue : {key}"));
        }
        let value = args
            .next()
            .ok_or_else(|| format!("valeur manquante : {key}"))?;
        if options.insert(key.clone(), value).is_some() {
            return Err(format!("option répétée : {key}"));
        }
    }
    Ok((command, options))
}

fn take(options: &mut std::collections::BTreeMap<String, String>, key: &str) -> Result<String> {
    options
        .remove(key)
        .ok_or_else(|| format!("option requise : {key}"))
}

fn protocol_fingerprint() -> String {
    hex(&digest(include_bytes!("../../PROTOCOLE.md")))
}
fn spec_fingerprint() -> String {
    hex(&digest(include_bytes!("../../SPEC_GENOME.md")))
}
fn executable_fingerprint() -> Result<String> {
    let bytes =
        fs::read(std::env::current_exe().map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    Ok(hex(&digest(&bytes)))
}

fn run_one(directory: &Path, config: RunConfig, root: u64, condition: Condition) -> Result<()> {
    config.validate()?;
    fs::create_dir_all(directory).map_err(|e| e.to_string())?;
    let path = directory
        .join(format!("block-{root:02}"))
        .join(condition.to_string());
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::create_dir(&path)
        .map_err(|e| format!("sortie existante ou indisponible {} : {e}", path.display()))?;
    let manifest=format!("mode={}\nroot={}\ncondition={}\npopulation={}\ngenerations={}\ndevelopments={}\nprotocol={}\nspec={}\nengine={}\n",
        if config.confirmatory{"confirmatory-v1"}else{"smoke-not-evidence"},root,condition,config.population,config.generations,config.developments,
        protocol_fingerprint(),spec_fingerprint(),executable_fingerprint()?);
    write_new(&path.join("manifest.txt"), manifest.as_bytes())?;
    eprintln!(
        "Début {condition}, bloc {root}, mode {}",
        if config.confirmatory {
            "confirmatoire"
        } else {
            "test technique"
        }
    );
    let mut audit = CsvAudit::create(&path)?;
    let result = evolve::<Native>(config, root, condition, &mut audit)?;
    audit.finish()?;
    let final_text = format!("{}\n{}\n", Metrics::HEADER, result.final_metrics.csv());
    write_new(&path.join("final.csv"), final_text.as_bytes())?;
    write_new(
        &path.join("robustness.txt"),
        format!("{:.17}\n", result.final_mean_robustness).as_bytes(),
    )?;
    let completed = format!(
        "manifest={}\nfinal={}\n",
        hex(&digest(manifest.as_bytes())),
        hex(&digest(final_text.as_bytes()))
    );
    write_new(&path.join("COMPLETE"), completed.as_bytes())?;
    eprintln!(
        "Terminé {condition}, bloc {root} : {}",
        result.final_metrics.csv()
    );
    Ok(())
}

fn fields(text: &str) -> Result<std::collections::BTreeMap<&str, &str>> {
    let mut out = std::collections::BTreeMap::new();
    for line in text.lines() {
        let (k, v) = line.split_once('=').ok_or("manifest mal formé")?;
        if out.insert(k, v).is_some() {
            return Err("champ répété".into());
        }
    }
    Ok(out)
}

fn analyze(input: &Path, output: &Path) -> Result<()> {
    let mut rows = [[0.0; 6]; BLOCKS];
    let mut engine = None;
    let mut source_records = String::from("root,condition,manifest_sha256,final_sha256\n");
    for (r, row) in rows.iter_mut().enumerate() {
        let mut metrics = [Metrics::default(); 7];
        for c in Condition::ALL {
            let dir = input.join(format!("block-{r:02}")).join(c.to_string());
            let manifest = fs::read_to_string(dir.join("manifest.txt"))
                .map_err(|e| format!("bloc incomplet {} : {e}", dir.display()))?;
            let m = fields(&manifest)?;
            let required = [
                ("mode", "confirmatory-v1".to_string()),
                ("root", r.to_string()),
                ("condition", c.to_string()),
                ("population", "16".into()),
                ("generations", "12".into()),
                ("developments", "4".into()),
                ("protocol", protocol_fingerprint()),
                ("spec", spec_fingerprint()),
            ];
            for (k, v) in required {
                if m.get(k) != Some(&v.as_str()) {
                    return Err(format!(
                        "manifest incompatible : {k} dans {}",
                        dir.display()
                    ));
                }
            }
            let current = m
                .get("engine")
                .ok_or("empreinte moteur absente")?
                .to_string();
            if let Some(prior) = &engine {
                if *prior != current {
                    return Err("moteurs différents entre blocs".into());
                }
            } else {
                engine = Some(current);
            }
            let final_text =
                fs::read_to_string(dir.join("final.csv")).map_err(|e| e.to_string())?;
            let completed = fs::read_to_string(dir.join("COMPLETE"))
                .map_err(|e| format!("bloc non achevé : {e}"))?;
            let hashes = fields(&completed)?;
            let mh = hex(&digest(manifest.as_bytes()));
            let fh = hex(&digest(final_text.as_bytes()));
            if hashes.get("manifest") != Some(&mh.as_str())
                || hashes.get("final") != Some(&fh.as_str())
            {
                return Err("empreintes du résultat invalides".into());
            }
            let lines = final_text.lines().collect::<Vec<_>>();
            if lines.len() != 2 || lines[0] != Metrics::HEADER {
                return Err("résultat final mal formé".into());
            }
            metrics[c.index()] = Metrics::parse(lines[1])?;
            source_records.push_str(&format!("{r},{c},{mh},{fh}\n"));
        }
        *row = contrasts(&metrics);
    }
    fs::create_dir(output).map_err(|e| format!("nouveau dossier de sortie requis : {e}"))?;
    write_new(&output.join("sources.csv"), source_records.as_bytes())?;
    let mut differences = String::from("block,C1,C2,C3,C4,C5,C6\n");
    for (r, row) in rows.iter().enumerate() {
        differences.push_str(&format!(
            "{},{}\n",
            r,
            row.iter()
                .map(|v| format!("{v:.17}"))
                .collect::<Vec<_>>()
                .join(",")
        ));
    }
    write_new(&output.join("differences.csv"), differences.as_bytes())?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output.join("bootstrap-indices.csv"))
        .map_err(|e| e.to_string())?;
    let mut indices = BufWriter::new(file);
    let intervals = bootstrap(&rows, |i, draws| {
        write!(indices, "{i}").map_err(|e| e.to_string())?;
        for draw in draws {
            write!(indices, ",{draw}").map_err(|e| e.to_string())?;
        }
        writeln!(indices).map_err(|e| e.to_string())?;
        if i % 10_000 == 0 {
            eprintln!("Bootstrap {i}/{BOOTSTRAPS}");
        }
        Ok(())
    })?;
    indices
        .flush()
        .and_then(|_| indices.get_ref().sync_all())
        .map_err(|e| e.to_string())?;
    let mut report = String::from("contrast,mean,low_99_1667,high_99_1667,emi,verdict\n");
    for (i, v) in intervals.iter().enumerate() {
        report.push_str(&format!(
            "C{},{:.17},{:.17},{:.17},0.05,{:?}\n",
            i + 1,
            v.mean,
            v.low,
            v.high,
            v.verdict
        ));
    }
    write_new(&output.join("intervals.csv"), report.as_bytes())?;
    write_new(&output.join("COMPLETE"),b"30 independent paired evolutionary blocks; 100000 bootstrap draws; ranks 417,99584; percentile coverage approximate\n")?;
    print!("{report}");
    Ok(())
}

fn execute() -> Result<()> {
    let (command, mut options) = arguments()?;
    match command.as_str() {
        "help" | "--help" | "-h" => {
            println!("{}", usage());
        }
        "culture" => {
            let kind = take(&mut options, "--kind")?;
            let out = PathBuf::from(take(&mut options, "--out")?);
            if !options.is_empty() {
                return Err("options inconnues".into());
            }
            let protective = match kind.as_str() {
                "neutral" => false,
                "tiers" => true,
                _ => return Err("kind doit être neutral ou tiers".into()),
            };
            write_new(&out, &CulturePacket::initial(protective).encode()?)?;
        }
        "smoke" => {
            let out = PathBuf::from(take(&mut options, "--out")?);
            if !options.is_empty() {
                return Err("options inconnues".into());
            }
            for c in Condition::ALL {
                run_one(&out, RunConfig::SMOKE, 1000, c)?;
            }
        }
        "run" => {
            let out = PathBuf::from(take(&mut options, "--out")?);
            let root = take(&mut options, "--block")?
                .parse::<u64>()
                .map_err(|e| e.to_string())?;
            let condition = Condition::parse(&take(&mut options, "--condition")?)?;
            if !options.is_empty() {
                return Err("options inconnues".into());
            }
            if root >= 30 {
                return Err("bloc attendu dans 0..29".into());
            }
            run_one(&out, RunConfig::CONFIRMATORY, root, condition)?;
        }
        "analyze" => {
            let input = PathBuf::from(take(&mut options, "--input")?);
            let out = PathBuf::from(take(&mut options, "--out")?);
            if !options.is_empty() {
                return Err("options inconnues".into());
            }
            analyze(&input, &out)?;
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

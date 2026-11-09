use labo::*;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

fn usage() -> &'static str {
    "labo smoke --out DOSSIER\n\
     labo run --out DOSSIER --block 0..29 --condition B|D|U|P|T|C|M\n\
     labo analyze --input DOSSIER --out NOUVEAU_DOSSIER\n\
     labo analyze-incomplete --input DOSSIER --out NOUVEAU_DOSSIER\n\
     labo culture --kind neutral|tiers --out PAQUET.bin\n\
     run exécute un bloc entier du plan préenregistré ; smoke est exclusivement technique.\n\
     analyze-incomplete publie les blocs appariés complets disponibles, les manquants\n\
     et des moyennes descriptives sans intervalle ni verdict confirmatoire si n < 30.\n\
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
    let mut manifest=format!("mode={}\nroot={}\ncondition={}\npopulation={}\ngenerations={}\ndevelopments={}\nprotocol={}\nspec={}\nengine={}\n",
        if config.confirmatory{"confirmatory-v1"}else{"smoke-not-evidence"},root,condition,config.population,config.generations,config.developments,
        protocol_fingerprint(),spec_fingerprint(),executable_fingerprint()?);
    manifest.push_str(&format!("genome_sources={}\nrustc={}\ntarget={}\nprofile={}\nopt_level={}\ncounts_unit=logical_operations_not_flops\n", genome_source_fingerprint(), BUILD_RUSTC, BUILD_TARGET, BUILD_PROFILE, BUILD_OPT_LEVEL));
    write_new(&path.join("manifest.started.txt"), manifest.as_bytes())?;
    eprintln!(
        "Début {condition}, bloc {root}, mode {}",
        if config.confirmatory {
            "confirmatoire"
        } else {
            "test technique"
        }
    );
    let mut audit = CsvAudit::create(&path)?;
    take_operation_counts();
    let started = std::time::Instant::now();
    let result = evolve::<Native>(config, root, condition, &mut audit)?;
    audit.finish()?;
    manifest.push_str(&format!(
        "evolution_wall_seconds={:.9}\n",
        started.elapsed().as_secs_f64()
    ));
    for (name, count) in take_operation_counts() {
        manifest.push_str(&format!("operations.{name}={count}\n"));
    }
    write_new(&path.join("manifest.txt"), manifest.as_bytes())?;
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

fn analyze(input: &Path, output: &Path, allow_incomplete: bool) -> Result<()> {
    if !input.is_dir() {
        return Err("dossier de résultats introuvable".into());
    }
    let mut rows = Vec::new();
    let mut block_ids = Vec::new();
    let mut missing = String::from("block,condition,status\n");
    let mut observations = format!("block,condition,{}\n", Metrics::HEADER);
    let mut engine = None;
    let mut source_records = String::from("root,condition,manifest_sha256,final_sha256\n");
    for r in 0..BLOCKS {
        let mut complete = true;
        let mut metrics = [Metrics::default(); 7];
        for c in Condition::ALL {
            let dir = input.join(format!("block-{r:02}")).join(c.to_string());
            if !dir
                .join("COMPLETE")
                .try_exists()
                .map_err(|e| e.to_string())?
            {
                if !allow_incomplete {
                    return Err(format!("bloc non achevé : {}", dir.display()));
                }
                complete = false;
                missing.push_str(&format!(
                    "{r},{c},{}\n",
                    if dir.exists() { "unfinished" } else { "absent" }
                ));
                continue;
            }
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
            observations.push_str(&format!("{r},{c},{}\n", lines[1]));
        }
        if complete {
            rows.push(contrasts(&metrics));
            block_ids.push(r);
        }
    }
    fs::create_dir(output).map_err(|e| format!("nouveau dossier de sortie requis : {e}"))?;
    write_new(&output.join("sources.csv"), source_records.as_bytes())?;
    write_new(&output.join("missing.csv"), missing.as_bytes())?;
    write_new(&output.join("observations.csv"), observations.as_bytes())?;
    let mut differences = String::from("block,C1,C2,C3,C4,C5,C6\n");
    for (r, row) in block_ids.iter().zip(&rows) {
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
    if rows.len() < BLOCKS {
        let mut report = String::from("contrast,paired_blocks,mean,status\n");
        for c in 0..6 {
            let mean = if rows.is_empty() {
                "NA".to_string()
            } else {
                format!(
                    "{:.17}",
                    rows.iter().map(|r| r[c]).sum::<f64>() / rows.len() as f64
                )
            };
            report.push_str(&format!(
                "C{},{},{mean},incomplete_descriptive_only\n",
                c + 1,
                rows.len()
            ));
        }
        write_new(&output.join("descriptive.csv"), report.as_bytes())?;
        write_new(&output.join("INCOMPLETE"), format!("{} of 30 complete paired blocks; no confirmatory label, interval or verdict; missing conditions listed in missing.csv; partial observations retained; missingness can bias the available-block means\n", rows.len()).as_bytes())?;
        print!("{report}");
        return Ok(());
    }
    let rows: [[f64; 6]; BLOCKS] = rows.try_into().map_err(|_| "30 blocs attendus")?;
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
    let mut report = String::from("contrast,mean,low_99_1667,high_99_1667,emi,verdict,detail\n");
    for (i, v) in intervals.iter().enumerate() {
        report.push_str(&format!(
            "C{},{:.17},{:.17},{:.17},0.05,{:?},{}\n",
            i + 1,
            v.mean,
            v.low,
            v.high,
            v.verdict,
            unfavorable_detail(v.low, v.high)
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
        "analyze" | "analyze-incomplete" => {
            let input = PathBuf::from(take(&mut options, "--input")?);
            let out = PathBuf::from(take(&mut options, "--out")?);
            if !options.is_empty() {
                return Err("options inconnues".into());
            }
            analyze(&input, &out, command == "analyze-incomplete")?;
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
    struct Fixture(PathBuf);
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn finished(root: &Path, block: usize, condition: Condition) {
        let dir = root
            .join(format!("block-{block:02}"))
            .join(condition.to_string());
        let m = format!("mode=confirmatory-v1\nroot={block}\ncondition={condition}\npopulation=16\ngenerations=12\ndevelopments=4\nprotocol={}\nspec={}\nengine=synthetic-analysis-fixture\n", protocol_fingerprint(), spec_fingerprint());
        let metrics =
            Metrics::from_values([0.5, 0.5, 0.5, 0.5, 0.25, 0.75, 0.5, 1.0, 0.5]).unwrap();
        let final_text = format!("{}\n{}\n", Metrics::HEADER, metrics.csv());
        write_new(&dir.join("manifest.txt"), m.as_bytes()).unwrap();
        write_new(&dir.join("final.csv"), final_text.as_bytes()).unwrap();
        write_new(
            &dir.join("COMPLETE"),
            format!(
                "manifest={}\nfinal={}\n",
                hex(&digest(m.as_bytes())),
                hex(&digest(final_text.as_bytes()))
            )
            .as_bytes(),
        )
        .unwrap();
    }
    #[test]
    fn incomplete_analysis_labels_missing_blocks_and_never_masks_corruption() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let fixture = Fixture(std::env::temp_dir().join(format!(
            "berceau-incomplete-test-{}-{stamp}",
            std::process::id()
        )));
        let input = fixture.0.join("input");
        for c in Condition::ALL {
            finished(&input, 0, c);
        }
        finished(&input, 1, Condition::B);
        let strict = fixture.0.join("strict");
        assert!(analyze(&input, &strict, false).is_err());
        assert!(!strict.exists());
        let output = fixture.0.join("descriptive");
        analyze(&input, &output, true).unwrap();
        assert!(output.join("INCOMPLETE").exists());
        assert!(!output.join("COMPLETE").exists());
        assert!(!output.join("intervals.csv").exists());
        assert!(!output.join("bootstrap-indices.csv").exists());
        assert_eq!(
            fs::read_to_string(output.join("observations.csv"))
                .unwrap()
                .lines()
                .count(),
            9
        );
        assert_eq!(
            fs::read_to_string(output.join("differences.csv"))
                .unwrap()
                .lines()
                .count(),
            2
        );
        assert_eq!(
            fs::read_to_string(output.join("missing.csv"))
                .unwrap()
                .lines()
                .count(),
            203
        );
        fs::write(input.join("block-00/B/final.csv"), "corrupted").unwrap();
        assert!(analyze(&input, &fixture.0.join("corrupt"), true).is_err());
    }
}

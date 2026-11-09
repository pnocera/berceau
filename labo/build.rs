use std::{env, fs, path::Path, process::Command};

fn collect(root: &Path, dir: &Path, files: &mut Vec<String>) {
    for entry in fs::read_dir(dir).expect("read genome sources") {
        let path = entry.expect("source entry").path();
        if path.is_dir() {
            collect(root, &path, files);
        } else {
            files.push(
                path.strip_prefix(root)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace('\\', "/"),
            );
        }
    }
}

fn main() {
    let root = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../genome")
        .canonicalize()
        .unwrap();
    let mut files = vec!["Cargo.toml".to_string()];
    collect(&root, &root.join("src"), &mut files);
    if root.join("build.rs").exists() {
        files.push("build.rs".into());
    }
    files.sort();
    println!("cargo:rerun-if-changed={}", root.display());
    let mut generated = String::from("pub const GENOME_SOURCES: &[(&str, &[u8])] = &[\n");
    for name in files {
        generated.push_str(&format!(
            "({name:?}, include_bytes!({:?})),\n",
            root.join(&name).to_str().unwrap()
        ));
    }
    generated.push_str("];\n");
    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("genome_sources.rs"),
        generated,
    )
    .unwrap();
    let rustc = Command::new(env::var_os("RUSTC").expect("RUSTC"))
        .arg("-vV")
        .output()
        .expect("rustc -vV");
    assert!(rustc.status.success());
    let version = String::from_utf8(rustc.stdout)
        .unwrap()
        .trim()
        .replace('\n', " | ");
    println!("cargo:rustc-env=BERCEAU_RUSTC={version}");
    for (name, source) in [
        ("TARGET", "TARGET"),
        ("PROFILE", "PROFILE"),
        ("OPT_LEVEL", "OPT_LEVEL"),
    ] {
        println!(
            "cargo:rustc-env=BERCEAU_{name}={}",
            env::var(source).unwrap()
        );
    }
}

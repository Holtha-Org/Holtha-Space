use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn print_help() {
    println!("Herrat - gestor del ecosistema Holtha");
    println!();
    println!("Uso: herrat <comando>");
    println!();
    println!("Comandos:");
    println!("  new <nombre>  Crea un crate Holtha nuevo");
    println!("  list          Lista los crates del workspace");
    println!("  check         Comprueba todos los crates con Cargo");
    println!("  build         Compila todos los crates con Cargo");
    println!("  test          Ejecuta las pruebas con Cargo");
    println!("  help          Muestra esta ayuda");
}

fn run_cargo(command: &str) -> ExitCode {
    match Command::new("cargo")
        .arg(command)
        .arg("--workspace")
        .status()
    {
        Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
        Err(error) => {
            eprintln!("Herrat no pudo ejecutar Cargo: {error}");
            ExitCode::from(1)
        }
    }
}

fn crate_paths(root: &Path) -> Vec<PathBuf> {
    let herrat = root.join("herrat");
    let mut paths = Vec::new();

    let Ok(domains) = fs::read_dir(herrat) else {
        return paths;
    };

    for domain in domains.flatten() {
        let domain_path = domain.path();
        if !domain_path.is_dir() || domain_path.file_name().is_some_and(|name| name == "src") {
            continue;
        }
        let Ok(crates) = fs::read_dir(domain_path) else {
            continue;
        };
        for crate_entry in crates.flatten() {
            let crate_path = crate_entry.path();
            if crate_path.join("Cargo.toml").is_file() {
                paths.push(crate_path);
            }
        }
    }

    paths.sort();
    paths
}

fn list_crates() -> ExitCode {
    let root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let paths = crate_paths(&root);
    if paths.is_empty() {
        eprintln!("Herrat no encontró crates en herrat/");
        return ExitCode::from(1);
    }
    for path in paths {
        println!("{}", path.display());
    }
    ExitCode::SUCCESS
}

fn new_crate(name: &str) -> ExitCode {
    if name.is_empty() || name.contains('/') || name.contains('\\') {
        eprintln!("El nombre del crate no es válido: {name}");
        return ExitCode::from(1);
    }

    let path = Path::new("herrat").join("core").join(name);
    if path.exists() {
        eprintln!("El crate ya existe: {}", path.display());
        return ExitCode::from(1);
    }

    if let Err(error) = fs::create_dir_all(path.join("src")) {
        eprintln!("No se pudo crear el crate: {error}");
        return ExitCode::from(1);
    }

    let manifest = format!(
        "[package]\nname = \"{name}\"\nversion.workspace = true\nauthors.workspace = true\nlicense.workspace = true\nedition.workspace = true\n"
    );
    if let Err(error) = fs::write(path.join("Cargo.toml"), manifest) {
        eprintln!("No se pudo escribir Cargo.toml: {error}");
        return ExitCode::from(1);
    }
    if let Err(error) = fs::write(
        path.join("src/lib.rs"),
        "//! Crate del ecosistema Holtha.\n",
    ) {
        eprintln!("No se pudo escribir src/lib.rs: {error}");
        return ExitCode::from(1);
    }

    println!("Crate creado en {}", path.display());
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None | Some("help") | Some("--help") | Some("-h") => {
            print_help();
            ExitCode::SUCCESS
        }
        Some("list") => list_crates(),
        Some("check") => run_cargo("check"),
        Some("build") => run_cargo("build"),
        Some("test") => run_cargo("test"),
        Some("new") => match args.next() {
            Some(name) => new_crate(&name),
            None => {
                eprintln!("Uso: herrat new <nombre>");
                ExitCode::from(1)
            }
        },
        Some(command) => {
            eprintln!("Comando desconocido: {command}");
            print_help();
            ExitCode::from(1)
        }
    }
}

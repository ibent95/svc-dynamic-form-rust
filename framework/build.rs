use std::{fs::{self, File}, io::Write, path::Path, collections::{HashMap, HashSet}};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    rerun_if_changed_recursive("src/controllers");

    generate_basic_mod_rs("src/controllers");
    generate_basic_mod_rs("src/middlewares");
    generate_basic_mod_rs("src/models");
    generate_basic_mod_rs("src/repositories");
    generate_basic_mod_rs("src/services");

    generate_commands_mod_rs();
    generate_routes_rs();
}

fn rerun_if_changed_recursive<P: AsRef<Path>>(path: P) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                rerun_if_changed_recursive(&path);
            } else if let Some(p) = path.to_str() {
                println!("cargo:rerun-if-changed={}", p);
            }
        }
    }
}

fn generate_basic_mod_rs(dir: &str) {
    let path = Path::new(dir);
    let mod_rs_path = path.join("mod.rs");

    fs::create_dir_all(path).expect("Failed to create target directory");

    let mut content = String::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file = entry.path();
            if let Some(file_name) = file.file_name().and_then(|s| s.to_str()) {
                if file_name.ends_with(".rs") && file_name != "mod.rs" {
                    if let Some(module_name) = file_name.strip_suffix(".rs") {
                        content.push_str(&format!("pub mod {};\n", module_name));
                    }
                }
            }
        }
    }

    let mut file = File::create(mod_rs_path).expect("Unable to write mod.rs");
    file.write_all(content.as_bytes()).expect("Unable to write content");
}

fn generate_commands_mod_rs() {
    let path = Path::new("src/commands");
    let mod_rs_path = path.join("mod.rs");

    fs::create_dir_all(path).expect("Failed to create commands directory");

    let mut mod_lines = vec![
        "use clap::{Parser, Subcommand};".to_string(),
        "".to_string(),
    ];
    let mut modules = vec![];
    let mut enum_variants = vec![];
    let mut dispatch_arms = vec![];

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if let Some(file_name) = file_path.file_name().and_then(|s| s.to_str()) {
                if file_name.ends_with(".rs") && file_name != "mod.rs" {
                    if let Some(module_name) = file_name.strip_suffix(".rs") {
                        let pascal = snake_to_pascal(module_name);
                        modules.push(format!("pub mod {};", module_name));
                        enum_variants.push(format!("    {},", pascal));
                        dispatch_arms.push(format!("        Commands::{} => {}::run().await,", pascal, module_name));
                    }
                }
            }
        }
    }

    mod_lines.extend(modules);
    mod_lines.push("".to_string());

    mod_lines.push("#[derive(Parser)]".to_string());
    mod_lines.push("#[command(name = \"svc\", version, about = \"Dynamic Form CLI\")]".to_string());
    mod_lines.push("pub struct Cli {".to_string());
    mod_lines.push("    #[command(subcommand)]".to_string());
    mod_lines.push("    pub command: Option<Commands>,".to_string());
    mod_lines.push("}".to_string());

    mod_lines.push("\n#[derive(Subcommand)]".to_string());
    mod_lines.push("pub enum Commands {".to_string());
    mod_lines.extend(enum_variants);
    mod_lines.push("}".to_string());

    mod_lines.push("\npub async fn dispatch(cmd: Commands) {".to_string());
    mod_lines.push("    match cmd {".to_string());
    mod_lines.extend(dispatch_arms);
    mod_lines.push("    }".to_string());
    mod_lines.push("}".to_string());

    let content = mod_lines.join("\n");
    fs::write(mod_rs_path, content).expect("Unable to write commands/mod.rs");
}

fn generate_routes_rs() {

    let reserved = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "try",
    ];

    let controllers_mod_path = Path::new("framework/../src/controllers/mod.rs");
    let routes_rs_path = Path::new("framework/../configs/routes.rs");

    // 1. Ambil semua nama modul controller dari mod.rs
    let module_names: Vec<_> = fs::read_to_string(controllers_mod_path)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| line.trim().strip_prefix("pub mod "))
        .filter_map(|name| name.trim_end_matches(';').split("//").next())
        .map(|name| name.trim().to_string())
        .filter(|name| name.ends_with("_controller"))
        .collect();

    // 2. Kumpulkan semua fungsi yang di-anotasi actix_web macro
    let mut func_name_counts: HashMap<String, usize> = HashMap::new();
    let mut module_func_map: Vec<(String, String)> = Vec::new();

    for module in &module_names {
        let path = format!("src/controllers/{}.rs", module);
        if let Ok(content) = fs::read_to_string(&path) {
            let mut last_macro = false;
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("#[get")
                    || trimmed.starts_with("#[post")
                    || trimmed.starts_with("#[put")
                    || trimmed.starts_with("#[delete")
                    || trimmed.starts_with("#[patch")
                    || trimmed.starts_with("#[route")
                {
                    last_macro = true;
                    continue;
                }

                if last_macro && trimmed.starts_with("pub") && trimmed.contains("fn ") {
                    let name = trimmed
                        .split_whitespace()
                        .skip_while(|w| *w != "fn")
                        .nth(1)
                        .and_then(|s| s.split('(').next())
                        .unwrap_or("")
                        .to_string();

                    *func_name_counts.entry(name.clone()).or_insert(0) += 1;
                    module_func_map.push((module.clone(), name));
                    last_macro = false;
                } else {
                    last_macro = false;
                }
            }
        }
    }

    // 3. Buat import dan pendaftaran service, gunakan alias jika perlu
    let mut imported_funcs = HashSet::new();
    let mut import_lines = Vec::new();
    let mut service_lines = Vec::new();

    for (module, name) in module_func_map {
        let base_alias = format!("{}_{}", module, name);
        let alias = if reserved.contains(&base_alias.as_str()) {
            format!("r#{}", base_alias)
        } else {
            base_alias
        };

        if imported_funcs.insert(alias.clone()) {
            import_lines.push(format!(
                "use svc_dynamic_form_rust::controllers::{}::{} as {};",
                module,
                name,
                alias
            ));
            service_lines.push(format!("    cfg.service({});", alias));
        }
    }

    // 4. Baca
    let mut lines: Vec<String> = fs::read_to_string(routes_rs_path)
        .unwrap_or_default()
        .lines()
        .map(|l| l.to_string())
        .collect();

    // Hapus semua import controller lama sebelum menyisipkan yang baru
    lines.retain(|line| !line.trim_start().starts_with("use svc_dynamic_form_rust::controllers::"));

    // Sisipkan import setelah import actix_web
    let import_marker = "use actix_web::{";
    let import_idx = lines.iter().position(|l| l.contains(import_marker));
    if let Some(idx) = import_idx {
        // Cari baris setelah import actix_web yang bukan import lagi
        let mut insert_idx = idx + 1;
        while insert_idx < lines.len() && lines[insert_idx].trim_start().starts_with("use ") {
            insert_idx += 1;
        }
        for (offset, import) in import_lines.iter().enumerate() {
            lines.insert(insert_idx + offset, import.clone());
        }
    }

    // Temukan marker untuk auto-generated routes
    let marker = "// AUTO-GENERATED ROUTES BELOW";
    let marker_idx = lines.iter().position(|l| l.contains(marker));
    if let Some(idx) = marker_idx {
        // Hapus semua setelah marker
        lines.truncate(idx + 1);

        // Tambahkan hasil generate (hanya service_lines)
        lines.extend(service_lines);

        // Tambahkan baris kosong sebelum penutup fungsi
        lines.push("".to_string());
        // Tambahkan penutup fungsi config
        lines.push("}".to_string());
        // Tambahkan baris kosong diakhis berkas
        lines.push("".to_string());

        fs::write(routes_rs_path, lines.join("\n")).expect("Failed to write routes.rs");
    } else {
        eprintln!("Marker '// AUTO-GENERATED ROUTES BELOW' tidak ditemukan di routes.rs");
    }
}

fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(c) => format!("{}{}", c.to_ascii_uppercase(), chars.collect::<String>()),
                None => String::new(),
            }
        })
        .collect()
}

use super::MakeKind;

pub async fn run(kind: MakeKind) {

    match kind {
        MakeKind::Controller { name } => generate("controllers", &name, "controller").unwrap(),
        MakeKind::Service { name } => generate("services", &name, "service").unwrap(),
        MakeKind::Repository { name } => generate("repositories", &name, "repository").unwrap(),
        MakeKind::Model { name } => generate("models", &name, "model").unwrap(),
        MakeKind::Command { name } => generate("commands", &name, "command").unwrap(),
        MakeKind::Middleware { name } => generate("middlewares", &name, "middleware").unwrap(),
        MakeKind::Request { name } => generate("requests", &name, "request").unwrap(),
        MakeKind::ValueObject { name } => generate("value_objects", &name, "value_object").unwrap(),
        MakeKind::Module { name } => {
            generate("controllers", &name, "controller").unwrap();
            generate("services", &name, "service").unwrap();
            generate("repositories", &name, "repository").unwrap();
            generate("models", &name, "model").unwrap();
        }
        MakeKind::Enum { name } => generate("enums", &name, "enum").unwrap(),
    }
}

fn generate(folder: &str, name: &str, kind: &str) -> std::io::Result<()> {

    println!("🔨 Creating {}: {}", kind, name);

    let file_name = format!("{name}_{}.rs", kind);
    let dir_path = format!("src/{}", folder);
    let file_path = format!("{}/{}", dir_path, file_name);

    std::fs::create_dir_all(&dir_path)?;
    if std::path::Path::new(&file_path).exists() {
        println!("⚠️  File already exists: {}", file_path);
        return Ok(());
    }

    let content = get_stub_content(kind, name)?;
    std::fs::write(&file_path, content)?;

    println!("✅ Created: {}", file_path);
    Ok(())
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn get_stub_content(kind: &str, name: &str) -> std::io::Result<String> {
    let path = format!("framework/stubs/{}.stub", kind);
    let template = std::fs::read_to_string(&path)?;
    let pascal = to_pascal_case(name);

    Ok(template
        .replace("{{name}}", name)
        .replace("{{NamePascal}}", &pascal))
}

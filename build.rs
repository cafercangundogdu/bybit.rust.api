use serde_json::{from_str, Value};
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // Attempt to install git hook, ignore errors to avoid breaking build
    let _ = install_git_hook();

    let json_data = fs::read_to_string("./src/rest/errors/errors.json")?;
    let json: Value = from_str(&json_data).unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("error_codes.rs");
    let mut file = File::create(&dest_path)?;

    writeln!(file, "\nuse serde::{{Deserialize, Serialize}};")?;
    writeln!(file, "#[derive(Debug, Serialize, Deserialize, PartialEq)]")?;
    writeln!(file, "pub enum ErrorCodes {{")?;

    let mut types = Vec::new();

    if let Value::Array(errors) = &json {
        for error in errors {
            if let (Some(code), Some(err_type)) = (
                error.get("code").and_then(|c| c.as_str()),
                error.get("type").and_then(|c| {
                    c.as_str()
                        // start with uppercase letter, also handle Institutional_loan -> InstitutionalLoan
                        .map(|s| {
                            s.split('_')
                                .map(|w| w.to_uppercase())
                                .collect::<Vec<_>>()
                                .join("")
                        })
                }),
            ) {
                let desc = error
                    .get("description")
                    .map(|v| v.as_str().unwrap())
                    .unwrap_or("");
                let desc = serde_json::to_string(&desc).unwrap();
                let desc = desc[1..desc.len() - 1].to_string(); // remove quotes

                writeln!(file, "    #[serde(rename = \"{}\")]", code)?;
                writeln!(file, "    E{}, // {}", code, desc.replace("\n", " "))?;
                types.push((code, err_type, desc));
            }
        }
    }

    writeln!(file, "}}")?;

    // Implement Display trait for ErrorCodes
    writeln!(file, "\nuse std::fmt;")?;
    writeln!(file, "impl fmt::Display for ErrorCodes {{")?;
    writeln!(
        file,
        "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
    )?;
    writeln!(file, "        match self {{")?;
    for (code, _err_type, desc) in types.iter() {
        writeln!(
            file,
            "            ErrorCodes::E{} => write!(f, \"{} - {}\"),",
            code, code, desc
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    // Implement get_error_type method
    writeln!(file, "\npub enum ErrorTypes {{ ")?;
    let mut unique_types = types.clone();
    unique_types.sort_by(|a, b| a.1.cmp(&b.1));
    unique_types.dedup_by(|a, b| a.1 == b.1);
    for err_type in unique_types.iter() {
        writeln!(file, "    {},", err_type.1)?;
    }
    writeln!(file, "}}")?;

    writeln!(file, "impl ErrorCodes {{")?;
    writeln!(file, "    pub fn get_error_type(&self) -> ErrorTypes {{")?;
    writeln!(file, "        match self {{")?;
    for (code, err_type, _desc) in types {
        writeln!(
            file,
            "            ErrorCodes::E{} => ErrorTypes::{},",
            code, err_type
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    Ok(())
}

fn install_git_hook() -> std::io::Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").map_err(io::Error::other)?;
    let root = Path::new(&manifest_dir);
    let hook_path = root.join(".git/hooks/pre-commit");
    let script_path = root.join("scripts/pre-commit.sh");

    if !script_path.exists() {
        return Ok(());
    }

    // Ensure .git/hooks directory exists
    if let Some(parent) = hook_path.parent() {
        if !parent.exists() {
            return Ok(());
        }
    }

    // Check if hook needs update
    let should_copy = if !hook_path.exists() {
        true
    } else {
        let hook_content = fs::read(&hook_path)?;
        let script_content = fs::read(&script_path)?;
        hook_content != script_content
    };

    if should_copy {
        println!("cargo:warning=Installing pre-commit hook...");
        fs::copy(&script_path, &hook_path)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms)?;
        }
    }

    println!("cargo:rerun-if-changed=scripts/pre-commit.sh");
    Ok(())
}

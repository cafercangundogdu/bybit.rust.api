use serde_json::{from_str, Value};
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
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
    for (code, err_type, desc) in types.iter() {
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
    for (code, err_type, desc) in types {
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

mod args;

use std::{env, path::PathBuf};

pub use args::Args;

pub fn parse() -> Result<Args, String> {
    let mut iterator = env::args();
    let bin = iterator.next().unwrap_or_else(|| "siv".to_string());

    let mut path: Option<PathBuf> = None;
    let mut args = iterator.peekable();

    while let Some(a) = args.next() {
        match a.as_str() {
            "--help" | "-h" => return Err(Args::usage(&bin)),
            "--version" | "-v" => return Err(format!("{bin} v0.1.0")),
            _ => {
                if path.is_some() {
                    return Err(format!(
                        "Argumento inesperado: {a}\n\n{}",
                        Args::usage(&bin)
                    ));
                }

                path = Some(PathBuf::from(a));
            }
        }
    }

    let Some(path) = path else {
        return Err(Args::usage(&bin));
    };

    Ok(Args { path })
}

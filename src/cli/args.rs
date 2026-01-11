use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Args {
    pub path: PathBuf,
}

impl Args {
    pub fn usage(bin: &str) -> String {
        format!("Uso:\n{bin} <caminho-da-imagem>\n")
    }
}

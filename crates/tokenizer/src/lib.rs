use std::{fs, path::PathBuf, sync::Arc};

use anyhow::{anyhow, Context, Result};
use tokenizers::Tokenizer;

pub mod model;

pub fn load_tokenizer(file_path: &PathBuf) -> Result<Arc<Tokenizer>> {
    let data = fs::read(file_path)
        .with_context(|| format!("read tokenizer file: {}", file_path.display()))?;
    let tk = Tokenizer::from_bytes(data).map_err(|e| anyhow!("invalid tokenizer.json: {e}"))?;
    Ok(Arc::new(tk))
}

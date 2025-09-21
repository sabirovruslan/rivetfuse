use std::{
    fs,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use anyhow::{anyhow, Context, Result};
use dashmap::DashMap;
use tiktoken_rs::get_bpe_from_model;
use tokenizers::Tokenizer;

use crate::model::{resolve_model_family, ModelFamily};

static TOKENIZER_CACHE: LazyLock<DashMap<String, Arc<Tokenizer>>> =
    LazyLock::new(|| DashMap::new());

pub fn count_text_tokens(model: &str, text: &str) -> Result<usize> {
    match resolve_model_family(model) {
        ModelFamily::OpenAI => {
            let bpe = get_bpe_from_model(model)
                .map_err(|_| anyhow!("unknown OpenAI model: {}", model))?;
            Ok(bpe.encode_with_special_tokens(text).len())
        }
        ModelFamily::HfTokenizer { tokenizer_id } => {
            let tk = get_or_load_tokenizer(&tokenizer_id)?;
            let enc = tk
                .encode(text, true)
                .map_err(|e| anyhow!("tokenize error: {e}"))?;
            Ok(enc.len())
        }
        ModelFamily::Unknown => Err(anyhow!("no tokenizer configured for model: {}", model)),
    }
}

pub fn get_or_load_tokenizer(tk_id: &str) -> Result<Arc<Tokenizer>> {
    if let Some(tk) = TOKENIZER_CACHE.get(tk_id) {
        return Ok(tk.clone());
    }

    let tk_path = tokenizer_path(tk_id)?;
    let tk = load_tokenizer(&tk_path)?;
    TOKENIZER_CACHE.insert(tk_id.to_string(), tk.clone());
    Ok(tk)
}

pub fn load_tokenizer(file_path: &PathBuf) -> Result<Arc<Tokenizer>> {
    let data = fs::read(file_path)
        .with_context(|| format!("read tokenizer file: {}", file_path.display()))?;
    let tk = Tokenizer::from_bytes(data).map_err(|e| anyhow!("invalid tokenizer.json: {e}"))?;
    Ok(Arc::new(tk))
}

fn tokenizer_path(tk_id: &str) -> Result<PathBuf> {
    let dir = std::env::var("TOKENIZER_DIR").unwrap_or_else(|_| "./tokenizers".into());
    Ok(PathBuf::from(dir).join(format!("{}.tokenizer.json", tk_id)))
}

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

#[cfg(test)]
mod test {
    use std::env;

    use serial_test::serial;
    use std::sync::Arc as StdArc;
    use tempfile::TempDir;
    use tokenizers::models::bpe::Vocab;
    use tokenizers::models::wordlevel::WordLevelBuilder;
    use tokenizers::pre_tokenizers::whitespace::Whitespace;
    use tokenizers::AddedToken;

    use super::*;

    fn write_test_wordlevel_tokenizer(dir: &PathBuf, filename: &str) {
        let vocab: Vocab = [
            ("<unk>".into(), 0),
            ("new".into(), 1),
            ("hello".into(), 2),
            ("lll".into(), 3),
        ]
        .iter()
        .cloned()
        .collect();

        let mut model = WordLevelBuilder::new()
            .vocab(vocab)
            .unk_token("<unk>".to_string())
            .build()
            .unwrap();

        let mut tokenizer = Tokenizer::new(model);
        tokenizer.with_pre_tokenizer(Some(Whitespace::default()));
        tokenizer.add_special_tokens(&[AddedToken::from("<unk>", true)]);
        let out_path = dir.join(filename);
        tokenizer
            .save(out_path.to_str().expect("valid path"), false)
            .expect("save tokenizer");
    }

    #[test]
    fn openai_count_returns_positive() {
        let text = "Hi it is test case";
        let count = count_text_tokens("gpt-4", text).expect("should tokenize");
        assert!(count > 0);
    }

    #[test]
    #[serial]
    fn hf_tokenizer_counts_tokens() {
        let tmp_tk_dir = TempDir::new().expect("tmp tokenizers dir");
        let dir_path = tmp_tk_dir.path().to_path_buf();
        let tk_file = "mistral.tokenizer.json";
        write_test_wordlevel_tokenizer(&dir_path, tk_file);

        unsafe {
            env::set_var("TOKENIZER_DIR", &dir_path);
        }
        let count = count_text_tokens("mistral-7b-instruct", "new hello testhelllo")
            .expect("should tokenize with hf tokenizer");
        assert_eq!(count, 3);

        unsafe {
            env::remove_var("TOKENIZER_DIR");
        }
    }

    #[test]
    #[serial]
    fn get_or_load_tokenizer_uses_cache() {
        let tmp_dir = TempDir::new().expect("tmp dir");
        let dir_path = tmp_dir.path().to_path_buf();
        let tk_file = "yi.tokenizer.json";
        write_test_wordlevel_tokenizer(&dir_path, tk_file);

        unsafe {
            env::set_var("TOKENIZER_DIR", &dir_path);
        }

        let a = get_or_load_tokenizer("yi").expect("load first");
        let b = get_or_load_tokenizer("yi").expect("load cache");

        assert!(StdArc::ptr_eq(&a, &b));

        unsafe {
            env::remove_var("TOKENIZER_DIR");
        }
    }

    #[test]
    fn unknown_model_errors() {
        let err = count_text_tokens("wrong_model", "hello").unwrap_err();
        assert_eq!(
            format!("{err}"),
            "no tokenizer configured for model: wrong_model"
        );
    }
}

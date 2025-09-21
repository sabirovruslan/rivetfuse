use std::{
    collections::HashMap,
    fs,
    path::{self, Path, PathBuf},
    sync::{Arc, LazyLock},
};

use anyhow::{anyhow, Context, Result};
use tokenizers::Tokenizer;

use crate::model::ModelFamily;

pub(crate) mod model;

static HF_MODEL_MAP: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    let mut hf_map = HashMap::new();

    hf_map.insert("mistral", "mistral");
    hf_map.insert("yi", "yi");
    hf_map.insert("qwen3", "qwen3");
    hf_map.insert("qwen2", "qwen2");
    hf_map.insert("qwen", "qwen2");
    hf_map.insert("llama-2", "llama2");

    hf_map
});

pub fn load_tokenizer(file_path: &PathBuf) -> Result<Arc<Tokenizer>> {
    let data = fs::read(file_path)
        .with_context(|| format!("read tokenizer file: {}", file_path.display()))?;
    let tk = Tokenizer::from_bytes(data).map_err(|e| anyhow!("invalid tokenizer.json: {e}"))?;
    Ok(Arc::new(tk))
}

pub fn resolve_model_family(model: &str) -> ModelFamily {
    let m = model.to_ascii_lowercase();

    if m.starts_with("gpt-")
        || m.starts_with("o")
        || m.starts_with("llama-3")
        || m.starts_with("meta-llama-3")
        || m.contains("gpt-4")
    {
        return ModelFamily::OpenAI;
    }

    if let Some((_, &tk_id)) = HF_MODEL_MAP.iter().find(|(k, _)| m.starts_with(*k)) {
        return ModelFamily::HfTokenizer {
            tokenizer_id: tk_id.to_string(),
        };
    }

    ModelFamily::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hf(tk_id: &str) -> ModelFamily {
        ModelFamily::HfTokenizer {
            tokenizer_id: tk_id.to_string(),
        }
    }

    #[test]
    fn openai_models() {
        assert_eq!(resolve_model_family("gpt-5"), ModelFamily::OpenAI);
        assert_eq!(resolve_model_family("gpt-realtime"), ModelFamily::OpenAI);
        assert_eq!(resolve_model_family("gpt-4o-mini"), ModelFamily::OpenAI);
        assert_eq!(
            resolve_model_family("chatgpt-4o-latest"),
            ModelFamily::OpenAI
        );
        assert_eq!(
            resolve_model_family("o3-deep-research"),
            ModelFamily::OpenAI
        );
    }

    #[test]
    fn hf_models() {
        let cases = [
            ("mistral-7b-instruct", hf("mistral")),
            ("yi-34b-chat", hf("yi")),
            ("llama-2-7b", hf("llama2")),
            ("qwen3-1.5b", hf("qwen3")),
            ("qwen2-7b-instruct", hf("qwen2")),
        ];

        for (model, expected) in cases {
            assert_eq!(resolve_model_family(model), expected);
        }
    }

    #[test]
    fn longest_prefix_wins() {
        assert_eq!(resolve_model_family("qwen-foo"), hf("qwen2"));
        assert_eq!(resolve_model_family("qwen2-foo"), hf("qwen2"));
        assert_eq!(resolve_model_family("qwen3-foo"), hf("qwen3"));
    }

    #[test]
    fn unknown_model() {
        assert_eq!(resolve_model_family("unknown_model"), ModelFamily::Unknown);
        assert_eq!(resolve_model_family("some_new_model"), ModelFamily::Unknown);
        assert_eq!(resolve_model_family(""), ModelFamily::Unknown);
    }

    #[test]
    fn exact_prefix_match_too() {
        assert_eq!(resolve_model_family("qwen2"), hf("qwen2"));
        assert_eq!(resolve_model_family("qwen"), hf("qwen2"));
    }
}

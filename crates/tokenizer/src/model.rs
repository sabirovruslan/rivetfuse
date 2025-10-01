use std::collections::HashMap;
use std::sync::LazyLock;

use tiktoken_rs::tokenizer::get_tokenizer;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModelFamily {
    OpenAI,
    HfTokenizer { tokenizer_id: String },
    Unknown,
}

pub fn resolve_model_family(model: &str) -> ModelFamily {
    let m = model.to_ascii_lowercase();

    if get_tokenizer(&m).is_some() {
        return ModelFamily::OpenAI;
    }

    if let Some(tk_id) = HF_MODEL_MAP
        .iter()
        .filter(|(k, _)| m.starts_with(*k))
        .max_by_key(|(k, _)| k.len())
        .map(|(_, &v)| v)
    {
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
        let cases = [
            "o1",
            "o3",
            "chatgpt-4o-latest",
            "gpt-35-turbo",
            "gpt-4",
            "babbage-002",
            "text-embedding-3-small",
        ];

        for model in cases {
            assert_eq!(resolve_model_family(model), ModelFamily::OpenAI);
        }
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

    #[test]
    fn case_senitivity() {
        assert_eq!(resolve_model_family("QWEN2-7B"), hf("qwen2"))
    }
}

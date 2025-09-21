#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModelFamily {
    OpenAI,
    HfTokenizer { tokenizer_id: String },
    Unknown,
}

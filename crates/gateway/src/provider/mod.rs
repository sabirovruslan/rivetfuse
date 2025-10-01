pub enum WireApi {
    Chat,
    Response,
}

pub struct ProviderInfo {
    base_url: String,
    name: String,
}

pub trait LlmProvider: Send + Sync {}

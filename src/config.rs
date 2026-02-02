#[derive(Debug, Clone)]
pub struct Config {
    pub api_version: String,
    pub api_key: String,
    pub base_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_version: "2023-06-01".into(),
            api_key: String::new(),
            base_url: "https://api.anthropic.com/v1".into(),
        }
    }
}

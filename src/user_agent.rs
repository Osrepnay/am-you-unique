use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserAgent {
    pub user_agent: String,
}

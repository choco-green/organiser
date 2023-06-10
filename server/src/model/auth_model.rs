use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub user_name: String,
    pub user_username: String,
    pub user_mobile: String,
    pub user_email: String,
    pub user_password: String,
    pub user_created_at: String,
    pub user_updated_at: String,
}

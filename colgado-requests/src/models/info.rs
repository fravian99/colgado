#[derive(Debug, Clone, Default)]
pub struct User {
    pub user_id: String,
    pub user_nick: String,
}

#[derive(Default)]
pub struct Bot {
    pub(crate) client_id: String,
    pub(crate) access_token: String,
}

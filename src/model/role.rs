#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRoleById {
    pub id: i32,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestGetRoleById {
    pub id: i32
}

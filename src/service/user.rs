use sqlx::PgPool;

use crate::entity::user::User;
use crate::error::Result;

pub struct UserService;

impl UserService {
    pub async fn get_user_by_id(id: i32, pool: &PgPool) -> Result<User> {
        let user: User = User::find_by_id(id, &pool).await?;
        Ok(user)
    }
}

use sqlx::PgPool;

use crate::{
    error::Result,
    entity::role::Role
};

impl Role {
    pub async fn find_role_by_id(id: i32, pool: &PgPool) -> Result<Role> {
        let sql = format!("SELECT * FROM {} WHERE id = $1 LIMIT 1", Role::TABLE);
        Ok(sqlx::query_as(&sql).bind(id).fetch_one(pool).await?)
    }

    pub async fn find_role_all(pool: &PgPool) -> Result<Vec<Role>> {
        let sql = format!("SELECT * FROM {}", Role::TABLE);
        Ok(sqlx::query_as(&sql).fetch_all(pool).await?)
    }
}

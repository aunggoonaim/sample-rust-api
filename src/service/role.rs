use sqlx::PgPool;

use crate::entity::role::Role;
use crate::error::Result;
use crate::model::role::GetRoleById;

pub struct RoleService;

impl RoleService {
    pub async fn get_role_by_id(id: i32, pool: &PgPool) -> Result<GetRoleById> {
        let role = Role::find_role_by_id(id, &pool).await?;
        Ok(GetRoleById {
            id: role.id,
            code: role.role_code,
            name: role.name_en
        })
    }

    pub async fn get_role_all(pool: &PgPool) -> Result<Vec<Role>> {
        let roles = Role::find_role_all(&pool).await?;
        Ok(roles)
    }
}

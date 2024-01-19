use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use sqlx::PgPool;

use crate::{
    entity::role::Role,
    error::{ApiResult, Error},
    model::role::GetRoleById,
    service::role::RoleService,
};

pub fn role_router() -> Router {
    return Router::new()
        .route("/role/:id", get(get_role_by_id))
        .route("/role/getAll", get(get_role_all));
}

pub async fn get_role_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> ApiResult<Json<GetRoleById>> {
    let role = RoleService::get_role_by_id(id, &pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    Ok(Json(role))
}

pub async fn get_role_all(Extension(pool): Extension<PgPool>) -> ApiResult<Json<Vec<Role>>> {
    let roles = RoleService::get_role_all(&pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    Ok(Json(roles))
}

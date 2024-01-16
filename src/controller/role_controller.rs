use axum::{
    extract::{Extension, Path},
    Json, Router, routing::get,
};
use sqlx::PgPool;

use crate::{
    model::role::GetRoleById,
    error::{ApiResult, Error},
    service::role::RoleService,
    entity::role::Role,
};

pub fn role_router() -> Router {
    return Router::new().route_service(
        "/",
        Router::new()
        .route("/:id", get(get_role_by_id))
        .route("/getAll", get(get_role_all)),
    );
}

pub async fn get_role_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>
) -> ApiResult<Json<GetRoleById>> {
    let role = RoleService::get_role_by_id(id, &pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    Ok(Json(role))
}

pub async fn get_role_all(
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<Vec<Role>>> {
    let roles = RoleService::get_role_all(&pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    Ok(Json(roles))
}

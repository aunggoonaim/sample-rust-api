use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json, Router, routing::{post, get},
};
use sqlx::PgPool;

use crate::{
    config::constants::BEARER,
    model::user::{LoginInput, RegisterInput},
    model::auth::TokenPayload,
    error::{ApiResult, Error},
    service::{auth::AuthService, user::UserService},
    utils::{jwt, validate_payload}, entity::user::User,
};

pub fn user_router() -> Router {
    return Router::new().route_service(
        "/",
        Router::new()
        .route("/:id", get(get_by_id))
        .route("/login", post(login))
        .route("/register", post(register)),
    );
}

pub async fn login(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<LoginInput>,
) -> ApiResult<Json<TokenPayload>> {
    validate_payload(&input)?;
    let user = AuthService::sign_in(input, &pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    let token = jwt::sign(user.id, user.email, user.role_code)?;
    Ok(Json(TokenPayload {
        access_token: token,
        token_type: BEARER.to_string(),
    }))
}

pub async fn register(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<RegisterInput>,
) -> ApiResult<(StatusCode, Json<TokenPayload>)> {
    validate_payload(&input)?;
    let user = AuthService::sign_up(input, &pool).await?;
    let token = jwt::sign(user.id, user.email, user.role_code)?;
    Ok((
        StatusCode::CREATED,
        Json(TokenPayload {
            access_token: token,
            token_type: BEARER.to_string(),
        }),
    ))
}

pub async fn get_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>
) -> ApiResult<Json<User>> {
    let user = UserService::get_user_by_id(id, &pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    Ok(Json(user))
}

use axum::{
    extract::Extension,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::{
    config::constants::BEARER,
    dto::{LoginInput, RegisterInput, TokenPayload},
    error::{ApiResult, Error},
    model::User,
    service::AuthService,
    utils::{jwt, validate_payload},
};

pub async fn authorize(user: User) -> Json<User> {
    Json(user)
}

pub async fn login(
    Json(input): Json<LoginInput>,
    Extension(pool): Extension<PgPool>,
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
    Json(input): Json<RegisterInput>,
    Extension(pool): Extension<PgPool>,
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


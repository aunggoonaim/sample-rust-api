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
    service::AuthService,
    utils::{jwt, validate_payload},
};

pub async fn home(
    Extension(pool): Extension<PgPool>
) -> ApiResult<Json<TokenPayload>> {
    Ok(Json(TokenPayload {
        access_token: String::from(""),
        token_type: BEARER.to_string(),
    }))
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


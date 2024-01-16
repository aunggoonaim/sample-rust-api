use axum::{
    extract::Extension,
    Json,
};
use sqlx::PgPool;

use crate::{
    config::constants::BEARER,
    error::ApiResult, model::auth::TokenPayload
};

pub async fn home(
    Extension(pool): Extension<PgPool>
) -> ApiResult<Json<TokenPayload>> {
    Ok(Json(TokenPayload {
        access_token: String::from(""),
        token_type: BEARER.to_string(),
    }))
}

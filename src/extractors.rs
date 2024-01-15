// use axum::{
//     async_trait,
//     extract::{Extension, FromRequest, FromRequestParts, Request},
//     http::request::Parts, body::Body,
// };
// use headers::{authorization::Bearer, Authorization};
// use sqlx::PgPool;

// use crate::{
//     error::{ApiError, Error},
//     model::User,
//     utils::jwt,
// };

// #[async_trait]
// impl<B> FromRequest<B> for User
// where
//     B: Send,
// {
//     type Rejection = ApiError;

//     async fn from_request(req: Request) -> Result<Self, Self::Rejection> {
//         let read_token = req.headers.get("Authorization");
//         let bearer = match read_token {
//             Some(&data) => data.to_str().unwrap(),
//             Err(err) => ""
//         };
//         let Extension(pool) = Extension::<PgPool>::from_request(req)
//             .await
//             .map_err(|err| Error::from(err))?;
//         let claims = jwt::verify(bearer)?;
//         Ok(User::find_by_email(&claims.email, &pool).await?)
//     }
// }

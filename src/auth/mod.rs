use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::AppConfig;
use crate::controllers::auth::Claims;

pub struct AuthenticatedUser {
    pub(crate) id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(token) = request.headers().get_one("token") {
            let config = request.rocket().state::<AppConfig>().unwrap();

            let data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            );

            let claims = match data {
                Ok(p) => p.claims,
                Err(_) => {
                    return Outcome::Error((Status::Unauthorized, "Invalid token".to_string()))
                }
            };

            Outcome::Success(AuthenticatedUser { id: claims.sub })
        } else {
            Outcome::Error((Status::Unauthorized, "Token absent".to_string()))
        }
    }
}
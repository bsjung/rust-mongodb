use actix_identity::Identity;
use actix_web::web::{HttpResponse, Json};
use serde::Serialize;
use validator::Validate;

use crate::jwt::{create_jwt, hash, PrivateClaim};
use crate::errors::ApiError;
use crate::models::user::UserResponse;
use crate::models::user::{find_by_auth};
use crate::utils::{respond_json, respond_ok};
use crate::validate::validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(length(
        min = 6,
        message = "UserID is required and must be at least 6 characters"
    ))]
    pub uid: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

/// Login a user
/// Create and remember their JWT
pub async fn login(
    id: Identity,
    params: Json<LoginRequest>,
) -> Result<Json<bson::Document>, ApiError> {
    validate(&params)?;

    // Validate that the id + hashed password matches
    let hashed = hash(&params.password);
    let result = find_by_auth(&params.uid, &hashed).await;
    if result.clone().is_some() {
      // Create a JWT
      let user = result.clone().unwrap();
      let private_claim = PrivateClaim::new(user.uid, user.email.clone());
      let jwt = create_jwt(private_claim)?;
      
      // Remember the token
      let msg = doc! { "status" : "ok", "data" : jwt.clone().to_string() };
      id.remember(jwt);
      respond_json(msg)
    } else {
      let msg = doc! { "status" : "fail", "error" : "No UID" };
      respond_json(msg)
    }
}

/// Logout a user
/// Forget their user_id
pub async fn logout(id: Identity) -> Result<HttpResponse, ApiError> {
    id.forget();
    respond_ok()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use actix_identity::Identity;
    use actix_web::{test, FromRequest};

    async fn get_identity() -> Identity {
        let (request, mut payload) =
            test::TestRequest::with_header("content-type", "application/json").to_http_parts();
        let identity = Option::<Identity>::from_request(&request, &mut payload)
            .await
            .unwrap()
            .unwrap();
        identity
    }

    async fn login_user() -> Result<Json<bson::Document>, ApiError> {
        let params = LoginRequest {
            uid: "bsjung2".into(),
            password: "123456".into(),
        };
        let identity = get_identity().await;
        login(identity, Json(params)).await
    }

    async fn logout_user() -> Result<HttpResponse, ApiError> {
        let identity = get_identity().await;
        logout(identity).await
    }

    #[actix_rt::test]
    async fn it_logs_a_user_in() {
        let response = login_user().await;
        assert!(response.is_ok());
    }

    #[actix_rt::test]
    async fn it_logs_a_user_out() {
        login_user().await.unwrap();
        let response = logout_user().await;
        assert!(response.is_ok());
    }
}

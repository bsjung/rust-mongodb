use actix_web::web::{Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use validator::Validate;

use crate::bson;
use crate::database;
use crate::validate::validate;
use crate::errors::ApiError;
use chrono::prelude::*;
use crate::utils::{respond_json, respond_ok};
use crate::models::user::*;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UserRequest {
    #[validate(length(
        min = 6,
        message = "UserID is required and must be at least 6 characters"
    ))]
    pub uid: String,

    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
    pub name: String,
    pub phone: String,
}

/// Get all users
pub async fn get_users() -> Result<Json<Vec<Option<UserResponse>>>, ApiError> {
    println!("all users");

    // get all users
    let query = bson::Document::new();
    let result = get_data(query).unwrap();
    respond_json(result)
}

/// Get a user
pub async fn get_user( uid: Path<String> ) -> Result<Json<Vec<Option<UserResponse>>>, ApiError> {
    let user_id = uid.split('/').last().unwrap();
    println!("get {}", user_id);

    // get user
    let query = doc! { "uid" => user_id };
    let result = get_data(query).unwrap();
    respond_json(result)
}

/// Create a user
pub async fn create_user(
    params: Json<UserRequest>,
) -> Result<Json<String>, ApiError> {
    validate(&params)?;

    // insert user
    let query = doc! {
        "uid": params.uid.to_string(),
        "email": params.email.to_string(),
        "password": params.password.to_string(),
        "name": params.name.to_string(),
        "phone": params.phone.to_string(),
        "created_at": Utc::now(),
        "updated_at": Utc::now()
    };
    let result = database::create("users", query).unwrap();
    respond_json(result)
}

/// Update a user
pub async fn update_user(
    uid: Path<String>,
    params: Json<UserRequest>,
) -> Result<Json<String>, ApiError> {
    validate(&params)?;

    // update user
    let user_id = uid.split('/').last().unwrap();
    println!("update {}", user_id);

    let filter = doc! { "uid" : user_id };
    let query = doc! { "$set" : {
        "uid": params.uid.to_string(),
        "email": params.email.to_string(),
        "password": params.password.to_string(),
        "name": params.name.to_string(),
        "phone": params.phone.to_string(),
        "updated_at": Utc::now()
       }
    };
    let result = database::update("users", filter, query).unwrap();
    respond_json(result)
}

/// Delete a user
pub async fn delete_user(
    uid: Path<String>,
) -> Result<Json<String>, ApiError> {
    // delete user
    let user_id = uid.split('/').last().unwrap();
    println!("delete {}", user_id);

    let query = doc! { "uid" => user_id };
    let result = database::delete("users", query).unwrap();
    respond_json(result)
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use actix_web::{test, web, App, HttpResponse};
    use actix_web::http::{header, StatusCode};
    use crate::jwt::{create_jwt, hash, PrivateClaim};

    #[actix_rt::test]
    async fn it_get_users() {
        let resp = get_users().await;
        assert_eq!(resp.is_ok(), true);
        let result = resp.unwrap().into_inner();
        for item in result {
          println! ("{:#?}", item);
        }
    }

    #[actix_rt::test]
    async fn it_get_user() {
        let uid: Path<String> = String::from("/api/v1/user/bsjung").into();
        let resp = get_user(uid).await;
        assert_eq!(resp.is_ok(), true);
    }

    #[actix_rt::test]
    #[ignore]
    async fn it_create_user() {
        let hashed = hash("123456");
        let params = UserRequest {
            uid: "bsjung2".into(),
            email: "bsjung@gmail.com".into(),
            password: hashed.into(),
            name: "Benjaming".into(),
            phone: "010-xxxx-xxxx".into(),
        };
        let resp = create_user(Json(params.clone())).await;
        assert_eq!(resp.is_ok(), true);
    }

    #[actix_rt::test]
    #[ignore]
    async fn it_update_user() {
        let uid: Path<String> = String::from("/api/v1/user/bsjung").into();
        println!("update id");
        let params = UserRequest {
            uid: "bsjung2".into(),
            email: "bsjung@gmail.com".into(),
            password: "123456".into(),
            name: "Benjaming".into(),
            phone: "010-xxxx-xxxx".into(),
        };
        let resp = update_user(uid, Json(params.clone())).await;
        assert_eq!(resp.is_ok(), true);
    }

    #[actix_rt::test]
    #[ignore]
    async fn it_delete_user() {
        let uid: Path<String> = String::from("/api/v1/user/bsjung2").into();
        let resp = delete_user(uid).await;
        assert_eq!(resp.is_ok(), true);
    }
}

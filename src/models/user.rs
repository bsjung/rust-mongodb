
use serde::Serialize;
use chrono::prelude::*;

use crate::database;
use crate::errors::ApiError;

#[derive(Clone, Debug)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub phone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub uid: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub phone: String,
}

/// Helper function : doc --> model
fn doc_to_model(doc: &bson::Document) -> UserResponse {
    UserResponse {
        uid: String::from(doc.get_str("uid").unwrap()),
        email: String::from(doc.get_str("email").unwrap()),
        password: String::from(doc.get_str("password").unwrap()),
        name: String::from(doc.get_str("name").unwrap()),
        phone: String::from(doc.get_str("phone").unwrap()),
        //created_at: doc.get_utc_datetime("created_at").unwrap().clone(),
        //updated_at: doc.get_utc_datetime("updated_at").unwrap().clone(),
    }
}

/// Find User by id and password info
pub async fn find_by_auth( id: &str , password: &str ) -> Option<UserResponse> {
    println!("[DEBUG] find_by_auth ");
    println!("id : {}", id);
    println!("password : {}", password);
    let coll = database::collection("users");
    let query = doc! { "uid" : id.to_string(), "password" : password.to_string() };
    let cursor = coll.find_one(query, None).unwrap();
    cursor.map(|x| doc_to_model(&x))
}

/// Get User with query
pub fn get_data(query : bson::Document ) -> Result<Vec<Option<UserResponse>>, ApiError> {
    let coll = database::collection("users");
    let cursor = coll.find(query, None).unwrap();

    let result = cursor.into_iter()
        .map(|document| {
            match document {
                Ok(doc) => Ok(Some(doc_to_model(&doc))),
                Err(e) => return Err(e).map_err(ApiError::DBError)
            }
        })
        .collect::<Result<Vec<Option<UserResponse>>, ApiError>>();

    result.into()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_get_data() {
       let query = doc! { "uid" => "bsjung"};
       let results = get_data(query);
       /*
       for item in results {
         println! ("{:#?}", item);
       }
       */
    }

    #[test]
    #[ignore]
    pub fn test_create_data() {
      println!("Creating User");
      let data = doc! {
            "uid" : "bsjung",
            "email" : "bsjung@gmail.com",
            "password" : "xxxx",
            "name" : "Benjamin Jung",
            "phone" : "010-5049-xxxx",
            "created_at" : Utc::now(),
            "updated_at" : Utc::now(),
           };
      let result = database::create("users", data);
    
      if result.is_err()  {
          println!("Could not result new user!")
      }
    }
}

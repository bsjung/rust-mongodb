use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use crate::config::CONFIG;
use crate::errors::ApiError;

lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {
    Client::with_uri_str(&CONFIG.database_url)
        .expect("Failed to initialize standalone client.")
}

pub fn collection(coll_name: &str) -> Collection {
    MONGO.database(&CONFIG.database).collection(coll_name)
}

/// Insert User with data
pub fn create(table : &str, data : bson::Document) -> Result<String, ApiError> {
    let coll = collection(table);
    let insert_result = coll.insert_one(data, None);

    match insert_result {
        Ok(result) => Ok(result.inserted_id.to_string()),
        Err(e) => return Err(e).map_err(ApiError::DBError)
    }
}

/// Update User with filter and set query.
pub fn update(table : &str, filter : bson::Document, set : bson::Document) -> Result<String, ApiError> {
    let coll = collection(table);
    let update_result = coll.update_one(filter, set, None);

    match update_result {
        Ok(_) => Ok("ok".to_string()),
        Err(e) => return Err(e).map_err(ApiError::DBError)
    }
}

/// Delete User with filter
pub fn delete(table : &str, filter : bson::Document) -> Result<String, ApiError> {
    let coll = collection(table);
    let delete_result = coll.delete_many(filter, None);

    match delete_result {
        Ok(_) => Ok("ok".to_string()),
        Err(e) => return Err(e).map_err(ApiError::DBError)
    }
}


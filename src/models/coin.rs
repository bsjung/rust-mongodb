use serde::Serialize;
use chrono::prelude::*;

use crate::database;
use crate::errors::ApiError;

#[derive(Clone, Debug)]
pub struct Coin {
    pub uid: String,
    pub coin: String,
    pub ticker: String,
    pub amount: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoinResponse {
    pub uid: String,
    pub coin: String,
    pub ticker: String,
    pub amount: String,
}

/// Helper function : doc --> model
fn doc_to_model(doc: &bson::Document) -> CoinResponse {
    CoinResponse {
        uid: String::from(doc.get_str("uid").unwrap()),
        coin: String::from(doc.get_str("coin").unwrap()),
        ticker: String::from(doc.get_str("ticker").unwrap()),
        amount: String::from(doc.get_str("amount").unwrap()),
        //created_at: doc.get_utc_datetime("created_at").unwrap().clone(),
        //updated_at: doc.get_utc_datetime("updated_at").unwrap().clone(),
    }
}

/// Get Coin with query
pub fn get_data(query : bson::Document ) -> Result<Vec<Option<CoinResponse>>, ApiError> {
    let coll = database::collection("coins");
    let cursor = coll.find(query, None).unwrap();

    let result = cursor.into_iter()
        .map(|document| {
            match document {
                Ok(doc) => Ok(Some(doc_to_model(&doc))),
                Err(e) => return Err(e).map_err(ApiError::DBError)
            }
        })
        .collect::<Result<Vec<Option<CoinResponse>>, ApiError>>();

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
      println!("Creating Coin");
      let data = doc! {
            "uid" => "bsjung",
            "coin" => "Bitcoin",
            "ticker" => "BTC",
            "amount" => "100",
            "created_at" => Utc::now(),
            "updated_at" => Utc::now(),
           };
      let result = database::create("coins", data);
    
      if result.is_err()  {
          println!("Could not insert  new coin!")
      }
    }
}

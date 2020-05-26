use crate::errors::ApiError;
use actix_web::{body::Body, web::{HttpResponse, Json}};
use serde::Serialize;

/// Helper function to reduce boilerplate of an OK/Json response
pub fn respond_json<T>(data: T) -> Result<Json<T>, ApiError> where T: Serialize, {
    Ok(Json(data))
}

/// Helper function to reduce boilerplate of an empty OK response
pub fn respond_ok() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body(Body::Empty))
}

/// Log
pub fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                buf.default_styled_level(record.level()),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    pub struct TestResponse {
        pub first_name: String,
    }

    #[test]
    fn it_responds_json() {
        let response = TestResponse {
            first_name: "Satoshi".into(),
        };
        let result = respond_json(response.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner(), response);
    }

    #[test]
    fn it_responds_ok() {
        let result = respond_ok();
        assert!(result.is_ok());
    }
}

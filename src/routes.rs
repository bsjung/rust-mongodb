
use crate::handlers::{
    auth::{login, logout},
    health::get_health,
    user::{create_user, delete_user, get_user, get_users, update_user},
};
use crate::middleware::auth::Auth as AuthMiddleware;
use actix_web::{web, HttpRequest};


async fn get_hello(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // hello
        .route("/hello", web::get().to(get_hello))
        // Healthcheck
        .route("/", web::get().to(get_health))
        .route("/health", web::get().to(get_health))
        // /api/v1 routes
        .service(
            web::scope("/api/v1")
                // Lock down routes with AUTH Middleware
                .wrap(AuthMiddleware)
                // AUTH routes
                .service(
                    web::scope("/auth")
                        .route("/login", web::post().to(login))
                        .route("/logout", web::get().to(logout)),
                )
                // USER routes
                .service(
                    web::scope("/user")
                        .route("/{id}", web::get().to(get_user))
                        .route("/{id}", web::put().to(update_user))
                        .route("/{id}", web::delete().to(delete_user))
                        .route("", web::get().to(get_users))
                        .route("", web::post().to(create_user))
                ),
        );
}

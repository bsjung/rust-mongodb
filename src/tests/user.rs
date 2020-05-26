#[cfg(test)]
mod tests {
    use crate::handlers::user::{UserRequest};
    use crate::tests::helpers::tests::{assert_get, assert_post};
    use actix_web::web::Path;

    const PATH: &str = "/api/v1/user";

    #[actix_rt::test]
    #[ignore]
    async fn it_get_user() {
        let uid: Path<String> = String::from(PATH).into();
        let user_id = uid.split('/').last().unwrap();
        let url = format!("{}/{}", PATH, user_id);
        assert_get(&url).await;
    }

    #[actix_rt::test]
    #[ignore]
    async fn it_get_users() {
        assert_get(PATH).await;
    }

    #[actix_rt::test]
    #[ignore]
    async fn it_create_user() {
        let params = UserRequest {
            uid: "bsjung2".into(),
            email: "bsjung@gmail.com".into(),
            password: "123456".into(),
            name: "Benjaming".into(),
            phone: "010-xxxx-xxxx".into(),
        };
        assert_post(PATH, params).await;
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::helpers::tests::assert_get;

    #[actix_rt::test]
    #[ignore]
    async fn test_health() {
        assert_get("/health").await;
    }
}

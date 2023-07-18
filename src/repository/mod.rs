use async_trait::async_trait;

#[async_trait]
pub trait Repository<'a> {
    type Output;

    fn new() -> Self;
    async fn find_all(self);
    async fn find(self, id: String);
}
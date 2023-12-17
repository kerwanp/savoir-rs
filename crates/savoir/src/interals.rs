#[async_trait::async_trait]
pub trait AsyncTryFrom<T>: Sized {
    type Error;
    async fn async_try_from(value: T) -> Result<Self, Self::Error>;
}

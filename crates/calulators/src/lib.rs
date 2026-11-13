pub trait Calculator: Send + Sync {
    type Input: Send;
    type Ctx: Send + Sync;
    type Error: std::error::Error + Send + Sync + 'static;

    fn calculate(&self, ctx: &Self::Ctx, input: Self::Input) -> Result<Box<str>, Self::Error>;
}

mod calulators;

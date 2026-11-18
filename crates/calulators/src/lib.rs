use std::any::Any;

pub trait Calculator: Any + Send + Sync {
    type Input: Send;
    type Output: Send;
    type Ctx: Send + Sync;
    type Error: std::error::Error + Send;

    fn calculate(&self, ctx: &Self::Ctx, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

mod calculator_state;
mod calulators;
mod mem;

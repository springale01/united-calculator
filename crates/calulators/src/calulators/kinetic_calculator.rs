use crate::{Calculator, calulators::GenericCalculatorError, mem::GenericMemory};

pub struct KineticCalculator {
    mem: GenericMemory,
}

impl KineticCalculator {
    fn new() -> Self {
        KineticCalculator {
            mem: GenericMemory::default(),
        }
    }
}

impl Calculator for KineticCalculator {
    type Ctx = ();
    type Error = GenericCalculatorError;
    type Input = ();
    type Output = ();
    fn calculate(&self, ctx: &Self::Ctx, input: Self::Input) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

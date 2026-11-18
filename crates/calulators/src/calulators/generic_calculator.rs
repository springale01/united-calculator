use crate::mem;
/*
    Scope:
    Handle basic arithmictic，exp and logs
*/
pub struct GenericCalculator {
    mem: mem::GenericMemory,
}

impl GenericCalculator {
    pub fn new() -> Self {
        Self {
            mem: mem::GenericMemory::default(),
        }
    }
    pub fn calculate(&self, input: &str) -> GenericCalculatorOutput {
        todo!()
    }
}
#[derive(Debug, strum::AsRefStr, strum::EnumIs)]
pub enum GenericCalculatorOutput {
    Float(tools::UnitedFloat),
    Int(i64),
}

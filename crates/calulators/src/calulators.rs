mod generic_calculator;
mod kinetic_calculator;

#[derive(Debug, thiserror::Error)]
pub enum GenericCalculatorError {
    #[error("Bad input, gotten: {input}")]
    BadInput { input: i64 },
    #[error("Custom Error: {0}")]
    Custom(Box<dyn std::error::Error + Send>),
}

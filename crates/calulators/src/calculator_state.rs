use std::{any::Any, marker::PhantomData};

use crate::Calculator;

pub struct CalculatorState<T> {
    active: Box<dyn Any>,
    // so like. you downcast active with the generic T and hope it works
    _pineapple: PhantomData<fn() -> T>,
}

#[derive(Debug, strum::Display, strum::EnumIter, strum::EnumIs)]
pub enum ActiveCalculator {
    GenericCalculator {},
}

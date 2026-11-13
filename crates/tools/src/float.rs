use std::marker::PhantomData;

mod private {
    pub trait Sealed {}
}

pub trait FloatType: private::Sealed {
    fn validate(float: f64) -> bool;
}

pub struct NonCursedFloat;
pub struct MaybeCursedFloat;

// Only our types get the secret permission!
impl private::Sealed for NonCursedFloat {}
impl private::Sealed for MaybeCursedFloat {}

impl FloatType for NonCursedFloat {
    fn validate(float: f64) -> bool {
        float == 0.0 || float.is_normal()
    }
}

impl FloatType for MaybeCursedFloat {
    fn validate(_: f64) -> bool {
        true
    }
}

pub struct GoodFloat<T: FloatType> {
    value: f64,
    _marker: PhantomData<fn() -> T>,
}

impl<T: FloatType> GoodFloat<T> {
    pub fn new(value: f64) -> Option<Self> {
        T::validate(value).then_some(Self {
            value,
            _marker: PhantomData,
        })
    }
}
#[test]
fn test() {}

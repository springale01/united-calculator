use std::marker::PhantomData;

mod private {
    pub trait Sealed {}
}

pub trait FloatType: private::Sealed {
    fn validate(float: f64) -> bool;
}
#[derive(Debug)]
pub struct NonCursedFloat;
#[derive(Debug)]
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
#[derive(Debug)]
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
    pub fn value(&self) -> f64 {
        self.value
    }
}
#[derive(Debug, strum::AsRefStr, strum::EnumIs)]
pub enum UnitedFloat {
    Cursed(GoodFloat<MaybeCursedFloat>),
    NonCursed(GoodFloat<NonCursedFloat>),
}
#[test]
fn test() {}

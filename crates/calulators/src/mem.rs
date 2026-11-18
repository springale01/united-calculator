use std::{collections::VecDeque, ops::Index, sync::Arc};

mod record;

pub(crate) struct GenericMemory {
    items: VecDeque<Calculations>,
}
impl Index<usize> for GenericMemory {
    type Output = Calculations;
    fn index(&self, index: usize) -> &Self::Output {
        &self.items.get(index).unwrap()
    }
}
impl GenericMemory {
    pub fn new(expected_capacity: Option<usize>) -> Self {
        if let Some(capcity) = expected_capacity {
            Self {
                items: VecDeque::with_capacity(capcity),
            }
        } else {
            Self {
                items: VecDeque::new(),
            }
        }
    }
    pub fn items(&self) -> impl Iterator<Item = &Calculations> {
        self.items.iter()
    }
    pub fn get(&self, index: usize) -> Option<&Calculations> {
        self.items.get(index)
    }
}
impl Default for GenericMemory {
    fn default() -> Self {
        Self::new(None)
    }
}
pub(crate) struct Calculations {
    input: Arc<[Arc<str>]>,
    output: Arc<str>,
}

impl Calculations {
    pub fn new(input: Arc<[Arc<str>]>, output: Arc<str>) -> Self {
        Self { input, output }
    }
    pub fn new_from_borrowed(input: &[&str], output: &str) -> Self {
        let input = input
            .into_iter()
            .copied()
            .map(|item| Arc::from(item))
            .collect::<Arc<[Arc<str>]>>();

        Self {
            input,
            output: output.into(),
        }
    }
    pub fn inputs(&self) -> &[Arc<str>] {
        self.input.iter().as_slice()
    }
    pub fn outputs(&self) -> &str {
        &self.output
    }
}

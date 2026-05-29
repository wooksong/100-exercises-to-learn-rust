pub struct WrappingU32 {
    #[allow(dead_code)]
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

#[allow(dead_code)]
fn example() {
    #[allow(unused_variables)]
    let wrapping: WrappingU32 = 42.into();
    #[allow(unused_variables)]
    let wrapping = WrappingU32::from(42);
}

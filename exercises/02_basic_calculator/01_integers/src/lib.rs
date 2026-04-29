fn compute(a: u32, b: u32) -> u32 {
    let multiplier: u8 = 4;
    a + b * multiplier as u32
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}

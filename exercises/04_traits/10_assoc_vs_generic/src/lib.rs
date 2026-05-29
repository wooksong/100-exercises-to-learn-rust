pub trait Power<Exponent = Self> {
    type Output;

    fn power(&self, e: Exponent) -> Self::Output;
}

impl Power<u32> for u32 {
    type Output = u32;

    fn power(&self, e: u32) -> Self::Output {
        self.pow(e)
    }
}

impl Power<u16> for u32 {
    type Output = u32;

    fn power(&self, e: u16) -> Self::Output {
        self.pow(e.into())
    }
}

impl Power<&u32> for u32 {
    type Output = u32;

    fn power(&self, e: &u32) -> Self::Output {
        self.pow(*e)
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}

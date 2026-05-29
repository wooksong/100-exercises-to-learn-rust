use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self { value: *value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self {
            value: (*value).into(),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs),
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(*rhs),
        }
    }
}

impl Add for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            value: self.value.saturating_add((*rhs).value),
        }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, rhs: &u16) -> bool {
        self.value == *rhs
    }
}

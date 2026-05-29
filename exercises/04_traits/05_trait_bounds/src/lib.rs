use std::cmp::PartialOrd;

/// Return the minimum of two values.
pub fn min<T>(left: T, right: T) -> T
where
    T: PartialOrd,
{
    if left <= right {
        left
    } else {
        right
    }
}

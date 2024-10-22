use crate::binary_search::{DynamicBinarySearch, StaticBinarySearch};

trait Traditional<D, I>
where
    D: std::cmp::PartialOrd,
    I: num::Integer,
{
    fn two_conditions(data: &[D], value: &D) -> Option<I>;
}

pub struct SignedImplementation;

pub struct UnsignedImplementation;

impl<D, I> Traditional<D, I> for SignedImplementation
where
    D: std::cmp::PartialOrd,
    I: num::Integer + num_traits::NumCast + std::marker::Copy + std::ops::Shr<u16, Output = I>,
{
    #[inline]
    fn two_conditions(data: &[D], value: &D) -> Option<I> {
        use num_traits::cast::cast;

        let mut low: I = cast(0).unwrap();
        let mut high: I = cast(data.len() - 1).unwrap();
        while low <= high {
            let mid: I = low + ((high - low) >> 1u16);
            let index: usize = cast(mid).unwrap();
            if data[index] > *value {
                high = mid - cast(1).unwrap();
            } else if data[index] < *value {
                low = mid + cast(1).unwrap();
            } else {
                return Some(mid);
            }
        }
        return None;
    }
}

impl<D, I> Traditional<D, I> for UnsignedImplementation
where
    D: std::cmp::PartialOrd,
    I: num::Integer + num_traits::NumCast + std::marker::Copy + std::ops::Shr<u16, Output = I>,
{
    #[inline]
    fn two_conditions(data: &[D], value: &D) -> Option<I> {
        use num_traits::cast::cast;

        let mut low: I = cast(0).unwrap();
        let mut high: I = cast(data.len() - 1usize).unwrap();
        while low < high {
            let mid: I = low + ((high - low + cast(1).unwrap()) >> 1u16);
            let index: usize = cast(mid).unwrap();
            if data[index] > *value {
                high = mid - cast(1).unwrap();
            } else if data[index] < *value {
                low = mid + cast(1).unwrap();
            } else {
                return Some(mid);
            }
        }
        let index: usize = cast(low).unwrap();
        if (index < data.len()) && (data[index] == *value) {
            return Some(low);
        }
        return None;
    }
}

impl<D, I> DynamicBinarySearch<D, I> for SignedImplementation
where
    D: std::cmp::PartialOrd,
    I: num::Integer + num_traits::NumCast + std::marker::Copy + std::ops::Shr<u16, Output = I>,
{
    #[inline]
    fn r#impl(&self, data: &[D], value: &D) -> Option<I> {
        if data.len() > 0 {
            return Self::two_conditions(data, value);
        }
        return None;
    }
}

impl<D, I, const N: usize> StaticBinarySearch<D, I, N> for SignedImplementation
where
    D: std::cmp::PartialOrd,
    I: num::Integer + num_traits::NumCast + std::marker::Copy + std::ops::Shr<u16, Output = I>,
{
    #[inline]
    fn r#impl(&self, data: &[D; N], value: &D) -> Option<I> {
        if N > 0 {
            return Self::two_conditions(data.as_slice(), value);
        }
        return None;
    }
}

impl<D, I> DynamicBinarySearch<D, I> for UnsignedImplementation
where
    D: std::cmp::PartialOrd,
    I: num::Integer + num_traits::NumCast + std::marker::Copy + std::ops::Shr<u16, Output = I>,
{
    #[inline]
    fn r#impl(&self, data: &[D], value: &D) -> Option<I> {
        if data.len() > 0 {
            return Self::two_conditions(data, value);
        }
        return None;
    }
}

impl<D, I, const N: usize> StaticBinarySearch<D, I, N> for UnsignedImplementation
where
    D: std::cmp::PartialOrd,
    I: num::Integer + num_traits::NumCast + std::marker::Copy + std::ops::Shr<u16, Output = I>,
{
    #[inline]
    fn r#impl(&self, data: &[D; N], value: &D) -> Option<I> {
        if N > 0 {
            return Self::two_conditions(data, value);
        }
        return None;
    }
}

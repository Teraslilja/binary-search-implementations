use crate::binary_search::{DynamicBinarySearch, StaticBinarySearch};

trait Alternative<D, I>
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned + num::Integer,
{
    fn one_condition(data: &[D], value: &D) -> Option<I>;
}

pub struct Implementation;

impl<D, I> Alternative<D, I> for Implementation
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num_traits::NumCast
        + std::marker::Copy
        + std::ops::Shr<u16, Output = I>,
{
    #[inline]
    fn one_condition(data: &[D], value: &D) -> Option<I> {
        use num_traits::cast::cast;

        let mut low: I = cast(0).unwrap();
        let mut high: I = cast(data.len() - 1).unwrap();
        while low < high {
            let mid: I = low + ((high - low + cast(1).unwrap()) >> 1u16);
            let index: usize = cast(mid).unwrap();
            if data[index] > *value {
                high = mid - cast(1).unwrap();
            } else {
                low = mid;
            }
        }
        let index: usize = cast(low).unwrap();
        if data[index] == *value {
            return Some(low);
        }
        return None;
    }
}

impl<D, I> DynamicBinarySearch<D, I> for Implementation
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num_traits::NumCast
        + std::marker::Copy
        + std::ops::Shr<u16, Output = I>,
{
    #[inline]
    fn r#impl(&self, data: &[D], value: &D) -> Option<I> {
        return if data.len() > 0 {
            Self::one_condition(data, value)
        } else {
            None
        };
    }
}

impl<D, I, const N: usize> StaticBinarySearch<D, I, N> for Implementation
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num_traits::NumCast
        + std::marker::Copy
        + std::ops::Shr<u16, Output = I>,
{
    #[inline]
    fn r#impl(&self, data: &[D; N], value: &D) -> Option<I> {
        return if N > 0 {
            Self::one_condition(data, value)
        } else {
            None
        };
    }
}

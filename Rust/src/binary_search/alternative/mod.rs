use crate::binary_search::{DataTypes, IndexTypes, UnsignedIndexType};
use crate::binary_search::{DynamicBinarySearch, StaticBinarySearch};

trait Alternative<D, I>: DataTypes<D> + UnsignedIndexType<I>
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned + num::Integer,
{
    fn one_condition(data: &[D], value: D) -> Option<I>;
}

pub struct Implementation;

impl<D: std::cmp::PartialOrd> DataTypes<D> for Implementation {}
impl<I: num::Integer> IndexTypes<I> for Implementation {}
impl<I: num_traits::Unsigned + num::Integer> UnsignedIndexType<I> for Implementation {}

impl<D, I> Alternative<D, I> for Implementation
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num_traits::NumCast
        + std::ops::Shr<u16, Output = I>
        + std::marker::Copy,
{
    #[inline]
    fn one_condition(data: &[D], value: D) -> Option<I> {
        use num_traits::cast::cast;

        let mut low: I = cast(0).unwrap();
        let mut high: I = cast(data.len() - 1).unwrap();
        while low < high {
            let mid: I = low + ((high - low + cast(1).unwrap()) >> 1u16);
            let index: usize = cast(mid).unwrap();
            if data[index] > value {
                high = mid - cast(1).unwrap();
            } else {
                low = mid;
            }
        }
        let index: usize = cast(low).unwrap();
        if data[index] == value {
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
        + std::ops::Shr<u16, Output = I>
        + std::marker::Copy,
{
    #[inline]
    fn r#impl(&self, data: &[D], value: D) -> Option<I> {
        if data.len() > 0 {
            return Self::one_condition(data, value);
        }
        return None;
    }
}

impl<D, I, const N: usize> StaticBinarySearch<D, I, N> for Implementation
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num_traits::NumCast
        + std::ops::Shr<u16, Output = I>
        + std::marker::Copy,
{
    #[inline]
    fn r#impl(&self, data: &[D; N], value: D) -> Option<I> {
        if N > 0 {
            return Self::one_condition(data, value);
        }
        return None;
    }
}

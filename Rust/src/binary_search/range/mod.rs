use crate::binary_search::{DataTypes, IndexTypes, UnsignedIndexType};
use crate::binary_search::{DynamicBinarySearch, StaticBinarySearch};

trait Range<D, I>: DataTypes<D> + UnsignedIndexType<I>
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned + num::Integer,
{
    fn with_width(data: &[D], value: D) -> Option<I>;
}

pub struct Implementation;

impl<D: std::cmp::PartialOrd> DataTypes<D> for Implementation {}
impl<I: num::Integer> IndexTypes<I> for Implementation {}
impl<I: num::Integer + num_traits::Unsigned> UnsignedIndexType<I> for Implementation {}

impl<D, I> Range<D, I> for Implementation
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num_traits::NumCast
        + std::ops::Shr<u16, Output = I>
        + std::marker::Copy,
{
    #[inline]
    fn with_width(data: &[D], value: D) -> Option<I> {
        use num_traits::cast::cast;

        let mut low: I = cast(0).unwrap();
        let mut width: I = cast(data.len()).unwrap();
        while width > cast(1).unwrap() {
            width = (width + cast(1).unwrap()) >> 1u16;
            let mid: I = low + width;
            let index: usize = cast(mid).unwrap();
            if (index < data.len()) && (data[index] <= value) {
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
            return Self::with_width(data, value);
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
            return Self::with_width(data, value);
        }
        return None;
    }
}

use crate::binary_search::{DynamicBinarySearch, StaticBinarySearch};

trait Range<D, I>
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned + num::Integer,
{
    fn with_width(data: &[D], value: &D) -> Option<I>;
}

pub struct Implementation;

impl<D, I> Range<D, I> for Implementation
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num_traits::NumCast
        + std::marker::Copy
        + std::ops::Shr<u16, Output = I>,
{
    #[inline]
    fn with_width(data: &[D], value: &D) -> Option<I> {
        use num_traits::cast::cast;

        let mut low: I = cast(0).unwrap();
        let n: I = cast(data.len()).unwrap(); //< This fails, if data length don't fit into type I
        let mut width: I = n;
        while width > cast(1).unwrap() {
            width = (width + cast(1).unwrap()) >> 1u16; //< This fails, if width +1 don't fit into type I
            let mid: I = low + width;
            if (mid < n) && (data[cast::<I, usize>(mid).unwrap()] <= *value) {
                low = mid;
            }
        }
        return if data[cast::<I, usize>(low).unwrap()] == *value {
            Some(low)
        } else {
            None
        };
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
            Self::with_width(data, value)
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
            Self::with_width(data, value)
        } else {
            None
        };
    }
}

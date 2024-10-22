use super::binary_search::{DynamicBinarySearch, StaticBinarySearch};

pub struct Dynamic;

impl Dynamic {
    pub fn filler<D, I>(n: usize) -> Vec<D>
    where
        D: std::cmp::PartialOrd
            + num_traits::NumCast
            + num_traits::bounds::UpperBounded
            + std::ops::Sub<Output = D>
            + std::ops::Add<Output = D>
            + std::ops::Rem<D, Output = D>
            + std::ops::AddAssign
            + std::marker::Copy,
        I: std::cmp::PartialEq
            + num::Integer
            + num_traits::NumCast
            + num::traits::bounds::UpperBounded,
    {
        use num_traits::cast::cast;
        use std::cmp::min;

        //// const MAX_VALUE: D = D::max_value();
        let max_value: D = D::max_value();
        //// const maxCount: usize = min(I::MAX as usize, usize::MAX - 1usize) - 0 + 1;
        let max_count: usize = min(cast(I::max_value()).unwrap(), usize::MAX - 1) - 0 + 1;
        assert!(n <= max_count);
        let max_double_value: D = if n > 1 {
            cast((n - 1) << 1u32).unwrap()
        } else {
            cast(0).unwrap()
        };
        assert!(max_double_value <= max_value);
        let mut tmp: Vec<D> = Vec::with_capacity(n);
        tmp.resize(n, max_value);
        for i in 0..n {
            let dv: D = cast(i << 1u16).unwrap();
            tmp[i] = dv;
        }
        return tmp;
    }

    pub fn test<D, I>(bs: &dyn DynamicBinarySearch<D, I>, data: &[D]) -> bool
    where
        D: std::cmp::PartialOrd
            + num_traits::NumCast
            + num_traits::bounds::UpperBounded
            + std::ops::Sub<Output = D>
            + std::ops::Add<Output = D>
            + std::ops::Rem<D, Output = D>
            + std::ops::AddAssign
            + std::marker::Copy,
        I: std::cmp::PartialEq
            + num::Integer
            + num_traits::NumCast
            + num::traits::bounds::UpperBounded
            + std::ops::Shr<u16, Output = I>,
    {
        use num_traits::cast::cast;

        let start_value: D = *data.first().unwrap_or(&cast(0).unwrap()) - cast(1).unwrap();
        let end_value: D = *data.last().unwrap_or(&cast(0).unwrap()) + cast(1).unwrap();
        let mut success: bool = true;
        //// for value in start_value..=end_value {
        let mut value = start_value;
        while value <= end_value {
            let result: Option<I> = bs.r#impl(data, &value);
            let is_even: bool = (value % cast(2).unwrap()) == cast(0).unwrap();
            let subsuccess: bool = if is_even && (data.len() > 0) {
                let index: I = cast::<D, I>(value).unwrap() >> 1;
                result == Some(index)
            } else {
                result.is_none()
            };
            success = success && subsuccess;

            value += cast(1).unwrap();
        }
        return success;
    }
}

pub struct Static;

impl Static {
    pub fn filler<D, I, const N: usize>() -> [D; N]
    where
        D: std::cmp::PartialOrd
            + num_traits::bounds::UpperBounded
            + num_traits::NumCast
            + std::ops::Sub<Output = D>
            + std::ops::Add<Output = D>
            + std::ops::Rem<D, Output = D>
            + std::ops::Shr<u16>
            + std::ops::AddAssign
            + std::marker::Copy,
        I: num::Integer + num_traits::NumCast + num_traits::bounds::UpperBounded,
    {
        use num_traits::cast::cast;
        use std::cmp::min;

        //// const MAX_VALUE: D = D::max_value();
        let max_value: D = D::max_value();
        //// const MAX_COUNT: usize = min(MAX_VALUE as usize, usize::MAX - 1usize) - 0 + 1;
        let max_count: usize = min(cast(I::max_value()).unwrap(), usize::MAX - 1) - 0 + 1;
        assert!(N <= max_count);
        let mut tmp: [D; N] = [max_value; N];
        //// const MAX_DOUBLE_VALUE: D = D::from((N - 1usize) << 1u32);
        let max_double_value: D = if N > 1 {
            cast((N - 1) << 1).unwrap()
        } else {
            cast(0).unwrap()
        };
        assert!(max_double_value <= max_value);
        for i in 0..N {
            let dv: D = cast(i << 1).unwrap();
            tmp[i] = dv;
        }
        return tmp;
    }

    pub fn test<D, I, const N: usize>(bs: &dyn StaticBinarySearch<D, I, N>, data: &[D; N]) -> bool
    where
        D: std::cmp::PartialOrd
            + num_traits::bounds::UpperBounded
            + num_traits::NumCast
            + std::ops::Sub<Output = D>
            + std::ops::Add<Output = D>
            + std::ops::Rem<D, Output = D>
            + std::ops::AddAssign
            + std::marker::Copy,
        I: num::Integer
            + num_traits::NumCast
            + num_traits::bounds::UpperBounded
            + std::ops::Shr<u16, Output = I>,
    {
        use num_traits::cast::cast;

        let start_value: D = *data.first().unwrap_or(&cast(0).unwrap()) - cast(1).unwrap();
        let end_value: D = *data.last().unwrap_or(&cast(0).unwrap()) + cast(1).unwrap();
        let mut success: bool = true;
        let mut value: D = start_value;
        //// for value in startValue..=endValue {
        while value <= end_value {
            let result: Option<I> = bs.r#impl(data, &value);
            let is_even: bool = (value % cast(2).unwrap()) == cast(0).unwrap();
            let subsuccess: bool = if is_even && (data.len() > 0) {
                let index: I = cast::<D, I>(value).unwrap() >> 1;
                result == Some(index)
            } else {
                result.is_none()
            };
            success = success && subsuccess;

            value += cast(1).unwrap();
        }
        return success;
    }
}

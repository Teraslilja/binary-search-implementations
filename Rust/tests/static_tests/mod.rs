use bs::helpers::Static;
use crate::testing::matcher::assert_true;
use crate::testing::test::{FrameWorkFixture, FrameWorkTrait};
use crate::testing::TestResult;

type DataType = i32;
type SignedIndexType = isize;
type IndexType = usize;

#[derive(Clone)]
pub struct BinarySearchStaticTests {
    pub framework_fixture: FrameWorkFixture<Self>,
}

impl FrameWorkTrait<BinarySearchStaticTests> for BinarySearchStaticTests {
    fn get_fixture(&self) -> &Self {
        return self;
    }

    fn get_framework_fixture(&self) -> &FrameWorkFixture<Self> {
        return &self.framework_fixture;
    }
}

use bs::binary_search::power;

fn test<const SIZE: usize>() -> bool {
    let testdata: [DataType; SIZE] = Static::filler::<DataType, SignedIndexType, SIZE>();
    let bs: power::StaticImplementation = power::StaticImplementation {};
    let result: bool = Static::test::<DataType, IndexType, SIZE>(&bs, &testdata);
    return result;
}

impl BinarySearchStaticTests {
    fn size_0(_fixture: &BinarySearchStaticTests) -> TestResult {
        let result: bool = test::<0>();
        return assert_true(result);
    }

    fn size_1(_fixture: &BinarySearchStaticTests) -> TestResult {
        let result: bool = test::<1>();
        return assert_true(result);
    }

    fn size_2(_fixture: &BinarySearchStaticTests) -> TestResult {
        let result: bool = test::<2>();
        return assert_true(result);
    }

    fn size_3(_fixture: &BinarySearchStaticTests) -> TestResult {
        let result: bool = test::<3>();
        return assert_true(result);
    }

    fn size_10(_fixture: &BinarySearchStaticTests) -> TestResult {
        let result: bool = test::<10>();
        return assert_true(result);
    }

    fn size_100(_fixture: &BinarySearchStaticTests) -> TestResult {
        let result: bool = test::<100>();
        return assert_true(result);
    }

    fn size_511(_fixture: &BinarySearchStaticTests) -> TestResult {
        let result: bool = test::<511>();
        return assert_true(result);
    }

    fn size_512(_fixture: &BinarySearchStaticTests) -> TestResult {
        let result: bool = test::<512>();
        return assert_true(result);
    }

    fn size_513(_fixture: &BinarySearchStaticTests) -> TestResult {
        let result: bool = test::<513>();
        return assert_true(result);
    }
    fn size_64ki(_fixture: &BinarySearchStaticTests) -> TestResult {
        const N: usize = 1usize << 16u32;
        let result: bool = test::<N>();
        return assert_true(result);
    }
}

use crate::testing::test::{TestCase, TestFixture};

pub const TESTFIXTURE: TestFixture<BinarySearchStaticTests> = TestFixture {
    name: "BinarySearchStaticTests",
    tests: &[
        TestCase {
            name: "size_0",
            test: &BinarySearchStaticTests::size_0,
        },
        TestCase {
            name: "size_1",
            test: &BinarySearchStaticTests::size_1,
        },
        TestCase {
            name: "size_2",
            test: &BinarySearchStaticTests::size_2,
        },
        TestCase {
            name: "size_3",
            test: &BinarySearchStaticTests::size_3,
        },
        TestCase {
            name: "size_10",
            test: &BinarySearchStaticTests::size_10,
        },
        TestCase {
            name: "size_100",
            test: &BinarySearchStaticTests::size_100,
        },
        TestCase {
            name: "size_511",
            test: &BinarySearchStaticTests::size_511,
        },
        TestCase {
            name: "size_512",
            test: &BinarySearchStaticTests::size_512,
        },
        TestCase {
            name: "size_513",
            test: &BinarySearchStaticTests::size_513,
        },
        TestCase {
            name: "size_64ki",
            test: &BinarySearchStaticTests::size_64ki,
        },
    ],
};

use super::helpers::Dynamic;
use crate::testing::matcher::assert_true;
use crate::testing::parameterizedtest::{FrameWorkFixture, FrameWorkTrait};
use crate::testing::TestResult;

type DataType = i32;
type SignedIndexType = isize;
type IndexType = usize;

type TestData = usize;

#[derive(Clone)]
pub struct BinarySearchDynamicTests {
    pub framework_fixture: FrameWorkFixture<Self, TestData>,
    pub argument: Vec<DataType>,
}

impl FrameWorkTrait<BinarySearchDynamicTests, TestData> for BinarySearchDynamicTests {
    fn setup(&mut self, param: &TestData) {
        self.argument = Dynamic::filler::<DataType, SignedIndexType>(*param);
    }

    fn get_fixture(&self) -> &Self {
        return self;
    }

    fn get_framework_fixture(&self) -> &FrameWorkFixture<Self, TestData> {
        return &self.framework_fixture;
    }
}

use bs::binary_search::{alternative, power, range, traditional};

impl BinarySearchDynamicTests {
    fn signed_traditional(fixture: &BinarySearchDynamicTests, _param: &TestData) -> TestResult {
        let bs: traditional::SignedImplementation = traditional::SignedImplementation {};

        let result: bool = Dynamic::test::<DataType, SignedIndexType>(&bs, &fixture.argument);
        return assert_true(result);
    }

    fn unsigned_traditional(fixture: &BinarySearchDynamicTests, _param: &TestData) -> TestResult {
        let bs: traditional::UnsignedImplementation = traditional::UnsignedImplementation {};

        let result: bool = Dynamic::test::<DataType, IndexType>(&bs, &fixture.argument);
        return assert_true(result);
    }

    fn alternative(fixture: &BinarySearchDynamicTests, _param: &TestData) -> TestResult {
        let bs: alternative::Implementation = alternative::Implementation {};

        let result: bool = Dynamic::test::<DataType, IndexType>(&bs, &fixture.argument);
        return assert_true(result);
    }

    fn range(fixture: &BinarySearchDynamicTests, _param: &TestData) -> TestResult {
        let bs: range::Implementation = range::Implementation {};

        let result: bool = Dynamic::test::<DataType, IndexType>(&bs, &fixture.argument);
        return assert_true(result);
    }

    fn power(fixture: &BinarySearchDynamicTests, _param: &TestData) -> TestResult {
        let bs: power::DynamicImplementation = power::DynamicImplementation {};

        let result: bool = Dynamic::test::<DataType, IndexType>(&bs, &fixture.argument);
        return assert_true(result);
    }
}

use crate::testing::parameterizedtest::{DataSetFixture, TestCase, TestFixture};

pub const TESTFIXTURE: TestFixture<BinarySearchDynamicTests, TestData> = TestFixture {
    name: "BinarySearchDynamicTests",
    tests: &[
        TestCase {
            name: "signed_traditional",
            test: &BinarySearchDynamicTests::signed_traditional,
        },
        TestCase {
            name: "unsigned_traditional",
            test: &BinarySearchDynamicTests::unsigned_traditional,
        },
        TestCase {
            name: "alternative",
            test: &BinarySearchDynamicTests::alternative,
        },
        TestCase {
            name: "range",
            test: &BinarySearchDynamicTests::range,
        },
        TestCase {
            name: "power",
            test: &BinarySearchDynamicTests::power,
        },
    ],
};

pub const DATASET: DataSetFixture<TestData> = DataSetFixture {
    name: "ArraySizes",
    dataset: &[0, 1, 2, 3, 10, 100, 511, 512, 513, 1 << 16],
};

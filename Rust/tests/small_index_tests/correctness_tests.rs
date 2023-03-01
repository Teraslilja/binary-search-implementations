use super::{DataType, SmallIndexType};
use crate::helpers::{Dynamic, Static};
use crate::testing::matcher::{assert_true, expect_fatal_failure};
use crate::testing::test::{FrameWorkFixture, FrameWorkTrait};
use crate::testing::TestResult;

#[derive(Clone)]
pub struct IncorrectnessTests {
    pub framework_fixture: FrameWorkFixture<Self>,
}

impl FrameWorkTrait<IncorrectnessTests> for IncorrectnessTests {
    fn get_fixture(&self) -> &Self {
        return self;
    }

    fn get_framework_fixture(&self) -> &FrameWorkFixture<Self> {
        return &self.framework_fixture;
    }
}

use bs::binary_search::range;

impl IncorrectnessTests {
    fn range(_fixture: &IncorrectnessTests) -> TestResult {
        let bs: range::Implementation = range::Implementation {};
        const SIZE: usize = SmallIndexType::MAX as usize - 0 + 1;
        let argument: Vec<DataType> = Dynamic::filler::<DataType, SmallIndexType>(SIZE);
        assert!(argument.len() == SIZE);

        return expect_fatal_failure(move || {
            let result: bool = Dynamic::test::<DataType, SmallIndexType>(&bs, &argument);
            return assert_true(result);
        });
    }
}

#[derive(Clone)]
pub struct CorrectnessTests {
    pub framework_fixture: FrameWorkFixture<Self>,
}

impl FrameWorkTrait<CorrectnessTests> for CorrectnessTests {
    fn get_fixture(&self) -> &Self {
        return self;
    }

    fn get_framework_fixture(&self) -> &FrameWorkFixture<Self> {
        return &self.framework_fixture;
    }
}

use bs::binary_search::power;

impl CorrectnessTests {
    fn power_dynamic(_fixture: &CorrectnessTests) -> TestResult {
        let bs: power::DynamicImplementation = power::DynamicImplementation {};
        const SIZE: usize = SmallIndexType::MAX as usize - 0 + 1;
        let argument: Vec<DataType> = Dynamic::filler::<DataType, SmallIndexType>(SIZE);
        assert!(argument.len() == SIZE);

        let result: bool = Dynamic::test::<DataType, SmallIndexType>(&bs, &argument);
        return assert_true(result);
    }

    fn power_static(_fixture: &CorrectnessTests) -> TestResult {
        let bs: power::StaticImplementation = power::StaticImplementation {};
        const SIZE: usize = SmallIndexType::MAX as usize - 0 + 1;
        let argument: [DataType; SIZE] = Static::filler::<DataType, SmallIndexType, SIZE>();
        assert!(argument.len() == SIZE);

        let result: bool = Static::test::<DataType, SmallIndexType, SIZE>(&bs, &argument);
        return assert_true(result);
    }
}

use crate::testing::test::{TestCase, TestFixture};

pub const INCORRECTNESS_TESTFIXTURE: TestFixture<IncorrectnessTests> = TestFixture {
    name: "IncorrectnessTests",
    tests: &[TestCase {
        name: "rangeFails",
        test: &IncorrectnessTests::range,
    }],
};

pub const CORRECTNESS_TESTFIXTURE: TestFixture<CorrectnessTests> = TestFixture {
    name: "CorrectnessTests",
    tests: &[
        TestCase {
            name: "power_dynamicPasses",
            test: &CorrectnessTests::power_dynamic,
        },
        TestCase {
            name: "power_staticPasses",
            test: &CorrectnessTests::power_static,
        },
    ],
};

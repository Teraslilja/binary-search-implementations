use crate::testing::matcher::assert_eq;
use crate::testing::parameterizedtest::{FrameWorkFixture, FrameWorkTrait};
use crate::testing::TestResult;

#[derive(Debug, Clone)]
pub struct ExpectedResults {
    is_power_of_2: bool,
    log2: Option<u32>,
    previous_power: usize,
}

#[derive(Debug, Clone)]
pub struct UtilityData {
    value: usize,
    expected_results: ExpectedResults,
}

#[derive(Clone)]
pub struct UtilityTests {
    pub framework_fixture: FrameWorkFixture<UtilityTests, UtilityData>,
}

impl FrameWorkTrait<UtilityTests, UtilityData> for UtilityTests {
    fn setup(&mut self, _param: &UtilityData) {}

    fn get_fixture(&self) -> &Self {
        return self;
    }

    fn get_framework_fixture(&self) -> &FrameWorkFixture<UtilityTests, UtilityData> {
        return &self.framework_fixture;
    }
}

use bs::binary_search::power::utility::{is_power_of_two, log2, previous_power_of_two};

impl UtilityTests {
    fn is_power2_correct_answer(
        _fixture: &UtilityTests,
        param: &UtilityData,
    ) -> TestResult {
        let result: bool = is_power_of_two(param.value);
        return assert_eq(result, param.expected_results.is_power_of_2);
    }

    fn log2_correct_answer(_fixture: &UtilityTests, param: &UtilityData) -> TestResult {
        let result: Option<u32> = log2(param.value);
        return assert_eq(result, param.expected_results.log2);
    }

    fn previous_power_correct_answer(
        _fixture: &UtilityTests,
        param: &UtilityData,
    ) -> TestResult {
        let result: usize = previous_power_of_two(param.value);
        return assert_eq(result, param.expected_results.previous_power);
    }
}

use crate::testing::parameterizedtest::{DataSetFixture, TestCase, TestFixture};

pub const TESTFIXTURE: TestFixture<UtilityTests, UtilityData> = TestFixture {
    name: "UtilityTests",
    tests: &[
        TestCase {
            name: "is_power2_correct_answer",
            test: &UtilityTests::is_power2_correct_answer,
        },
        TestCase {
            name: "log2_correct_answer",
            test: &UtilityTests::log2_correct_answer,
        },
        TestCase {
            name: "previous_power_correct_answer",
            test: &UtilityTests::previous_power_correct_answer,
        },
    ],
};

#[rustfmt::skip]
pub const DATASET1: DataSetFixture<UtilityData> = DataSetFixture{
    name: "ExactPowers",
    dataset: &[
        UtilityData {value: 1usize << 0u32,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: Some(0u32),
                                                        previous_power: 0usize,},},
        UtilityData {value: 1usize << 1u32,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: Some(1u32),
                                                        previous_power: 1usize << 0u32,},},
        UtilityData {value: 1usize << 2u32,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: Some(2u32),
                                                        previous_power: 1usize << 1u32,},},
        UtilityData {value: 1usize << 3u32,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: Some(3u32),
                                                        previous_power: 1usize << 2u32,},},
        UtilityData {value: 1usize << 4u32,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: Some(4u32),
                                                        previous_power: 1usize << 3u32,},},
        UtilityData {value: 1usize << 5u32,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: Some(5u32),
                                                        previous_power: 1usize << 4u32,},},
        UtilityData {value: 1usize << 6u32,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: Some(6u32),
                                                        previous_power: 1usize << 5u32,},},
        UtilityData {value: 1usize << 7u32,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: Some(7u32),
                                                        previous_power: 1usize << 6u32,},},
    ],
};

#[rustfmt::skip]
pub const DATASET2: DataSetFixture<UtilityData> = DataSetFixture{
    name: "UnderPowers",
    dataset: &[
        UtilityData {value: (1usize << 2u32) - 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(1u32),
                                                        previous_power: 1usize << 1u32,},},
        UtilityData {value: (1usize << 3u32) - 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(2u32),
                                                        previous_power: 1usize << 2u32,},},
        UtilityData {value: (1usize << 4u32) - 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(3u32),
                                                        previous_power: 1usize << 3u32,},},
        UtilityData {value: (1usize << 5u32) - 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(4u32),
                                                        previous_power: 1usize << 4u32,},},
        UtilityData {value: (1usize << 6u32) - 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(5u32),
                                                        previous_power: 1usize << 5u32,},},
        UtilityData {value: (1usize << 7u32) - 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(6u32),
                                                        previous_power: 1usize << 6u32,},},
    ],
};

#[rustfmt::skip]
pub const DATASET3: DataSetFixture<UtilityData> = DataSetFixture{
    name: "OverPowers",
    dataset: &[
        UtilityData {value: (1usize << 1u32) + 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(1u32),
                                                        previous_power: 1usize << 1u32,},},
        UtilityData {value: (1usize << 2u32) + 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(2u32),
                                                        previous_power: 1usize << 2u32,},},
        UtilityData {value: (1usize << 3u32) + 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(3u32),
                                                        previous_power: 1usize << 3u32,},},
        UtilityData {value: (1usize << 4u32) + 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(4u32),
                                                        previous_power: 1usize << 4u32,},},
        UtilityData {value: (1usize << 5u32) + 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(5u32),
                                                        previous_power: 1usize << 5u32,},},
        UtilityData {value: (1usize << 6u32) + 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(6u32),
                                                        previous_power: 1usize << 6u32,},},
        UtilityData {value: (1usize << 7u32) + 1usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(7u32),
                                                        previous_power: 1usize << 7u32,},},
    ],
};

#[rustfmt::skip]
pub const DATASET4: DataSetFixture<UtilityData> = DataSetFixture {
    name: "Specials",
    dataset: &[
        UtilityData {value: 0usize,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: None,
                                                        previous_power: 0usize,},},
        UtilityData {value: 1usize,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: Some(0u32),
                                                        previous_power: 0usize,},},
        UtilityData {value: 1usize << 61u32,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: Some(61u32),
                                                        previous_power: 1usize << 60u32,},},
        UtilityData {value: 1usize << 62u32,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: Some(62u32),
                                                        previous_power: 1usize << 61u32,},},
        UtilityData {value: 1usize << 63u32,
                     expected_results: ExpectedResults {is_power_of_2: true,
                                                        log2: Some(63u32),
                                                        previous_power: 1usize << 62u32,},},
        UtilityData {value: u8::MAX as usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(7u32),
                                                        previous_power: 1usize << 7u32,},},
        UtilityData {value: u16::MAX as usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(15u32),
                                                        previous_power: 1usize << 15u32,},},
        UtilityData {value: u32::MAX as usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(31u32),
                                                        previous_power: 1usize << 31u32,},},
        UtilityData {value: u64::MAX as usize,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(63u32),
                                                        previous_power: 1usize << 63u32,},},
        UtilityData {value: usize::MAX,
                     expected_results: ExpectedResults {is_power_of_2: false,
                                                        log2: Some(63u32),
                                                        previous_power: 1usize << 63u32,},},
    ],
};


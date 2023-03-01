use super::{DataType, SignedSmallIndexType, SmallIndexType};
use crate::helpers::Dynamic;
use crate::testing::matcher::assert_death_or_timeout;
use crate::testing::test::{FrameWorkFixture, FrameWorkTrait};
use crate::testing::TestResult;

const TIMEOUT_DURATION: i64 = 100;

#[derive(Clone)]
pub struct PanicOrTimeoutDeathTests {
    pub framework_fixture: FrameWorkFixture<Self>,
    pub timeout_after: chrono::Duration,
}

impl FrameWorkTrait<PanicOrTimeoutDeathTests> for PanicOrTimeoutDeathTests {
    fn setup(&mut self) {
        self.timeout_after = chrono::Duration::milliseconds(TIMEOUT_DURATION);
    }

    fn get_fixture(&self) -> &Self {
        return self;
    }

    fn get_framework_fixture(&self) -> &FrameWorkFixture<Self> {
        return &self.framework_fixture;
    }
}

use bs::binary_search::{alternative, range, traditional};

impl PanicOrTimeoutDeathTests {
    fn signed_traditional(fixture: &PanicOrTimeoutDeathTests) -> TestResult {
        let bs: traditional::SignedImplementation = traditional::SignedImplementation {};
        const SIZE: usize = SignedSmallIndexType::MAX as usize - 0 + 1;
        let argument: Vec<DataType> = Dynamic::filler::<DataType, SignedSmallIndexType>(SIZE);
        assert!(argument.len() == SIZE);

        return assert_death_or_timeout(fixture.timeout_after, move || {
            let _result: bool = Dynamic::test::<DataType, SignedSmallIndexType>(&bs, &argument);
        });
    }

    fn unsigned_traditional(fixture: &PanicOrTimeoutDeathTests) -> TestResult {
        let bs: traditional::UnsignedImplementation = traditional::UnsignedImplementation {};
        const SIZE: usize = SmallIndexType::MAX as usize - 0 + 1;
        let argument: Vec<DataType> = Dynamic::filler::<DataType, SmallIndexType>(SIZE);
        assert!(argument.len() == SIZE);

        return assert_death_or_timeout(fixture.timeout_after, move || {
            let _result: bool = Dynamic::test::<DataType, SmallIndexType>(&bs, &argument);
        });
    }

    fn alternative(fixture: &PanicOrTimeoutDeathTests) -> TestResult {
        let bs: alternative::Implementation = alternative::Implementation {};
        const SIZE: usize = SmallIndexType::MAX as usize - 0 + 1;
        let argument: Vec<DataType> = Dynamic::filler::<DataType, SmallIndexType>(SIZE);
        assert!(argument.len() == SIZE);

        return assert_death_or_timeout(fixture.timeout_after, move || {
            let _result: bool = Dynamic::test::<DataType, SmallIndexType>(&bs, &argument);
        });
    }

    fn range(fixture: &PanicOrTimeoutDeathTests) -> TestResult {
        let bs: range::Implementation = range::Implementation {};
        const SIZE: usize = SmallIndexType::MAX as usize - 0 + 1;
        let argument: Vec<DataType> = Dynamic::filler::<DataType, SmallIndexType>(SIZE);
        assert!(argument.len() == SIZE);

        return assert_death_or_timeout(fixture.timeout_after, move || {
            let _result: bool = Dynamic::test::<DataType, SmallIndexType>(&bs, &argument);
        });
    }
}

use crate::testing::test::{TestCase, TestFixture};

pub const TESTFIXTURE: TestFixture<PanicOrTimeoutDeathTests> = TestFixture {
    name: "PanicOrTimeoutDeathTests",
    tests: &[
        TestCase {
            name: "signed_traditionalPanicsOrHangs",
            test: &PanicOrTimeoutDeathTests::signed_traditional,
        },
        TestCase {
            name: "unsigned_traditionalPanicsOrHangs",
            test: &PanicOrTimeoutDeathTests::unsigned_traditional,
        },
        TestCase {
            name: "alternativePanicsOrHangs",
            test: &PanicOrTimeoutDeathTests::alternative,
        },
        TestCase {
            name: "rangePanicsOrHangs",
            test: &PanicOrTimeoutDeathTests::range,
        },
    ],
};

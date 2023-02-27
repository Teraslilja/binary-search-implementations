use super::helpers::{Dynamic, Static};
use crate::testing::matcher::assert_death_or_timeout;
use crate::testing::test::{FrameWorkFixture, FrameWorkTrait};
use crate::testing::TestResult;

type DataType = i32;
type SignedSmallIndexType = i8;
type SmallIndexType = u8;

const TIMEOUT_DURATION: i64 = 100;

#[derive(Clone)]
pub struct PanicOrTimeoutDeathTests {
    pub framework_fixture: FrameWorkFixture<Self>,
}

impl FrameWorkTrait<PanicOrTimeoutDeathTests> for PanicOrTimeoutDeathTests {
    fn get_fixture(&self) -> &Self {
        return self;
    }

    fn get_framework_fixture(&self) -> &FrameWorkFixture<Self> {
        return &self.framework_fixture;
    }
}

use bs::binary_search::{alternative, power, traditional};

impl PanicOrTimeoutDeathTests {
    fn signed_traditional(_fixture: &PanicOrTimeoutDeathTests) -> TestResult {
        let timeout_after: chrono::Duration = chrono::Duration::milliseconds(TIMEOUT_DURATION);
        let bs: traditional::SignedImplementation = traditional::SignedImplementation {};
        const SIZE: usize = SignedSmallIndexType::MAX as usize - 0 + 1;
        let argument: Vec<DataType> = Dynamic::filler::<DataType, SignedSmallIndexType>(&SIZE);
        assert!(argument.len() == SIZE);

        return assert_death_or_timeout(timeout_after, move || {
            let _result: bool = Dynamic::test::<DataType, SignedSmallIndexType>(&bs, &argument);
        });
    }

    fn unsigned_traditional(_fixture: &PanicOrTimeoutDeathTests) -> TestResult {
        let timeout_after: chrono::Duration = chrono::Duration::milliseconds(TIMEOUT_DURATION);
        let bs: traditional::UnsignedImplementation = traditional::UnsignedImplementation {};
        const SIZE: usize = SmallIndexType::MAX as usize - 0 + 1;
        let argument: Vec<DataType> = Dynamic::filler::<DataType, SmallIndexType>(&SIZE);
        assert!(argument.len() == SIZE);

        return assert_death_or_timeout(timeout_after, move || {
            let _result: bool = Dynamic::test::<DataType, SmallIndexType>(&bs, &argument);
        });
    }

    fn alternative(_fixture: &PanicOrTimeoutDeathTests) -> TestResult {
        let timeout_after: chrono::Duration = chrono::Duration::milliseconds(TIMEOUT_DURATION);
        let bs: alternative::Implementation = alternative::Implementation {};
        const SIZE: usize = SmallIndexType::MAX as usize - 0 + 1;
        let argument: Vec<DataType> = Dynamic::filler::<DataType, SmallIndexType>(&SIZE);
        assert!(argument.len() == SIZE);

        return assert_death_or_timeout(timeout_after, move || {
            let _result: bool = Dynamic::test::<DataType, SmallIndexType>(&bs, &argument);
        });
    }

    fn power_dynamic(_fixture: &PanicOrTimeoutDeathTests) -> TestResult {
        let timeout_after: chrono::Duration = chrono::Duration::milliseconds(TIMEOUT_DURATION);
        let bs: power::DynamicImplementation = power::DynamicImplementation {};
        const SIZE: usize = SmallIndexType::MAX as usize - 0 + 1;
        let argument: Vec<DataType> = Dynamic::filler::<DataType, SmallIndexType>(&SIZE);
        assert!(argument.len() == SIZE);

        return assert_death_or_timeout(timeout_after, move || {
            let _result: bool = Dynamic::test::<DataType, SmallIndexType>(&bs, &argument);
        });
    }

    fn power_static(_fixture: &PanicOrTimeoutDeathTests) -> TestResult {
        let timeout_after: chrono::Duration = chrono::Duration::milliseconds(TIMEOUT_DURATION);
        let bs: power::StaticImplementation = power::StaticImplementation {};
        const SIZE: usize = SmallIndexType::MAX as usize - 0 + 1;
        let argument: [DataType; SIZE] = Static::filler::<DataType, SmallIndexType, SIZE>();
        assert!(argument.len() == SIZE);

        return assert_death_or_timeout(timeout_after, move || {
            let _result: bool = Static::test::<DataType, SmallIndexType, SIZE>(&bs, &argument);
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
            name: "power_dynamic",
            test: &PanicOrTimeoutDeathTests::power_dynamic,
        },
        TestCase {
            name: "power_static",
            test: &PanicOrTimeoutDeathTests::power_static,
        },
    ],
};

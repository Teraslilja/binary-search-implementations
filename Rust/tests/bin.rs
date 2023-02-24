extern crate bs;

pub mod testing;
pub mod utility_tests;
pub mod helpers;
pub mod dynamic_tests;

#[test]
fn utility_tests()
{
    use testing::parameterizedtest::{FrameWorkFixture, FrameWorkTrait};
    use utility_tests::UtilityTests;
    use utility_tests::TESTFIXTURE;
    use utility_tests::{DATASET1, DATASET2, DATASET3, DATASET4};

    let mut tests1: UtilityTests = UtilityTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
            dataset_fixture: DATASET1,
        },
    };
    tests1.run_all_tests();

    let mut tests2: UtilityTests = UtilityTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
            dataset_fixture: DATASET2,
        },
    };
    tests2.run_all_tests();

    let mut tests3: UtilityTests = UtilityTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
            dataset_fixture: DATASET3,
        },
    };
    tests3.run_all_tests();

    let mut tests4: UtilityTests = UtilityTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
            dataset_fixture: DATASET4,
        },
    };
    tests4.run_all_tests();
}

#[test]
fn binary_search_dynamic_tests()
{
    use testing::parameterizedtest::{FrameWorkFixture, FrameWorkTrait};
    use dynamic_tests::BinarySearchDynamicTests;
    use dynamic_tests::TESTFIXTURE;
    use dynamic_tests::{DATASET5};

    let mut tests5: BinarySearchDynamicTests = BinarySearchDynamicTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
            dataset_fixture: DATASET5,
        },
        argument: vec![],
    };
    tests5.run_all_tests();
}

pub fn main() {
    utility_tests();
    binary_search_dynamic_tests();
}

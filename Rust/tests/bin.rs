extern crate bs;

pub mod dynamic_tests;
pub mod helpers;
pub mod static_tests;
pub mod testing;
pub mod utility_tests;

fn utility_tests_func() {
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

fn binary_search_dynamic_tests_func() {
    use dynamic_tests::BinarySearchDynamicTests;
    use dynamic_tests::DATASET5;
    use dynamic_tests::TESTFIXTURE;
    use testing::parameterizedtest::{FrameWorkFixture, FrameWorkTrait};

    let mut tests5: BinarySearchDynamicTests = BinarySearchDynamicTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
            dataset_fixture: DATASET5,
        },
        argument: vec![],
    };
    tests5.run_all_tests();
}

fn binary_search_static_tests_func() {
    use static_tests::BinarySearchStaticTests;
    use static_tests::TESTFIXTURE;
    use testing::test::{FrameWorkFixture, FrameWorkTrait};

    let mut tests: BinarySearchStaticTests = BinarySearchStaticTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
        },
    };
    tests.run_all_tests();
}

#[test]
fn utility_tests() {
    utility_tests_func();
}

#[test]
fn binary_search_dynamic_tests() {
    binary_search_dynamic_tests_func();
}

#[test]
fn binary_search_static_tests() {
    binary_search_static_tests_func();
}

pub fn main() {
    utility_tests_func();
    binary_search_dynamic_tests_func();
    binary_search_static_tests_func();
}

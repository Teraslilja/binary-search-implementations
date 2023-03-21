extern crate bs;

pub mod dynamic_tests;
pub mod small_index_tests;
pub mod static_tests;
pub mod testing;
pub mod utility_tests;

fn utility_tests_func() -> bool {
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

    let mut tests2: UtilityTests = UtilityTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
            dataset_fixture: DATASET2,
        },
    };

    let mut tests3: UtilityTests = UtilityTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
            dataset_fixture: DATASET3,
        },
    };

    let mut tests4: UtilityTests = UtilityTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
            dataset_fixture: DATASET4,
        },
    };

    return tests1.run_all_tests()
        && tests2.run_all_tests()
        && tests3.run_all_tests()
        && tests4.run_all_tests();
}

fn binary_search_dynamic_tests_func() -> bool {
    use dynamic_tests::BinarySearchDynamicTests;
    use dynamic_tests::DATASET;
    use dynamic_tests::TESTFIXTURE;
    use testing::parameterizedtest::{FrameWorkFixture, FrameWorkTrait};

    let mut tests: BinarySearchDynamicTests = BinarySearchDynamicTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
            dataset_fixture: DATASET,
        },
        argument: vec![],
    };
    return tests.run_all_tests();
}

fn binary_search_static_tests_func() -> bool {
    use static_tests::BinarySearchStaticTests;
    use static_tests::TESTFIXTURE;
    use testing::test::{FrameWorkFixture, FrameWorkTrait};

    let mut tests: BinarySearchStaticTests = BinarySearchStaticTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
        },
    };
    return tests.run_all_tests();
}

fn panic_or_timeout_tests_func() -> bool {
    use small_index_tests::panic_or_timeout_tests::PanicOrTimeoutDeathTests;
    use small_index_tests::panic_or_timeout_tests::TESTFIXTURE;
    use testing::test::{FrameWorkFixture, FrameWorkTrait};

    let mut tests: PanicOrTimeoutDeathTests = PanicOrTimeoutDeathTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: TESTFIXTURE,
        },
        timeout_after: chrono::Duration::zero(),
    };
    return tests.run_all_tests();
}

fn incorrectness_tests_func() -> bool {
    use small_index_tests::correctness_tests::IncorrectnessTests;
    use small_index_tests::correctness_tests::INCORRECTNESS_TESTFIXTURE;
    use testing::test::{FrameWorkFixture, FrameWorkTrait};

    let mut tests: IncorrectnessTests = IncorrectnessTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: INCORRECTNESS_TESTFIXTURE,
        },
    };
    return tests.run_all_tests();
}

fn correctness_tests_func() -> bool {
    use small_index_tests::correctness_tests::CorrectnessTests;
    use small_index_tests::correctness_tests::CORRECTNESS_TESTFIXTURE;
    use testing::test::{FrameWorkFixture, FrameWorkTrait};

    let mut tests: CorrectnessTests = CorrectnessTests {
        framework_fixture: FrameWorkFixture {
            test_fixture: CORRECTNESS_TESTFIXTURE,
        },
    };
    return tests.run_all_tests();
}

#[test]
fn utility_tests() {
    assert!(utility_tests_func());
}

#[test]
fn binary_search_dynamic_tests() {
    assert!(binary_search_dynamic_tests_func());
}

#[test]
fn binary_search_static_tests() {
    assert!(binary_search_static_tests_func());
}

#[test]
fn panic_or_timeout_tests() {
    assert!(panic_or_timeout_tests_func());
}

#[test]
fn incorrectness_tests() {
    assert!(incorrectness_tests_func());
}

#[test]
fn correctness_tests() {
    assert!(correctness_tests_func());
}

pub fn main() {
    utility_tests_func();
    binary_search_dynamic_tests_func();
    binary_search_static_tests_func();
    panic_or_timeout_tests_func();
    incorrectness_tests_func();
    correctness_tests_func();
}

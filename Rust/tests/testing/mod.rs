pub enum TestResult {
    Pass,
    FatalFailure,
}

pub mod matcher {
    use super::TestResult;

    pub fn assert_eq<U: std::cmp::PartialEq>(v1: U, v2: U) -> TestResult {
        return if v1 == v2 {
            TestResult::Pass
        } else {
            TestResult::FatalFailure
        };
    }

    pub fn assert_true(v: bool) -> TestResult {
        return if v {
            TestResult::Pass
        } else {
            TestResult::FatalFailure
        };
    }

    pub fn assert_false(v: bool) -> TestResult {
        return if !v {
            TestResult::Pass
        } else {
            TestResult::FatalFailure
        };
    }
}

pub mod test {
    use super::TestResult;

    pub type TestFunction<F> = dyn Fn(&F) -> TestResult;

    #[derive(Clone)]
    pub struct TestCase<F: 'static> {
        pub name: &'static str,
        pub test: &'static TestFunction<F>,
    }

    #[derive(Clone)]
    pub struct TestFixture<F: 'static> {
        pub name: &'static str,
        pub tests: &'static [TestCase<F>],
    }

    #[derive(Clone)]
    pub struct FrameWorkFixture<F: 'static> {
        pub test_fixture: TestFixture<F>,
    }

    pub trait FrameWorkTrait<F: 'static> {
        fn get_fixture(&self) -> &F;
        fn get_framework_fixture(&self) -> &FrameWorkFixture<F>;

        fn run_all_tests(&mut self) {
            self.run_fixture_tests();
        }

        fn fixture_header(&self) {
            println!(
                "[----------] {} tests from {}",
                self.get_framework_fixture().test_fixture.tests.len(),
                self.get_framework_fixture().test_fixture.name,
            );
        }

        fn setup(&mut self) {}

        fn test_header(&self, test_name: &str) {
            println!(
                "[ RUN      ] {}.{}",
                self.get_framework_fixture().test_fixture.name,
                test_name
            );
        }

        fn run_fixture_tests(&mut self) {
            self.fixture_header();
            self.setup();

            for testcase in self.get_framework_fixture().test_fixture.tests.iter() {
                self.test_header(&testcase.name);
                let result: TestResult = self.run_test(testcase);
                self.test_footer(&testcase.name, result);
            }

            self.teardown();
            self.fixture_footer();
        }

        fn run_test(&self, testcase: &TestCase<F>) -> TestResult {
            return (testcase.test)(self.get_fixture());
        }

        fn test_footer(&self, test_name: &str, result: TestResult) {
            match result {
                TestResult::Pass => {
                    println!(
                        "[       OK ] {}.{}",
                        self.get_framework_fixture().test_fixture.name,
                        test_name
                    );
                }
                _ => {
                    println!(
                        "[  FAILED  ] {}.{}",
                        self.get_framework_fixture().test_fixture.name,
                        test_name
                    );
                }
            }
        }

        fn teardown(&mut self) {}

        fn fixture_footer(&self) {
            println!(
                "[----------] {} tests from {}",
                self.get_framework_fixture().test_fixture.tests.len(),
                self.get_framework_fixture().test_fixture.name,
            );
        }
    }
}

pub mod parameterizedtest {
    use super::TestResult;

    pub type TestFunction<F, D> = dyn Fn(&F, &D) -> TestResult;

    #[derive(Clone)]
    pub struct TestCase<F: 'static, D: 'static> {
        pub name: &'static str,
        pub test: &'static TestFunction<F, D>,
    }

    unsafe impl<F, D> std::marker::Sync for TestCase<F, D> {}

    #[derive(Clone)]
    pub struct TestFixture<F: 'static, D: 'static> {
        pub name: &'static str,
        pub tests: &'static [TestCase<F, D>],
    }

    #[derive(Clone)]
    pub struct DataSetFixture<D: 'static> {
        pub name: &'static str,
        pub dataset: &'static [D],
    }

    #[derive(Clone)]
    pub struct FrameWorkFixture<F: 'static, D: 'static> {
        pub test_fixture: TestFixture<F, D>,
        pub dataset_fixture: DataSetFixture<D>,
    }

    pub trait FrameWorkTrait<F: 'static, D: 'static> {
        fn get_fixture(&self) -> &F;
        fn get_framework_fixture(&self) -> &FrameWorkFixture<F, D>;

        fn run_all_tests(&mut self) {
            self.run_fixture_tests();
        }

        fn fixture_header(&self) {
            let parameterized_test_count: usize =
                self.get_framework_fixture().test_fixture.tests.len()
                    * self.get_framework_fixture().dataset_fixture.dataset.len();
            println!(
                "[----------] {} tests from {}/{}",
                parameterized_test_count,
                self.get_framework_fixture().dataset_fixture.name,
                self.get_framework_fixture().test_fixture.name,
            );
        }

        fn setup(&mut self, _param: &D) {}

        fn run_fixture_tests(&mut self) {
            self.fixture_header();
            for testcase in self.get_framework_fixture().test_fixture.tests.iter() {
                self.run_parameterized_tests(testcase);
            }
            self.fixture_footer();
        }

        fn run_parameterized_tests(&mut self, testcase: &TestCase<F, D>) {
            for (index, param) in self
                .get_framework_fixture()
                .dataset_fixture
                .dataset
                .iter()
                .enumerate()
            {
                self.test_header(&testcase.name, index);

                self.setup(param);
                let result: TestResult = self.run_test(testcase, param);
                self.teardown(param);

                self.test_footer(&testcase.name, index, result);
            }
        }

        fn run_test(&self, testcase: &TestCase<F, D>, param: &D) -> TestResult {
            return (testcase.test)(self.get_fixture(), param);
        }

        fn test_header(&self, test_name: &str, test_index: usize) {
            println!(
                "[ RUN      ] {}/{}.{}/{}",
                self.get_framework_fixture().dataset_fixture.name,
                self.get_framework_fixture().test_fixture.name,
                test_name,
                test_index
            );
        }

        fn test_footer(&self, test_name: &str, test_index: usize, result: TestResult) {
            match result {
                TestResult::Pass => {
                    println!(
                        "[       OK ] {}/{}.{}/{}",
                        self.get_framework_fixture().dataset_fixture.name,
                        self.get_framework_fixture().test_fixture.name,
                        test_name,
                        test_index
                    );
                }
                _ => {
                    println!(
                        "[  FAILED  ] {}/{}.{}/{}",
                        self.get_framework_fixture().dataset_fixture.name,
                        self.get_framework_fixture().test_fixture.name,
                        test_name,
                        test_index
                    );
                }
            }
        }

        fn teardown(&mut self, _param: &D) {}

        fn fixture_footer(&self) {
            let parameterized_test_count: usize =
                self.get_framework_fixture().test_fixture.tests.len()
                    * self.get_framework_fixture().dataset_fixture.dataset.len();
            println!(
                "[----------] {} tests from {}/{}",
                parameterized_test_count,
                self.get_framework_fixture().dataset_fixture.name,
                self.get_framework_fixture().test_fixture.name,
            );
        }
    }
}

#[derive(PartialEq)]
pub enum TestResult {
    Pass,
    FatalFailure,
}

pub mod matcher {
    extern crate chrono;
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

    pub fn assert_death<F>(func: F) -> TestResult
    where
        F: FnMut() + std::marker::Send + 'static,
    {
        use std::thread::Builder;

        let builder: Builder = std::thread::Builder::new().name("assert_death".into());
        let dead_thread: std::io::Result<std::thread::JoinHandle<_>> = builder.spawn(func);
        if dead_thread.is_err() {
            return TestResult::FatalFailure;
        }
        let result = dead_thread.unwrap().join();
        match result {
            Err(_) => return TestResult::Pass,
            Ok(_) => return TestResult::FatalFailure,
        }
    }

    pub fn assert_death_or_timeout<F>(timeout: chrono::Duration, mut func: F) -> TestResult
    where
        F: FnMut() + std::marker::Send + 'static,
    {
        extern crate timer;
        use std::os::unix::thread::JoinHandleExt;

        // Create a thread to be killed (by itself or signal)
        let builder: std::thread::Builder =
            std::thread::Builder::new().name("assert_death_or_timeout".into());

        let dead_thread: std::io::Result<std::thread::JoinHandle<_>> = builder.spawn(move || {
            func();
        });
        if dead_thread.is_err() {
            // Failed to create a new thread
            return TestResult::FatalFailure;
        }
        let dead_thread = dead_thread.unwrap();
        let threadid: libc::pthread_t = dead_thread.as_pthread_t();

        // Create a timer that shall kill the previously created thread after timeout
        let timer = timer::Timer::new();
        let (tx, rx) = std::sync::mpsc::channel();
        let _guard = timer.schedule_with_delay(timeout, move || {
            // Execution of thread has taken too long, terminate the thread
            let result: libc::c_int = unsafe { libc::pthread_kill(threadid, libc::SIGTERM) };

            let _ignored: Result<_, _> = tx.send(result);
        });

        // Wait the thread to finish

        // This kills the (parent) process, if the thread were terminated with ANY signal in release mode
        // Works as expected in debug mode
        let kill_result = rx.recv();
        match kill_result
        {
            Err(_) => return TestResult::FatalFailure,
            Ok(status) => match status
            {
                // Killed the thread
                0 => return TestResult::Pass,
                // Thread already dead?
                libc::ESRCH => return TestResult::Pass,
                // Unexpected errors
                libc::EINVAL => return TestResult::FatalFailure,
                _ =>
                {
                    // Failed to kill thread, try to join it
                    let _join_result = dead_thread.join();
                    return TestResult::FatalFailure;
                },
            },
        }
    }

    pub fn expect_fatal_failure<F>(mut func: F) -> TestResult
    where
        F: FnMut() -> TestResult + std::marker::Send + 'static,
    {
        let result: TestResult = func();
        match result {
            TestResult::FatalFailure => return TestResult::Pass,
            _ => return TestResult::FatalFailure,
        }
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

        fn run_all_tests(&mut self) -> bool {
            return self.run_fixture_tests();
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

        fn run_fixture_tests(&mut self) -> bool {
            self.fixture_header();
            self.setup();

            let mut cumulative_result: bool = true;
            for testcase in self.get_framework_fixture().test_fixture.tests.iter() {
                self.test_header(&testcase.name);
                let result: TestResult = self.run_test(testcase);
                cumulative_result = cumulative_result && self.test_footer(&testcase.name, result);
            }

            self.teardown();
            self.fixture_footer();

            return cumulative_result;
        }

        fn run_test(&self, testcase: &TestCase<F>) -> TestResult {
            return (testcase.test)(self.get_fixture());
        }

        fn test_footer(&self, test_name: &str, result: TestResult) -> bool {
            match result {
                TestResult::Pass => {
                    println!(
                        "[       OK ] {}.{}",
                        self.get_framework_fixture().test_fixture.name,
                        test_name
                    );
                    return true;
                }
                _ => {
                    println!(
                        "[  FAILED  ] {}.{}",
                        self.get_framework_fixture().test_fixture.name,
                        test_name
                    );
                    return false;
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

        fn run_all_tests(&mut self) -> bool {
            return self.run_fixture_tests();
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

        fn run_fixture_tests(&mut self) -> bool {
            let mut cumulative_result: bool = true;
            self.fixture_header();
            for testcase in self.get_framework_fixture().test_fixture.tests.iter() {
                cumulative_result = cumulative_result && self.run_parameterized_tests(testcase);
            }
            self.fixture_footer();
            return cumulative_result;
        }

        fn run_parameterized_tests(&mut self, testcase: &TestCase<F, D>) -> bool {
            let mut cumulative_result: bool = true;
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

                cumulative_result =
                    cumulative_result && self.test_footer(&testcase.name, index, result);
            }
            return cumulative_result;
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

        fn test_footer(&self, test_name: &str, test_index: usize, result: TestResult) -> bool {
            match result {
                TestResult::Pass => {
                    println!(
                        "[       OK ] {}/{}.{}/{}",
                        self.get_framework_fixture().dataset_fixture.name,
                        self.get_framework_fixture().test_fixture.name,
                        test_name,
                        test_index
                    );
                    return true;
                }
                _ => {
                    println!(
                        "[  FAILED  ] {}/{}.{}/{}",
                        self.get_framework_fixture().dataset_fixture.name,
                        self.get_framework_fixture().test_fixture.name,
                        test_name,
                        test_index
                    );
                    return false;
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

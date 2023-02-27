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

    pub fn assert_death_or_timeout<F>(timeout: std::time::Duration, mut func: F) -> TestResult
    where
        F: FnMut() + std::marker::Send + 'static,
    {
        let result: TestResult = std::thread::scope(|_scope| {
            use std::thread::Builder;

            let mut system_threadid: libc::c_int = 0;

            // Create a thread to be killed (by itself or signal)
            let builder: Builder =
                std::thread::Builder::new().name("assert_death_or_timeout".into());
            let dead_thread: std::io::Result<std::thread::JoinHandle<_>> =
                builder.spawn(move || {
                    let id: libc::pthread_t = unsafe { libc::pthread_self() };
                    eprintln!("let system_threadid = {id}");
                    eprintln!(
                        "let log2(system_threadid) = {}",
                        bs::binary_search::power::utility::log2(id as usize).unwrap()
                    );
                    eprintln!("c_int::MAX = {}", libc::c_int::MAX);
                    eprintln!(
                        "log2(c_int::MAX) = {}",
                        bs::binary_search::power::utility::log2(libc::c_int::MAX as usize).unwrap()
                    );
                    system_threadid = id.try_into().unwrap();
                    assert!(system_threadid != 0);
                    (func)();
                });
            if dead_thread.is_err() {
                // Failed to create a new thread
                return TestResult::FatalFailure;
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));

            // Create a timer that shall kill the previously created thread
            eprintln!("get system_threadid = {system_threadid}");
            assert!(system_threadid != 0);
            let mut timerid: libc::timer_t = std::ptr::null_mut();
            {
                let mut sev: libc::sigevent =
                    unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
                sev.sigev_value.sival_ptr = timerid;
                sev.sigev_signo = libc::SIGKILL;
                sev.sigev_notify = libc::SIGEV_SIGNAL;
                sev.sigev_notify_thread_id = system_threadid;
                let result: libc::c_int = unsafe {
                    libc::timer_create(
                        libc::CLOCK_REALTIME,
                        std::ptr::addr_of_mut!(sev),
                        std::ptr::addr_of_mut!(timerid),
                    )
                };
                if result != 0 {
                    // Failed to create a timer
                    return TestResult::FatalFailure;
                }
            }

            // start the timer
            {
                let timeout_duration: libc::itimerspec = libc::itimerspec {
                    it_interval: libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    },
                    it_value: libc::timespec {
                        tv_sec: timeout.as_secs() as i64,
                        tv_nsec: timeout.subsec_nanos() as i64,
                    },
                };
                let result: libc::c_int = unsafe {
                    libc::timer_settime(
                        timerid,
                        0,
                        std::ptr::addr_of!(timeout_duration),
                        std::ptr::null_mut(),
                    )
                };
                if result != 0 {
                    // Failed to start the timer
                    return TestResult::FatalFailure;
                }
            }

            // Wait the thread to finish
            let join_result = dead_thread.unwrap().join();

            // Delete the timer
            {
                let result: libc::c_int = unsafe { libc::timer_delete(timerid) };
                if result != 0 {
                    // Failed to destroy the timer
                    return TestResult::FatalFailure;
                }
            }

            if join_result.is_err() {
                // Thead were killed
                return TestResult::Pass;
            }

            return TestResult::FatalFailure;
        });

        return result;
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

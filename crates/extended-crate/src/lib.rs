#![no_std]
#![cfg_attr(test, no_main)]
#![cfg_attr(test, feature(custom_test_frameworks))] // test setup: enable custom test frameworks
#![cfg_attr(test, test_runner(kunit::runner))] // test setup: use the custom test runner only in test mode
#![cfg_attr(test, reexport_test_harness_main = "test_main")] // test setup: rename the test harness entry point

#[cfg(test)]
kunit::klib!("extended-crate", klib_config = &TEST_CONFIG);

#[cfg(test)]
const TEST_CONFIG: kunit::KlibConfig = kunit::KlibConfigBuilder::new_default()
    .before_tests(|| init())
    .after_tests(|| teardown())
    .build();

#[allow(dead_code)]
fn init() {
    // some init here
}

#[allow(dead_code)]
fn teardown() {
    // some teardown here
}

#[cfg(test)]
mod tests {
    use kunit::kunit;

    #[kunit]
    fn trivial_basic_crate_assertion() {
        assert_eq!(2 + 2, 4);
    }
}

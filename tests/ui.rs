//! Tests for emitted diagnostics

#[cfg(all(test, not(miri)))]
mod ui {
    #[test]
    fn fail() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/fail/**/*.rs");
    }

    #[test]
    fn pass() {
        let t = trybuild::TestCases::new();
        t.pass("tests/ui/pass/**/*.rs");
    }
}

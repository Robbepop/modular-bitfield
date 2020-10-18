/// Repeats the given closure several times.
///
/// We do this in order to measure benchmarks that require at least some
/// amount of nanoseconds to run through.
pub fn repeat<F>(mut f: F)
where
    F: FnMut(),
{
    for _ in 0..10 {
        f();
    }
}

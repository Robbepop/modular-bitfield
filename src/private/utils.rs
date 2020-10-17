/// Helper struct to convert primitives and enum discriminants.
#[doc(hidden)]
pub struct Bits<T>(pub T);

impl<T> Bits<T> {
    /// Returns the raw underlying representation.
    #[inline(always)]
    pub fn into_raw(self) -> T {
        self.0
    }
}

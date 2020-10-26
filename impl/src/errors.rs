/// Creates a [`syn::Error`] with the format message and infers the
/// [`Span`](`proc_macro2::Span`) using [`ToTokens`](`quote::ToTokens`).
///
/// # Parameters
///
/// - The first argument must implement [`quote::ToTokens`] in order to
///   infer a [`Span`](`proc_macro2::Span`).
/// - The second argument is a format string.
/// - The rest are format string arguments.
///
/// # Note
///
/// On stable Rust this might yield higher quality error span information to the user
/// than [`format_err`].
/// - Source:
/// [`syn::Error::new_spanned`](https://docs.rs/syn/1.0.33/syn/struct.Error.html#method.new_spanned)
/// - Tracking issue: [`#54725`](https://github.com/rust-lang/rust/issues/54725)
macro_rules! format_err_spanned {
    ( $tokens:expr, $($msg:tt)* ) => {{
        ::syn::Error::new_spanned(
            &$tokens,
            format_args!($($msg)*)
        )
    }}
}

/// Creates a [`syn::Error`] with the format message and infers the
/// [`Span`](`proc_macro2::Span`) using [`Spanned`](`syn::spanned::Spanned`).
///
/// # Parameters
///
/// - The first argument must be a type that implements [`syn::spanned::Spanned`].
/// - The second argument is a format string.
/// - The rest are format string arguments.
///
/// # Note
///
/// On stable Rust this might yield worse error span information to the user
/// than [`format_err_spanned`].
/// - Source:
/// [`syn::Error::new_spanned`](https://docs.rs/syn/1.0.33/syn/struct.Error.html#method.new_spanned)
/// - Tracking issue: [`#54725`](https://github.com/rust-lang/rust/issues/54725)
macro_rules! format_err {
    ( $spanned:expr, $($msg:tt)* ) => {{
        ::syn::Error::new(
            <_ as ::syn::spanned::Spanned>::span(&$spanned),
            format_args!($($msg)*)
        )
    }}
}

pub trait CombineError {
    /// Combines `self` with the given `another` error and returns back combined `self`.
    fn into_combine(self, another: syn::Error) -> Self;
}

impl CombineError for syn::Error {
    fn into_combine(mut self, another: syn::Error) -> Self {
        self.combine(another);
        self
    }
}

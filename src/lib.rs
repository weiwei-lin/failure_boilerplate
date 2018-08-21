#[macro_export]
macro_rules! failure_boilerplate {
    ($error:ident, $error_kind:ty) => {
        #[derive(Debug)]
        pub struct $error {
            inner: ::failure::Context<$error_kind>
        }

        impl ::failure::Fail for $error {
            fn cause(&self) -> Option<&::failure::Fail> { self.inner.cause() }
            fn backtrace(&self) -> Option<&::failure::Backtrace> { self.inner.backtrace() }
        }
        impl fmt::Display for $error {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> fmt::Result { ::std::fmt::Display::fmt(&self.inner, f) }
        }
        impl $error {
            pub fn kind(&self) -> $error_kind { *self.inner.get_context() }
        }
        impl From<$error_kind> for $error {
            fn from(kind: $error_kind) -> $error { $error { inner: ::failure::Context::new(kind) } }
        }
        impl From<::failure::Context<$error_kind>> for $error {
            fn from(inner: ::failure::Context<$error_kind>) -> $error { $error { inner: inner } }
        }
    };
}

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
        impl ::std::fmt::Display for $error {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { ::std::fmt::Display::fmt(&self.inner, f) }
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

#[macro_export]
macro_rules! impl_serialize_for_error {
    ($error:ident) => {
        impl ::serde::Serialize for $error {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: ::serde::Serializer
            {
                use ::serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("code", &self.kind())?;
                map.serialize_entry("msg", &format!("{}", self))?;
                map.serialize_entry("causes", &::failure::Fail::iter_causes(self).map(|e| format!("{}", e)).collect::<Vec<_>>())?;
                map.end()
            }
        }
    }
}

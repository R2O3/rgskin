#[macro_export]
/// define an "enum" (struct) of a singular type
/// ```
/// def_const_type_enum! (pub enumName => Type {
///     field1 => value,
///     field2 => value,
/// });
macro_rules! def_const_type_enum {
    ($vis:vis $name:ident => $ty:ty {
        $($variant:ident => $val:expr),+
        $(,)?
    }) => {
        #[non_exhaustive]
        $vis struct $name;

        impl $name {
            $(
                pub const $variant: $ty = $val;
            )+

            #[allow(dead_code)]
            pub const VARIANTS: &'static [$ty] = &[$(Self::$variant),+];
        }
    };
}
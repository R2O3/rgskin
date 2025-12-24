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

            #[allow(dead_code)]
            pub fn iter_mapped<U, F>(mapper: F) -> impl Iterator<Item = U>
            where
                F: Fn($ty) -> U,
                $ty: Copy,
            {
                Self::VARIANTS.iter().copied().map(mapper)
            }
        }
    };
}
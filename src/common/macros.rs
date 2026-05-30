#[macro_export]
/// define an "enum" (struct) of a singular type
/// ```rust
/// def_const_type_enum! (pub enumName => StringPattern {
///     // No attributes
///     field1 => "value1",
///     
///     // With attributes
///     field2 => "value2" [AssetAttribute::Texture],
///     field3 => "value3" [AssetAttribute::Animatable(AnimationSpriteType::Single), AssetAttribute::Texture],
/// });
/// ```
macro_rules! def_const_type_enum {
    ($vis:vis $name:ident => StringPattern {
        $(
            $variant:ident => $val:literal $([ $($attr:expr),* $(,)? ])?
        ),+
        $(,)?
    }) => {
        #[non_exhaustive]
        $vis struct $name;

        impl $name {
            $(pub const $variant: $crate::StringPattern = $crate::StringPattern::new($val);)+

            pub const VARIANTS: &'static [$crate::StringPattern] = &[$(Self::$variant),+];

            pub fn iter_mapped<U, F>(f: F) -> impl Iterator<Item = U>
            where
                F: Fn($crate::StringPattern) -> U,
            {
                Self::VARIANTS.iter().cloned().map(f)
            }

            pub fn expand_all<'a>(params: &'a [(&'a str, &'a str)]) -> impl Iterator<Item = String> + 'a {
                Self::VARIANTS.iter().map(move |p| p.expand(params))
            }

            pub fn attributes(pattern: &$crate::StringPattern) -> &'static [crate::common::skin::AssetAttribute] {
                match pattern.raw() {
                    $(
                        $val => &[ $($( $attr ),*)? ],
                    )+
                    _ => &[],
                }
            }
        }
    };
}
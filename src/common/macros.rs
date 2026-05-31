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
        }

        impl $crate::ConstTypeEnum for $name {  // re-exported at crate root
            type Attribute = $crate::common::skin::AssetAttribute;

            const VARIANTS: &'static [$crate::StringPattern] = &[$(Self::$variant),+];

            fn attributes(
                pattern: &$crate::StringPattern,
            ) -> &'static [Self::Attribute] {
                match pattern.raw() {
                    $($val => &[ $($( $attr ),*)? ],)+
                    _ => &[],
                }
            }
        }
    };
}

pub trait ConstTypeEnum {
    type Attribute: 'static;

    const VARIANTS: &'static [crate::StringPattern];

    fn attributes(pattern: &crate::StringPattern) -> &'static [Self::Attribute];

    fn has_attribute(pattern: &crate::StringPattern, attr: &Self::Attribute) -> bool
    where
        Self::Attribute: PartialEq,
    {
        Self::attributes(pattern).contains(attr)
    }

    fn find_attribute<F>(
        pattern: &crate::StringPattern,
        pred: F,
    ) -> Option<&'static Self::Attribute>
    where
        F: Fn(&Self::Attribute) -> bool,
    {
        Self::attributes(pattern).iter().find(|a| pred(a))
    }

    fn iter_mapped<U, F>(f: F) -> impl Iterator<Item = U>
    where
        F: Fn(crate::StringPattern) -> U,
    {
        Self::VARIANTS.iter().cloned().map(f)
    }

    fn expand_all<'a>(
        params: &'a [(&'a str, &'a str)],
    ) -> impl Iterator<Item = String> + 'a {
        Self::VARIANTS.iter().map(move |p| p.expand(params))
    }
}
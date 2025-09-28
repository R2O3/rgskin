#[inline]
pub fn string_iter_as_str<'a, I>(iter: I) -> Vec<&'a str>
where
    I: IntoIterator<Item = &'a String>,
{
    iter.into_iter().map(|s| s.as_str()).collect()
}
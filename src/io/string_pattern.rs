use std::borrow::Cow;

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct StringPattern(pub Cow<'static, str>);

impl StringPattern {
    pub const fn new(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }

    pub fn expand(&self, params: &[(&str, &str)]) -> String {
        if !self.0.contains('{') {
            return self.0.to_string();
        }
        let mut out = self.0.to_string();
        for (key, val) in params {
            out = out.replace(&format!("{{{key}}}"), val);
        }
        out
    }

    pub fn is_literal(&self) -> bool {
        !self.0.contains('{')
    }

    pub fn raw(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for StringPattern {
    type Target = str;
    fn deref(&self) -> &str { &self.0 }
}

impl std::fmt::Display for StringPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&'static str> for StringPattern {
    fn from(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }
}

impl From<String> for StringPattern {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl From<&String> for StringPattern {
    fn from(s: &String) -> Self {
        Self(Cow::Owned(s.clone()))
    }
}

impl From<StringPattern> for String {
    fn from(p: StringPattern) -> Self { p.0.to_string() }
}

impl From<&StringPattern> for String {
    fn from(p: &StringPattern) -> Self { p.0.to_string() }
}

impl AsRef<str> for StringPattern {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<[u8]> for StringPattern {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl std::borrow::Borrow<str> for StringPattern {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl StringPattern {
    pub fn matches_path(&self, path: &str) -> bool {
        if match_template(&self.0, path) {
            return true;
        }

        if let Some(idx) = path.rfind('@') {
            let base = &path[..idx];
            if match_template(&self.0, base) {
                return true;
            }
        }

        false
    }

    pub fn get_sheet_size(&self) -> Option<(u32, u32)> {
        self.0.split("@").nth(1)
              .and_then(|suffix| suffix.strip_suffix("x"))
              .and_then(|num_str| num_str.parse::<u32>().ok())
              .map(|n| (n, n))
    }
}

// TODO: maybe use limited regex in the future? needs to be fast enough though

fn match_template(pattern: &str, input: &str) -> bool {
    let mut p = pattern;
    let mut i = input;

    loop {
        match p.find('{') {
            None => return p.eq_ignore_ascii_case(i),

            Some(brace_start) => {
                let prefix = &p[..brace_start];
                if !i.get(..prefix.len())
                       .map_or(false, |s| s.eq_ignore_ascii_case(prefix))
                {
                    return false;
                }
                i = &i[prefix.len()..];

                let close = match p[brace_start..].find('}') {
                    Some(j) => brace_start + j + 1,
                    None => return false, // malformed pattern
                };
                p = &p[close..];

                let next_brace = p.find('{').unwrap_or(p.len());
                let delimiter  = &p[..next_brace];

                let param_end = if delimiter.is_empty() {
                    i.len()
                } else {
                    match i.to_lowercase().find(&delimiter.to_lowercase()) {
                        Some(pos) => pos,
                        None => return false,
                    }
                };

                let param_val = &i[..param_end];
                if param_val.is_empty() || param_val.contains('/') {
                    return false;
                }

                i = &i[param_end..];
            }
        }
    }
}
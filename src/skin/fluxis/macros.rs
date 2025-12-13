
#[macro_export]
macro_rules! define_stage_overrides {
    ($(($field:ident, $key:expr)),* $(,)?) => {
        #[derive(Clone, Debug, Default)]
        pub struct StageOverrides {
            $(pub $field: String,)*
        }

        impl StageOverrides {
            pub fn serialize(&self) -> Vec<(&'static str, &str)> {
                let mut entries = Vec::new();
                $(
                    if !self.$field.is_empty() {
                        entries.push(($key, self.$field.as_str()));
                    }
                )*
                entries
            }

            pub fn set_field(&mut self, key: &str, value: String) -> bool {
                match key {
                    $($key => { self.$field = value; true })*
                    _ => false,
                }
            }

            pub fn get_fields(&self) -> Vec<(&'static str, &String)> {
                vec![
                    $(($key, &self.$field),)*
                ]
            }
        }
    };
}

#[macro_export]
macro_rules! define_keymode {
    ($(($field:ident, $element:expr, $element_type:expr, $suffix:expr)),* $(,)?) => {
        pub struct Keymodes;

        impl Keymodes {
            pub fn iter<F>(keymode: &Keymode, mut f: F)
            where
                F: FnMut(&Vec<String>, &'static str, &'static str, &'static str),
            {
                $(f(&keymode.$field, $element, $element_type, $suffix);)*
            }

            pub fn iter_mut<F>(keymode: &mut Keymode, mut f: F)
            where
                F: FnMut(&mut Vec<String>, &'static str, &'static str, &'static str),
            {
                $(f(&mut keymode.$field, $element, $element_type, $suffix);)*
            }

            pub fn get_field_mut<'a>(keymode: &'a mut Keymode, element: &'a str, element_type: &'a str, suffix: &'a str) 
                -> Option<&'a mut Vec<String>> 
            {
                match (element, element_type, suffix) {
                    $(
                        ($element, $element_type, $suffix) => Some(&mut keymode.$field),
                    )*
                    _ => None,
                }
            }
        }
    };
}

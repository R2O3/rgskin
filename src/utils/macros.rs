#[macro_export]
macro_rules! numeric_enum {
    (
        $vis:vis enum $name:ident {
            $($variant:ident = $value:expr),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $vis enum $name {
            $($variant),*
        }

        impl $name {
            pub const fn value(&self) -> i32 {
                match self {
                    $(Self::$variant => $value),*
                }
            }

            pub const fn as_i32(&self) -> i32 {
                self.value()
            }

            pub const fn as_u32(&self) -> u32 {
                self.value() as u32
            }

            pub const fn as_i64(&self) -> i64 {
                self.value() as i64
            }

            pub const fn as_u64(&self) -> u64 {
                self.value() as u64
            }

            pub const fn as_f32(&self) -> f32 {
                self.value() as f32
            }

            pub const fn as_f64(&self) -> f64 {
                self.value() as f64
            }

            pub const fn as_usize(&self) -> usize {
                self.value() as usize
            }

            pub const fn as_isize(&self) -> isize {
                self.value() as isize
            }
        }
    };
}

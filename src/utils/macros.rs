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

        impl num_traits::ToPrimitive for $name {
            fn to_i64(&self) -> Option<i64> {
                Some(self.value() as i64)
            }

            fn to_u64(&self) -> Option<u64> {
                Some(self.value() as u64)
            }

            fn to_i32(&self) -> Option<i32> {
                Some(self.value())
            }

            fn to_u32(&self) -> Option<u32> {
                Some(self.value() as u32)
            }

            fn to_f32(&self) -> Option<f32> {
                Some(self.value() as f32)
            }

            fn to_f64(&self) -> Option<f64> {
                Some(self.value() as f64)
            }

            fn to_isize(&self) -> Option<isize> {
                Some(self.value() as isize)
            }

            fn to_usize(&self) -> Option<usize> {
                Some(self.value() as usize)
            }
        }

        impl From<$name> for i32 {
            fn from(val: $name) -> Self {
                val.value()
            }
        }

        impl From<$name> for i64 {
            fn from(val: $name) -> Self {
                val.value() as i64
            }
        }

        impl From<$name> for u32 {
            fn from(val: $name) -> Self {
                val.value() as u32
            }
        }

        impl From<$name> for u64 {
            fn from(val: $name) -> Self {
                val.value() as u64
            }
        }

        impl From<$name> for f32 {
            fn from(val: $name) -> Self {
                val.value() as f32
            }
        }

        impl From<$name> for f64 {
            fn from(val: $name) -> Self {
                val.value() as f64
            }
        }

        impl From<$name> for isize {
            fn from(val: $name) -> Self {
                val.value() as isize
            }
        }

        impl From<$name> for usize {
            fn from(val: $name) -> Self {
                val.value() as usize
            }
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(n: i32) -> Result<Self, Self::Error> {
                match n {
                    $($value => Ok(Self::$variant),)*
                    _ => Err(()),
                }
            }
        }

        impl std::convert::TryFrom<i64> for $name {
            type Error = ();

            fn try_from(n: i64) -> Result<Self, Self::Error> {
                Self::try_from(n as i32)
            }
        }

        impl std::convert::TryFrom<u32> for $name {
            type Error = ();

            fn try_from(n: u32) -> Result<Self, Self::Error> {
                Self::try_from(n as i32)
            }
        }

        impl std::convert::TryFrom<u64> for $name {
            type Error = ();

            fn try_from(n: u64) -> Result<Self, Self::Error> {
                Self::try_from(n as i32)
            }
        }
    };
}

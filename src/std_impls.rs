use crate::{Debug, Formatter, Result};

macro_rules! std_debug {
    ($($t:ty),+) => {
        $(
            impl Debug for $t {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    f.write_debug(self)
                }
            }
        )+
    };
}

std_debug! {
    bool,
    char,
    f32,
    f64,
    isize,
    i8,
    i16,
    i32,
    i64,
    i128,
    usize,
    u8,
    u16,
    u32,
    u64,
    u128,
    String,
    str,
    ()
}

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => (
        impl<$($name:Debug),+> Debug for ($($name,)+) where last_type!($($name,)+): ?Sized {
            #[allow(non_snake_case, unused_assignments)]
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                let mut builder = f.debug_tuple("");
                let ($(ref $name,)+) = *self;
                $(
                    builder.field(&$name);
                )+

                builder.finish()
            }
        }
        peel! { $($name,)+ }
    )
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }

macro_rules! fmt_refs {
    ($($tr:ident),*) => {
        $(
        impl<T: ?Sized + $tr> $tr for &T {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result { $tr::fmt(&**self, f) }
        }
        impl<T: ?Sized + $tr> $tr for &mut T {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result { $tr::fmt(&**self, f) }
        }
        )*
    }
}

fmt_refs! { Debug /*, Display, Octal, Binary, LowerHex, UpperHex, LowerExp, UpperExp */ }

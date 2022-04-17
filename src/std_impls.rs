use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex, TryLockError},
    path::{Path, PathBuf}
};

use crate::{Debug, Formatter};

macro_rules! std_debug {
    ($($t:ty),+) => {
        $(
            impl Debug for $t {
                fn fmt(&self, f: &mut Formatter) {
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
    (),
    Path,
    PathBuf
}

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => (
        impl<$($name:Debug),+> Debug for ($($name,)+) where last_type!($($name,)+): ?Sized {
            #[allow(non_snake_case, unused_assignments)]
            fn fmt(&self, f: &mut Formatter) {
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
            fn fmt(&self, f: &mut Formatter) { $tr::fmt(&**self, f) }
        }
        impl<T: ?Sized + $tr> $tr for &mut T {
            fn fmt(&self, f: &mut Formatter) { $tr::fmt(&**self, f) }
        }
        )*
    }
}

fmt_refs! { Debug /*, Display, Octal, Binary, LowerHex, UpperHex, LowerExp, UpperExp */ }

impl<T: ?Sized + Debug> Debug for Box<T> {
    fn fmt(&self, f: &mut Formatter) {
        Debug::fmt(&**self, f)
    }
}

impl<T: Debug, const N: usize> Debug for [T; N] {
    fn fmt(&self, f: &mut Formatter) {
        Debug::fmt(&self[..], f)
    }
}

impl<T: ?Sized + Debug> Debug for Arc<T> {
    fn fmt(&self, f: &mut Formatter) {
        Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized + Debug> Debug for Rc<T> {
    fn fmt(&self, f: &mut Formatter) {
        Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized> Debug for *const T {
    fn fmt(&self, f: &mut Formatter) {
        f.write_debug(self)
    }
}

impl<T: ?Sized> Debug for *mut T {
    fn fmt(&self, f: &mut Formatter) {
        Debug::fmt(&(self as *const _), f)
    }
}

macro_rules! list_like {
    ($($t:ty),+) => {
        $(
            impl<T: Debug> Debug for $t {
                fn fmt(&self, f: &mut Formatter) {
                    f.debug_list().entries(self.iter()).finish()
                }
            }
        )+
    };
}

list_like! {
    [T], Vec<T>, VecDeque<T>, LinkedList<T>, BinaryHeap<T>
}

impl<K, V, S> Debug for HashMap<K, V, S>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter) {
        f.debug_map().entries(self.iter()).finish()
    }
}
impl<K: Debug, V: Debug> Debug for BTreeMap<K, V> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_map().entries(self.iter()).finish()
    }
}
impl<T, S> Debug for HashSet<T, S>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) {
        f.debug_set().entries(self.iter()).finish()
    }
}
impl<T> Debug for BTreeSet<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) {
        f.debug_set().entries(self.iter()).finish()
    }
}
impl<T: Debug> Debug for Option<T> {
    fn fmt(&self, f: &mut Formatter) {
        match self {
            Some(v) => f.debug_tuple("Some").field(v).finish(),
            None => f.debug_tuple("None").finish(),
        }
    }
}

impl<T: ?Sized> Debug for std::marker::PhantomData<T> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("PhantomData").finish()
    }
}

impl<T: Copy + Debug> Debug for std::cell::Cell<T> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("Cell").field("value", &self.get()).finish()
    }
}

impl<T: ?Sized + Debug> Debug for std::cell::RefCell<T> {
    fn fmt(&self, f: &mut Formatter) {
        match self.try_borrow() {
            Ok(borrow) => f.debug_struct("RefCell").field("value", &borrow).finish(),
            Err(_) => {
                // The RefCell is mutably borrowed so we can't look at its value
                // here. Show a placeholder instead.
                struct BorrowedPlaceholder;

                impl Debug for BorrowedPlaceholder {
                    fn fmt(&self, f: &mut Formatter) {
                        f.write_display("<borrowed>")
                    }
                }

                f.debug_struct("RefCell")
                    .field("value", &BorrowedPlaceholder)
                    .finish()
            }
        }
    }
}

impl<T: ?Sized + Debug> Debug for std::cell::Ref<'_, T> {
    fn fmt(&self, f: &mut Formatter) {
        Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized + Debug> Debug for std::cell::RefMut<'_, T> {
    fn fmt(&self, f: &mut Formatter) {
        Debug::fmt(&*(self.deref()), f)
    }
}

impl<T: ?Sized> Debug for std::cell::UnsafeCell<T> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("UnsafeCell").finish_non_exhaustive()
    }
}

impl Debug for std::fmt::Arguments<'_> {
    fn fmt(&self, f: &mut Formatter) {
        f.write_debug(self)
    }
}

impl<T: ?Sized + Debug> Debug for Mutex<T> {
    fn fmt(&self, f: &mut Formatter) {
        let mut d = f.debug_struct("Mutex");
        match self.try_lock() {
            Ok(guard) => {
                d.field("data", &&*guard);
            }
            Err(TryLockError::Poisoned(err)) => {
                d.field("data", &&**err.get_ref());
            }
            Err(TryLockError::WouldBlock) => {
                struct LockedPlaceholder;
                impl Debug for LockedPlaceholder {
                    fn fmt(&self, f: &mut Formatter) {
                        f.write_debug(&format_args!("<locked>"))
                    }
                }
                d.field("data", &LockedPlaceholder);
            }
        }
        d.field("poisoned", &self.is_poisoned());
        d.finish_non_exhaustive()
    }
}

// TODO: Tests

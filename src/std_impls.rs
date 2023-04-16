use std::{
    alloc::Layout,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    num::IntErrorKind,
    ops::Deref,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, Mutex, TryLockError},
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
        Debug::fmt(self.deref(), f)
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
                        f.write_debug(format_args!("<locked>"))
                    }
                }
                d.field("data", &LockedPlaceholder);
            }
        }
        d.field("poisoned", &self.is_poisoned());
        d.finish_non_exhaustive()
    }
}

impl Debug for Layout {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("Layout")
            .field("size", &self.size())
            .field("align", &self.align())
            .finish()
    }
}

impl<B> Debug for std::borrow::Cow<'_, B>
where
    B: Debug + ToOwned + ?Sized,
    <B as ToOwned>::Owned: Debug,
{
    fn fmt(&self, f: &mut Formatter) {
        match *self {
            Self::Borrowed(ref b) => Debug::fmt(b, f),
            Self::Owned(ref o) => Debug::fmt(o, f),
        }
    }
}

impl Debug for std::num::ParseIntError {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("ParseIntError")
            .field("kind", &self.kind())
            .finish()
    }
}

impl Debug for IntErrorKind {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_tuple(match self {
            Self::Empty => "Empty",
            Self::InvalidDigit => "InvalidDigit",
            Self::PosOverflow => "PosOverflow",
            Self::NegOverflow => "NegOverflow",
            Self::Zero => "Zero",
            _ => "Unknown",
        })
        .finish()
    }
}

impl Debug for std::time::Duration {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("Duration")
            .field("secs", &self.as_secs())
            .field("nanos", &self.subsec_nanos())
            .finish()
    }
}

impl Debug for Box<dyn std::error::Error + '_> {
    fn fmt(&self, f: &mut Formatter) {
        // TODO: Is this good
        f.write_debug(&**self)
    }
}

impl Debug for Box<dyn std::error::Error + Send + Sync + '_> {
    fn fmt(&self, f: &mut Formatter) {
        // TODO: Is this good
        f.write_debug(&**self)
    }
}

impl Debug for std::io::Error {
    fn fmt(&self, f: &mut Formatter) {
        Debug::fmt(&self.kind(), f) // TODO: This isn't ideal
    }
}
impl Debug for std::io::ErrorKind {
    fn fmt(&self, f: &mut Formatter) {
        // Quite alot of these are nighty only
        let kind = match self {
            Self::NotFound => "NotFound",
            Self::PermissionDenied => "PermissionDenied",
            Self::ConnectionRefused => "ConnectionRefused",
            Self::ConnectionReset => "ConnectionReset",
            // Self::HostUnreachable => "HostUnreachable",
            // Self::NetworkUnreachable => "NetworkUnreachable",
            Self::ConnectionAborted => "ConnectionAborted",
            Self::NotConnected => "NotConnected",
            Self::AddrInUse => "AddrInUse",
            Self::AddrNotAvailable => "AddrNotAvailable",
            // Self::NetworkDown => "NetworkDown",
            Self::BrokenPipe => "BrokenPipe",
            Self::AlreadyExists => "AlreadyExists",
            Self::WouldBlock => "WouldBlock",
            // Self::NotADirectory => "NotADirectory",
            // Self::IsADirectory => "IsADirectory",
            // Self::DirectoryNotEmpty => "DirectoryNotEmpty",
            // Self::ReadOnlyFilesystem => "ReadOnlyFilesystem",
            // Self::FilesystemLoop => "FilesystemLoop",
            // Self::StaleNetworkFileHandle => "StaleNetworkFileHandle",
            Self::InvalidInput => "InvalidInput",
            Self::InvalidData => "InvalidData",
            Self::TimedOut => "TimedOut",
            Self::WriteZero => "WriteZero",
            // Self::StorageFull => "StorageFull",
            // Self::NotSeekable => "NotSeekable",
            // Self::FilesystemQuotaExceeded => "FilesystemQuotaExceeded",
            // Self::FileTooLarge => "FileTooLarge",
            // Self::ResourceBusy => "ResourceBusy",
            // Self::ExecutableFileBusy => "ExecutableFileBusy",
            // Self::Deadlock => "Deadlock",
            // Self::CrossesDevices => "CrossesDevices",
            // Self::TooManyLinks => "TooManyLinks",
            // Self::InvalidFilename => "InvalidFilename",
            // Self::ArgumentListTooLong => "ArgumentListTooLong",
            Self::Interrupted => "Interrupted",
            Self::Unsupported => "Unsupported",
            Self::UnexpectedEof => "UnexpectedEof",
            Self::OutOfMemory => "OutOfMemory",
            Self::Other => "Other",
            // Self::Uncategorized => "Uncategorized",
            _ => "???",
        };
        f.write_display(kind)
    }
}

impl Debug for std::num::ParseFloatError {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("ParseFloatError").finish() // TODO: This has a kind, but
                                                   // it's secret.
    }
}

impl<T: Debug> Debug for std::ops::Range<T> {
    fn fmt(&self, f: &mut Formatter) {
        Debug::fmt(&self.start, f);
        f.word("..");
        Debug::fmt(&self.end, f);
    }
}
impl<T: Debug> Debug for std::ops::RangeFrom<T> {
    fn fmt(&self, f: &mut Formatter) {
        Debug::fmt(&self.start, f);
        f.word("..");
    }
}
impl Debug for std::ops::RangeFull {
    fn fmt(&self, f: &mut Formatter) {
        f.word("..")
    }
}
impl<T: Debug> Debug for std::ops::RangeInclusive<T> {
    fn fmt(&self, f: &mut Formatter) {
        Debug::fmt(&self.start(), f);
        f.word("..=");
        Debug::fmt(&self.end(), f);
    }
}
impl<T: Debug> Debug for std::ops::RangeTo<T> {
    fn fmt(&self, f: &mut Formatter) {
        f.word("..");
        Debug::fmt(&self.end, f);
    }
}
impl<T: Debug> Debug for std::ops::RangeToInclusive<T> {
    fn fmt(&self, f: &mut Formatter) {
        f.word("..=");
        Debug::fmt(&self.end, f);
    }
}

// TODO: Tests

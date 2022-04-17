#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

// From rust-lang/rust/library
pub mod builders;
mod formatter;
mod macros;
mod std_impls;

// From dtolnay/prettyplease/
mod algorithm;
mod convenience;
mod ring;

mod gen_impls;
mod hand_impls;

// Target line width.
const MARGIN: isize = 89;

// Number of spaces increment at each level of block indentation.
const INDENT: isize = 4;

// Every line is allowed at least this much space, even if highly indented.
const MIN_SPACE: isize = 60;

pub use algorithm::Formatter;
pub use debug3_derive::Debug;

/// Pretty printing of a type
///
/// This is essensialy the same as [`std::fmt::Debug`], but with a different
/// [`Formatter`] type.
///
/// Generally speaking, you should just `derive` a `Debug` implementation.
///
/// The output will always be pretty-printed.
///
/// This trait can be used with `#[derive]` if all fields implement `Debug`
///
/// # Stability
///
/// Derived `Debug` formats are not stable, and so may change with future
/// versions. Additionally, `Debug` implementations of types provided by the
/// standard library (`libstd`, `libcore`, `liballoc`, etc.) are not stable, and
/// may also change with future versions.
///
/// # Examples
///
/// Deriving an implementation:
///
/// ```
/// use debug3::{pprint, Debug};
///
/// #[derive(Debug)]
/// struct Dish {
///     name: &'static str,
///     price: i32,
/// }
///
/// let eggs = Dish {
///     name: "eggs",
///     price: 5,
/// };
/// assert_eq!(pprint(eggs), "Dish { name: \"eggs\", price: 5 }");
/// // This is long enough to spill
/// let green_eggs_and_ham = Dish {
///     name: "green eggs and ham",
///     price: 8,
/// };
/// assert_eq!(
///     pprint(green_eggs_and_ham),
///     "\
/// Dish {
///     name: \"green eggs and ham\",
///     price: 8,
/// }"
/// );
/// ```
///
/// There are a number of helper methods on the [`Formatter`] struct to help you
/// with manual implementations, such as [`debug_struct`].
///
/// [`debug_struct`]: Formatter::debug_struct
///
/// Types must use the standard suite of debug representations
/// provided by the `Formatter` trait (`debug_struct`, `debug_tuple`,
/// `debut_list`, `debug_set`, `debug_map`) as their is no way to do something
/// totaly custom by manually writing an arbitrary representation to the
/// [`Formatter`].
///
/// ```rust
/// use debug3::{pprint, Debug, Formatter};
///
/// struct Point {
///     theta: f64,
///     r: f64,
/// }
///
/// impl Debug for Point {
///     fn fmt(&self, f: &mut Formatter) {
///         f.debug_struct("Point")
///             .field("x", &(self.theta.cos() * self.r))
///             .field("y", &(self.theta.sin() * self.r))
///             .finish()
///     }
/// }
///
/// let origin = Point { theta: 0.0, r: 0.0 };
/// assert_eq!(pprint(origin), "Point { x: 0.0, y: 0.0 }");
///
/// struct ListMap<K, V>(Vec<(K, V)>);
///
/// impl<K: Debug, V: Debug> Debug for ListMap<K, V> {
///     fn fmt(&self, f: &mut Formatter) {
///         let mut m = f.debug_map();
///         for (k, v) in &self.0 {
///             m.key(k);
///             m.value(v);
///         }
///         m.finish();
///     }
/// }
///
/// let small_map = ListMap(vec![(1, 2), (3, 4), (5, 6)]);
/// assert_eq!(pprint(small_map), "{1: 2, 3: 4, 5: 6}");
///
/// let large_map = ListMap(vec![
///     (4, vec![2, 2]),
///     (6, vec![2, 3]),
///     (8, vec![2, 2, 2]),
///     (9, vec![3, 3]),
///     (10, vec![2, 5]),
///     (12, vec![2, 2, 3]),
///     (14, vec![2, 7]),
///     (15, vec![3, 5]),
///     (16, vec![2, 2, 2, 2]),
///     (18, vec![2, 3, 3]),
///     (20, vec![2, 2, 5]),
/// ]);
///
/// assert_eq!(
///     pprint(large_map),
///     "\
/// {
///     4: [2, 2],
///     6: [2, 3],
///     8: [2, 2, 2],
///     9: [3, 3],
///     10: [2, 5],
///     12: [2, 2, 3],
///     14: [2, 7],
///     15: [3, 5],
///     16: [2, 2, 2, 2],
///     18: [2, 3, 3],
///     20: [2, 2, 5],
/// }"
/// );
/// ```
pub trait Debug {
    /// Formats the value using the given formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{pprint, Debug, Formatter};
    ///
    /// struct Position {
    ///     longitude: f64,
    ///     latitude: f64,
    /// }
    ///
    /// impl Debug for Position {
    ///     fn fmt(&self, f: &mut Formatter) {
    ///         f.debug_tuple("")
    ///             .field(&self.longitude)
    ///             .field(&self.latitude)
    ///             .finish()
    ///     }
    /// }
    ///
    /// let position = Position {
    ///     longitude: 1.987,
    ///     latitude: 2.983,
    /// };
    /// assert_eq!(pprint(position), "(1.987, 2.983)");
    /// ```
    fn fmt(&self, f: &mut Formatter);
}

/// Pretty prit a value to a string
///
/// This is the main entry point for non-debuging use. For Debugging, see the
/// [`crate::dbg`] macro.
///
/// Note that this takes ownership of `x`, but becasuse `impl<T: + Debug> Debug
/// for &T`, you can still borrow.
///
/// Eg:
///
/// ```rust,compile_fail
/// use debug3::pprint;
/// let mut x = vec![1, 2, 3];
/// assert_eq!(pprint(x), "[1, 2, 3]");
/// x.push(4);
/// assert_eq!(pprint(x), "[1, 2, 3, 4]");
/// ```
///
/// should be:
///
/// ```rust
/// use debug3::pprint;
/// let mut x = vec![1, 2, 3];
/// assert_eq!(pprint(&x), "[1, 2, 3]");
/// x.push(4);
/// assert_eq!(pprint(&x), "[1, 2, 3, 4]");
/// ```
///
///
///
/// # Examples
///
/// ```
/// use debug3::pprint;
///
/// let x = vec![1, 2, 3];
/// assert_eq!(pprint(x), "[1, 2, 3]");
/// let y = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12], vec![13, 14, 15], vec![16, 17, 18], vec![19, 20, 21], vec![21, 22, 23]];
/// assert_eq!(pprint(y), "\
/// [
///     [1, 2, 3],
///     [4, 5, 6],
///     [7, 8, 9],
///     [10, 11, 12],
///     [13, 14, 15],
///     [16, 17, 18],
///     [19, 20, 21],
///     [21, 22, 23],
/// ]"
/// );
pub fn pprint<T: Debug>(x: T) -> String {
    let mut f = Formatter::new();
    x.fmt(&mut f);
    f.eof()
}

debug3
======

A space effiecent replacement for [`std::fmt::Debug`]

## The Pitch

Lets say you have data that looks like this:

```rust
let complex_structure = vec![
        vec![None, Some(2)],
        vec![Some(2), None],
        vec![Some(4), Some(777)],
        vec![None, Some(2)],
        vec![Some(2), None],
        vec![None, None, None, None, None],
];
```

And you want to format it as a string. You could use `format!("{:?}", complex_structure)` and get something like

```rust,ignore
[[None, Some(2)], [Some(2), None], [Some(4), Some(777)], [None, Some(2)], [Some(2), None], [None, None, None, None, None]]
```

But this is too much one one line, and is hard to read. And it gets worse for larger structures.

Fortunaly theirs an alternative `format!("{:#?}", complex_structure)`, which gives 

```rust,ignore
[
    [
        None,
        Some(
            2,
        ),
    ],
    [
        Some(
            2,
        ),
        None,
    ],
    [
        Some(
            4,
        ),
        Some(
            777,
        ),
    ],
    [
        None,
        Some(
            2,
        ),
    ],
    [
        Some(
            2,
        ),
        None,
    ],
    [
        None,
        None,
        None,
        None,
        None,
    ],
]
```

This has the oposite problem, where it uses too much space, even when the code could be packed denser.

`debug3` provides a third option that is denser than `:#?` but more readable than `:?`. If you use `debug3::pprint(complex_structure)`, you get

```rust,ignore
[
    [None, Some(2)],
    [Some(2), None],
    [Some(4), Some(777)],
    [None, Some(2)],
    [Some(2), None],
    [None, None, None, None, None],
]
```

## Overview

The main entrypoint is the [`Debug`] trait, which is the equivalent to [`std::fmt::Debug`], and has a similar API.

This can be either `#[derive]`d, or implemented manually.

```rust
use debug3::{Debug, Formatter, pprint};

#[derive(Debug)]
struct MyStruct {
    a: i32,
    b: i32,
}

struct AnotherStruct {
    a: i32,
    b: i32,
}

impl Debug for AnotherStruct {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("AnotherStruct")
            .field("a", &self.a)
            .field("b", &self.b)
            .finish()
    }
}

assert_eq!(pprint(MyStruct { a: 1, b: 2 }), "MyStruct { a: 1, b: 2 }");
assert_eq!(pprint(AnotherStruct { a: 1, b: 2 }), "AnotherStruct { a: 1, b: 2 }");
```

Once your type implements [`Debug`], you have several options to format it

- [`pprint`]: Convert it to a [`String`]
- [`dbg`]: Print it to stderr
<!-- TODO: More -->

## Comparison to `std::fmt::Debug`:

While the main advantage of `debug3` is the superior output quality, it has several drawbacks compared to [`std::fmt`] that you should know

1. Commonness: Virtually every type in Rust implements [`std::fmt::Debug`],
   vitrualy no types outside of [`std`] implement [`debug3::Debug`][`Debug`].
2. Availibility: [`std::fmt`] is also availible as [`core::fmt`], which allows
   you to use it in `no_std` environments. `debug3` requires several allocated
   data structures, so cannot support these environments.
3. Versitility: [`std::fmt::Formatter`] has many more API's for implementing
   [`std::fmt::Debug`]. In order to achive nice formatting, we cannot accept
   arbitrary strings, but must have items in the form of Structs, Tuples, Maps,
   Lists and Sets.
4. Configurabiliy: We dont suport stuff like `format!("{:x?}, 1)` to configure
   how numbers are printed.
5. Ease of use: We don't have a macro like [`std::format`] to easily create a
   string from several elements which implement [`Debug`]

## Prior Art

`debug3` would not be possible without all of the following excelent work

- [`core::fmt`](https://github.com/rust-lang/rust/tree/master/library/core/src/fmt) - The public API of [`Formatter`] and `[builders]`
- [`prettyplease`](https://github.com/dtolnay/prettyplease/) - Most of the prety printing algorithm is lifted directly from this crate
- [`custom_debug`](https://github.com/panicbit/custom_debug) - The derive macro for [`Debug`] is based on this crate.
- python's [`pprint`](https://docs.python.org/3/library/pprint.html) - Inspiration for this type of formatting for debug output.


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

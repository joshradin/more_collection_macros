#![deny(rustdoc::broken_intra_doc_links)]
#![deny(unused)]

//! # More Collection Macros
//!
//! This library provides a set of useful macros for creating [std collections], similar to the
//! default `vec!` macro
//!
//! [std collections]: std::collections
//!
//! # Comprehension
//!
//! These macros enable list/map comprehension similar to what's present in python.
//! For example, the following python code
//! ```python
//! list = [x*x for x in range(0,10)]
//! dict = {x : x*x for x in range(0,10)}
//! ```
//! is equivalent to the following rust code
//! ```rust
//! # #[macro_use] extern crate more_collection_macros;
//!
//! let list = list![x*x; x in 0..10];
//! let dict = map!{x => x*x; x in 0..10};
//! ```
//!
//! # Namespace-like
//!
//! With maps, you can also use identifiers as keys, which are then converted to `&'static str` keys.
//! ```
//! # #[macro_use] extern crate more_collection_macros;
//! map!{
//!     field1: 0,
//!     field2: 1
//! };
//! ```
//! You can't repeat fields with those notation. The following will give a _runtime_ error.
//! ```should_panic
//! # #[macro_use] extern crate more_collection_macros;
//! map!{
//!     field1: 0,
//!     field1: 1
//! };
//! ```


/// Create a map. Can either provide a function-like dictionary comprehension, or provide a list of
/// tuples containing a key and value to create a map.
///
/// Can use identifiers as keys, which are converted to [Strings](std::string::String)
///
/// # Example
/// ```
/// # #[macro_use] extern crate more_collection_macros;
/// # use std::collections::HashMap;
/// let dict: HashMap<&str, u32> = map! {
///     field: 15,
///     field2: 18
/// };
///
/// # assert_eq!(dict["field"], 15);
/// # assert_eq!(dict["field2"], 18);
///
/// let map = map! [("field", 15), ("field2", 18)];
/// assert_eq!(dict, map);
/// ```
#[macro_export]
macro_rules! map {
    ($( ($key:expr, $value:expr) ),* $(,)?) => {
        [$(($key, $value)),*].into_iter().collect::<std::collections::HashMap<_, _>>()
    };
    (
        $($key:ident : $value:expr),*
        $(,)?
    ) => {

        $crate::map!(
            std::collections::HashMap::<&'static str, _>::new(),
            $(
                stringify!($key) => $value
            ),*
        )

    };
    (
        $map:expr,
        $($key:expr => $value:expr),*
        $(,)?
    ) => {
        {
            use $crate::prelude::*;
            let mut map: HashMap<_, _> = $map;

            $(
            {
                let key = $key;
                if !map.contains_key(&key) {
                    let value = $value;
                    map.insert(key, value);

                } else {
                    panic!("{key} already in dict")
                }
            }
            )*

            map
        }
    };
    (
        $($key:expr => $value:expr),*
        $(,)?
    ) => {
        $crate::map!(std::collections::HashMap::new(), $($key => $value),*)
    };
    ($key:expr => $value:expr; $v:ident in $range:expr) => {
        $crate::iter![($key, $value); $v in $range].collect::<std::collections::HashMap<_, _>>()
    };
    ($key:expr => $value:expr; $v:ident in $range:expr; if $cond:expr) => {
        $crate::iter![($key, $value); $v in $range; if $cond].collect::<std::collections::HashMap<_, _>>()
    };
}


/// Creates a basic iter using the python-style list comprehension syntax
#[macro_export]
macro_rules! iter {
    ($ex:expr; $v:ident in $range:expr) => {
        IntoIterator::into_iter($range).map(|$v| $ex)
    };
    ($ex:expr; $v:ident in $range:expr; if $cond:expr) => {
        IntoIterator::into_iter($range)
            .filter(|$v| $cond)
            .map(|$v| $ex)
    };
}


/// A wrapper around [iter](crate::iter) that collects into a vector
#[macro_export]
macro_rules! list {
    ($($tok:tt)*) => {
        std::iter::Iterator::collect::<std::vec::Vec<_>>($crate::iter!($($tok)*))
    };
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn map() {
        let map = map! {
            "key1".to_string() => "value1",
            "key2".to_string() => "value2",
        };

        let expected = vec![(format!("key1"), "value1"), (format!("key2"), "value2")]
            .into_iter()
            .collect::<HashMap<_, _>>();

        assert_eq!(map, expected);
    }

    #[test]
    fn basic_dict() {
        let dict: HashMap<_, &str> = map! {
            key1: "value1",
            key2: "value2"
        };

        let expected = map! {
            "key1" => "value1",
            "key2" => "value2",
        };

        assert_eq!(dict, expected);
    }

    #[test]
    fn alt_dict() {
        assert_eq!(
            map! { "a" => 1u32, "b" => 2u32},
            map! { a: 1u32, b: 2u32}
        )
    }

    #[test]
    #[should_panic]
    fn repeat_keys() {
        let _dict: HashMap<_, i32> = map! {
            key1: 0,
            key2: 1,
            key3: 4,
            key2: 4,
        };
    }

    #[test]
    fn iterators() {
        let iter: Vec<i32> = iter!(a*a; a in 0..=5).collect();
        assert_eq!(iter, vec![0, 1, 4, 9, 16, 25]);
        let iter: Vec<i32> = iter!(a*a; a in 0..=5; if a % 2 == 0).collect();
        assert_eq!(iter, vec![0, 4, 16]);
        let list = list!(a*a; a in 0..=5; if a % 2 == 0);
        assert_eq!(list, iter);
    }

    #[test]
    fn map_comprehension() {
        let map = map![i => i*i; i in 0..=5];
        let expected = map![
            0 => 0,
            1 => 1,
            2 => 4,
            3 => 9,
            4 => 16,
            5 => 25
        ];
        assert_eq!(map, expected);
        let map = map![i => i*i; i in 0..=5; if i %2 == 0];
        let expected = map![
            0 => 0,
            2 => 4,
            4 => 16,
        ];
        assert_eq!(map, expected);
    }
}

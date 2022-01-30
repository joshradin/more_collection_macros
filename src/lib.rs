use std::collections::HashMap;


pub type Dict<T> = HashMap<String, T>;

#[macro_export]
macro_rules! dict {
    (
        $($key:ident : $value:expr),*
        $(,)?
    ) => {

        $crate::map!(
            Dict::new(),
            $(
                stringify!($key).to_string() => $value
            ),*
        )

    };
    (
        $($key:ident => $value:expr),*
        $(,)?
    ) => {
        $crate::dict!($($key : $value),*)
    };
}

#[macro_export]
macro_rules! map {
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
}

#[macro_export]
macro_rules! iter {
    ($ex:expr; $v:ident in $range:expr) => {
        IntoIterator::into_iter($range)
                        .map(|$v| $ex)
    };
    ($ex:expr; $v:ident in $range:expr; if $cond:expr) => {
        IntoIterator::into_iter($range)
                        .filter(|$v| $cond)
                        .map(|$v| $ex)
    };
}

#[macro_export]
/// A wrapper around [iter](::iter) that collects into a vector
macro_rules! list {
    ($($tok:tt)*) => {
        std::iter::Iterator::collect::<Vec<_>>($crate::iter!($($tok)*))
    };
}

pub mod prelude {
    pub use std::collections::HashMap;
    pub use super::{Dict, dict, iter, map, list};
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map() {

        let map = map!{
            "key1".to_string() => "value1",
            "key2".to_string() => "value2",
        };

        let expected = vec![
            (format!("key1"), "value1"),
            (format!("key2"), "value2")
        ].into_iter()
            .collect::<HashMap<_, _>>();

        assert_eq!(map, expected);
    }

    #[test]
    fn basic_dict() {
        let dict: Dict<&str> = dict!{
            key1: "value1",
            key2: "value2"
        };

        let expected = map!{
            "key1".to_string() => "value1",
            "key2".to_string() => "value2",
        };

        assert_eq!(dict, expected);
    }

    #[test]
    fn alt_dict() {
        assert_eq!(
            dict! { a => 1u32, b => 2u32},
            dict! { a: 1u32, b: 2u32}
        )
    }

    #[test]
    #[should_panic]
    fn repeat_keys() {
        let _dict: Dict<i32> = dict!{
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
}


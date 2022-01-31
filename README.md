# More Collection Macros
By Joshua Radin

Repostiory

This library provides a set of useful macros for creating [std collections], similar to the
default `vec!` macro


# Comprehension

These macros enable list/map comprehension similar to what's present in python.
For example, the following python code
```
list = [x*x for x in range(0,10)]
dict = {x : x*x for x in range(0,10)}
```
is equivalent to the following rust code
```
let list = list![x*x; x in 0..10];
let dict = map!{x => x*x; x in 0..10};
```

# Namespace-like

With maps, you can also use identifiers as keys, which are then converted to `&'static str` keys.
```
map!{
    field1: 0,
    field2: 1
};
```
You can't repeat fields with this notation. The following will give a _runtime_ error.
```should_panic
map!{
    field1: 0,
    field1: 1
};
```


[std collections]: https://doc.rust-lang.org/std/collections/index.html
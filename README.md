[![crates.io](https://img.shields.io/crates/v/negative-type-bound.svg)](https://crates.io/crates/negative-type-bound)

**this crate uses feature negative_impls and auto_traits**

You may want to do something like
```rust
struct Foo<T>(T);

impl<T: From<U>, U> From<Foo<U>> for Foo<T> {
    fn from(v: Foo<U>) -> Self {
        Self(T::from(v.0))
    }
}
```
but it wont work because it conflicts with implementation `impl<T> From<T> for T {}`
provided by `core`.

so we have to bound `Foo<T> != Foo<U>`

this crate provides trait `NotEqual`

`(T, U): NotEqual` is equivalent to T != U

so this will work
```rust
struct Foo<T>(T);

impl<T: From<U>, U> From<Foo<U>> for Foo<T> where (Foo<T>, Foo<U>): NotEqual {
    fn from(v: Foo<U>) -> Self {
        Self(T::from(v.0))
    }
}
```
more simply...
```rust
struct Foo<T>(T);

impl<T: From<U>, U> From<Foo<U>> for Foo<T> where (T, U): NotEqual {
    fn from(v: Foo<U>) -> Self {
        Self(T::from(v.0))
    }
}
```

this crate also provides `Equal`
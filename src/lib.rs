#![feature(negative_impls, auto_traits)]

/// **this crate uses feature negative_impls and auto_traits**
///
/// You may want to do something like
/// ```ignore
/// struct Foo<T>(T);
///
/// impl<T: From<U>, U> From<Foo<U>> for Foo<T> {
///     fn from(v: Foo<U>) -> Self {
///         Self(T::from(v.0))
///     }
/// }
/// ```
/// but it wont work because it conflicts with implementation `impl<T> From<T> for T {}`
/// provided by `core`
///
/// so we have to bound `Foo<T> != Foo<U>`
///
/// this crate provides trait `NotEqual`
///
/// `(T, U): NotEqual` is equivalent to T != U
///
/// so this will work
/// ```ignore
/// struct Foo<T>(T);
///
/// impl<T: From<U>, U> From<Foo<U>> for Foo<T> where (Foo<T>, Foo<U>): NotEqual {
///     fn from(v: Foo<U>) -> Self {
///         Self(T::from(v.0))
///     }
/// }
/// ```
/// more simply...
/// ```ignore
/// struct Foo<T>(T);
///
/// impl<T: From<U>, U> From<Foo<U>> for Foo<T> where (T, U): NotEqual {
///     fn from(v: Foo<U>) -> Self {
///         Self(T::from(v.0))
///     }
/// }
/// ```
///
/// this crate also provides `Equal`


pub auto trait NotEqual {}

impl<T> !NotEqual for (T, T) {}

pub trait Equal {}

impl<T> Equal for (T, T) {}

#[cfg(test)]
mod tests {
    use crate::NotEqual;

    #[test]
    fn a() {
        struct Foo<T>(T);

        impl<T: From<U>, U> From<Foo<U>> for Foo<T> where (T, U): NotEqual {
            fn from(v: Foo<U>) -> Self {
                Self(T::from(v.0))
            }
        }
    }
}

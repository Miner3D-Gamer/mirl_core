// #[macro_export]
// /// Converts {K: V} into ([K; N], [V; N])
// macro_rules! kv {
//     ($($k:expr => $v:expr),* $(,)?) => {
//         ([$($k),*], [$($v),*])
//     };
// }

#[macro_export]
/// Usage: `impl_empty_trait!(std::sync::Send for Struct1, Struct2, Struct3)`
macro_rules! impl_empty_trait {
    ($name:ident for $($t:ty),* $(,)?) => {
        $(
            impl $name for $t {}
        )*
    };
}
#[macro_export]
/// Implement a trait for multiple types with an identical body.
///
/// # Usage
/// ```ignore
/// impl_trait!(MyTrait { fn foo(&self) -> u32 { 42 } } for Struct1, Struct2, Struct3);
/// ```
///
/// With generics on the trait:
/// ```ignore
/// impl_trait!(MyTrait<u32> [T: Clone] { ... } for Struct1, Struct2);
/// //                        ^^^^^^^^^
/// //                        optional where-clause bounds
/// ```
macro_rules! impl_trait {
    ($name:path { $($body:tt)* } for $($t:ty),+ $(,)?) => {
        impl_trait!(@impl $name { $($body)* } [] $($t),+);
    };

    // Internal: peel one type at a time
    (@impl $name:path { $($body:tt)* } [$($done:tt)*] $t:ty, $($rest:ty),+) => {
        impl_trait!(@impl $name { $($body)* } [
            $($done)*
            impl $name for $t { $($body)* }
        ] $($rest),+);
    };

    // Internal: last type
    (@impl $name:path { $($body:tt)* } [$($done:tt)*] $t:ty) => {
        $($done)*
        impl $name for $t { $($body)* }
    };
}

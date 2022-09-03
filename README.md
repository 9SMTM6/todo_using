# TODO-Using

A super small crate only exporting a single [macro-by-example](https://doc.rust-lang.org/stable/rust-by-example/macros.html#macro_rules) to "use" parameters.

## Yet another dependency?

If you want to avoid yet another dependency, and don't need a fancy API, you can also just copy paste:

```
#[macro_export]
macro_rules! todo_using {
    ([$($using_ident: ident),* $(,)?], $($message_arg:tt)+) => {
        {$(
            let __suppress_unused_warning = $using_ident;
        )*};
        core::todo!($($message_arg)+);
    };
    ($($using_ident: ident),* $(,)?) => {
        {$(
            let __suppress_unused_warning = $using_ident;
        )*};
        core::todo!();
    };
}
```

for the extended version just [look into the source code](`todo_using`).

## Purpose of this crate
For you as an enduser this crate mostly serves as
1. An easy way to use the feature without thinking about it
2. As a way to discover how to implement this
3. As a vehicle for a few tests that ensure things work as expected

I mostly wrote this though to serve as a playground for both macros-by-example and a few other features of the Rust ecosystem, like documentation. I don't expect any mayor issues to come up, and if that should happen I expect to fix them, but of course there is no guarantee of continued maintenance.
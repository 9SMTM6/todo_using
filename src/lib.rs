#![doc = include_str!("../README.md")]

#[doc(hidden)]
#[macro_export]
macro_rules! use_args {
    ($($using_ident: ident), *) => {
        {
            $(
                let __suppress_unused_warning = $using_ident;
            )*
        };
    };
}

/// A wrapper around [`core::todo!`]
/// that allows "using" variables to suppress 
/// `unused` warnings while mocking functions with [todo].
/// 
/// # Examples
/// 
/// Both kinds of arguments seperated:
/// ```should_panic
/// # use todo_using::*;
/// # let arg1 = 5;
/// # let arg2 = "some String";
/// # let format_arg = "IDK what to put here";
/// todo_using!(
///   using = [arg1, arg2],
///   unimplemented("Unimplemented Code {}", format_arg)
/// );
/// ```
/// Just special casing for formatting arguments to [core::todo]:
/// ```should_panic
/// # use todo_using::*;
/// # let arg1 = 5;
/// # let arg2 = "some String";
/// # let format_arg = "IDK what to put here";
/// todo_using!(
///   [arg1, arg2],
///   unimplemented("Unimplemented Code {}", format_arg)
/// );
/// ```
/// Just marking the using arguments:
/// ```should_panic
/// # use todo_using::*;
/// # let arg1 = 5;
/// # let arg2 = "some String";
/// # let format_arg = "IDK what to put here";
/// todo_using!(
///   using = [arg1, arg2],
///   "Unimplemented Code {}", format_arg
/// );
/// ```
/// No special casing for unused variables nor todo arguments:
/// ```should_panic
/// # use todo_using::*;
/// # let arg1 = 5;
/// # let arg2 = "some String";
/// # let format_arg = "IDK what to put here";
/// todo_using!(
///   [arg1, arg2],
///   "Unimplemented Code {}", format_arg
/// );
/// ```
/// Just passing unused variables:
/// ```should_panic
/// # use todo_using::*;
/// # let arg1 = 5;
/// # let arg2 = "some String";
/// # let format_arg = "IDK what to put here";
/// todo_using![arg1, arg2];
/// ```
/// Keeping syntax for unused variables:
/// ```should_panic
/// # use todo_using::*;
/// # let arg1 = 5;
/// # let arg2 = "some String";
/// # let format_arg = "IDK what to put here";
/// todo_using![[arg1, arg2]];
/// ```
#[macro_export]
macro_rules! todo_using {
    ($(using=)?[$($using_ident: ident),* $(,)?], unimplemented($($message_arg:tt)+)) => {
        $crate::use_args!($($using_ident), *);
        core::todo!($($message_arg)+);
    };
    ($(using=)?[$($using_ident: ident),* $(,)?], $($message_arg:tt)*) => {
        $crate::use_args!($($using_ident), *);
        core::todo!($($message_arg)*);
    };
    ($(using=)?[$($using_ident: ident),* $(,)?]) => {
        $crate::use_args!($($using_ident), *);
        core::todo!();
    };
    ($($using_ident: ident),* $(,)?) => {
        $crate::use_args!($($using_ident), *);
        core::todo!();
    };
}

#[cfg(test)]
mod tests {
    use std::{panic::UnwindSafe, fmt::Debug};

    use super::*;

    fn capture_panic_message<R: Debug>(panicking_closure: impl FnOnce() -> R + UnwindSafe) -> String {
        *std::panic::catch_unwind(panicking_closure).unwrap_err().downcast::<String>().unwrap()
    }

    #[test]
    fn panics_match() {
        #[allow(unused_variables)]
        let arg1 = 5;
        let arg2 = "some String";
        let format_arg = "IDK what to put here";
        assert_eq!(
            capture_panic_message(||{todo_using!(using = [arg1, arg2], unimplemented("Unimplemented Code {}", format_arg));}),
            capture_panic_message(||{todo_using!([arg1, arg2], "Unimplemented Code {}", format_arg);}),
        );
    }
}

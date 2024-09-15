/// A macro to simplify the initialization of static variables using `spin::Lazy`.
///
/// # Overview
///
/// In Rust, static variables that require complex initialization often use `spin::Lazy`.
/// This macro, `rt_init!`, provides a more convenient syntax for declaring and initializing
/// such static variables, making the code easier to read and maintain.
///
/// # Syntax
///
/// ```ignore
/// rt_init! {
///     [visibility] static NAME: TYPE = INITIALIZER;
///     ...
/// }
/// ```
///
/// - `[visibility]`: Optional visibility modifier, such as `pub` or `pub(crate)`.
/// - `NAME`: The name of the static variable.
/// - `TYPE`: The type of the static variable.
/// - `INITIALIZER`: An expression that initializes the static variable. It will be
///   evaluated the first time the variable is accessed.
///
/// # Examples
///
/// Basic usage with different types:
///
/// ```rust
/// use rt_init::rt_init;
///
/// rt_init! {
///     static STATIC1: Vec<u64> = vec![1, 2, 3];
///     static STATIC2: u64 = 42;
///     static STATIC3: String = "Hello, World!".to_string();
/// }
///
/// fn main() {
///     println!("STATIC1: {:?}", *STATIC1);
///     println!("STATIC2: {}", *STATIC2);
///     println!("STATIC3: {}", *STATIC3);
/// }
/// ```
///
/// Using different visibility modifiers:
///
/// ```rust
/// use rt_init::rt_init;
///
/// rt_init! {
///     pub static PUB_STATIC: u32 = 100;
///     pub(crate) static CRATE_STATIC: u32 = 200;
///     static PRIVATE_STATIC: u32 = 300;
/// }
///
/// fn main() {
///     println!("PUB_STATIC: {}", *PUB_STATIC);
///     println!("CRATE_STATIC: {}", *CRATE_STATIC);
///     println!("PRIVATE_STATIC: {}", *PRIVATE_STATIC);
/// }
/// ```
///
/// # Notes
///
/// This macro uses `spin::Lazy` internally, which provides a mechanism for lazy
/// initialization. The initialization expression is only evaluated when the static
/// is first accessed, which can help with performance and initialization order issues.
#[macro_export]
macro_rules! rt_init {
    ($($vis:vis static $name:ident: $type:ty = $init:expr;)+) => {
        $(
            $vis static $name: ::spin::Lazy<$type> = ::spin::Lazy::new(|| $init);
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_initialization() {
        rt_init! {
            static STATIC1: Vec<u64> = vec![1, 2, 3];
            static STATIC2: u64 = 42;
            static STATIC3: String = "Hello, World!".to_string();
        }

        assert_eq!(*STATIC1, vec![1, 2, 3]);
        assert_eq!(*STATIC2, 42);
        assert_eq!(STATIC3.as_str(), "Hello, World!");
    }

    #[test]
    fn test_visibility_modifiers() {
        rt_init! {
            pub static PUB_STATIC: u32 = 100;
            pub(crate) static CRATE_STATIC: u32 = 200;
        }

        assert_eq!(*PUB_STATIC, 100);
        assert_eq!(*CRATE_STATIC, 200);
    }

    #[test]
    fn test_complex_initialization() {
        rt_init! {
            static COMPLEX_STATIC: Vec<u64> = {
                let mut vec = Vec::new();
                for i in 0..5 {
                    vec.push(i * 2);
                }
                vec
            };
        }

        assert_eq!(*COMPLEX_STATIC, vec![0, 2, 4, 6, 8]);
    }
}

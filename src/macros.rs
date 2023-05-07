//! Macros used in the library to make life easier.

/// Set a value from an expression that returns a `Result`, return an error message if not `Ok`.
///
/// # Arguments
///
/// - `$var:ident` -- The variable we wish to extract the value from the function into.
/// - `$fun:expr` -- The function to run. This function _must_ return a `Result`.
/// - `$msg:literal` -- The clear-text error message to return if the `$fun` returns an error.
///
/// # Examples
///
/// ```ignore
/// fn meaning_of_life(guess: u8) -> Result<u8, Error> {
///     if guess == 42 {
///         Ok(guess)
///     } else {
///         Err("Wrong!".into())
///     }
/// }
///
/// // Take the return value of `meaning_of_life` and put it into `mol`.
/// // Return the message if unsuccessful.
/// let_ok!(mol, meaning_of_life(42), "Meaning of life not found.");
///
/// assert_eq!(mol, 42);
/// ```
#[macro_export]
macro_rules! let_ok {
    ($var:ident, $fun:expr, $msg:literal) => {
        let Ok($var) = $fun else {
                                                                                    return Err($msg)
                                                                                };
    };
}

/// Set a value from an expression that returns an `Option`, return an error message if not `Some`.
///
/// # Arguments
///
/// - `$var:ident` -- The variable we wish to extract the value from the function into.
/// - `$fun:expr` -- The function to run. This function _must_ return an `Option`.
/// - `$msg:literal` -- The clear-text error message to return if the `$fun` returns `None`.
///
/// # Examples
///
/// ```ignore
/// fn meaning_of_life(guess: u8) -> Option<u8> {
///    if guess == 42 {
///       Some(guess)
///   } else {
///      None
///  }
/// }
///
/// // Take the return value of `meaning_of_life` and put it into `mol`.
/// // Return the message if unsuccessful.
///
/// // This is OK (returns Some(42)), so the variable gets assigned.
/// let_some!(mol, meaning_of_life(42), "Meaning of life not found.");
/// assert_eq!(mol, 42);
///
/// // This is not OK (returns None), and will return the error message.
/// let_some!(mol, meaning_of_life(43), "Meaning of life not found.");
/// assert_err!(mol, "Meaning of life not found.");
/// ```
#[macro_export]
macro_rules! let_some {
    ($var:ident, $fun:expr, $msg:literal) => {
        let Some($var) = $fun else {
                                                                                    return Err($msg)
                                                                                };
    };
}

/// Creates a `pub const fn` that returns the indicated value from `self`.
///
/// This is used to create the `get` functions for the various fields in the atoms.
/// The function name is the same as the field name, and the return type is the same as the field type.
/// See the source for the definition of the struct.
///
/// # Arguments
///
/// - `$id:ident` -- The name of the function to create, as well as the name of the variable to get from `Self`.
/// - `$typ:ty` -- The type of the value to return, this is the same as the type of the variable in `Self`.
///
/// # Examples
///
/// ```ignore
/// struct Foo {
///    bar: u8,
/// }
///
/// impl Foo {
///   // Create a function called `bar` that returns the value of `self.bar`.
///  retval!(bar, u8);
/// }
///
/// let foo = Foo { bar: 42 };
/// assert_eq!(foo.bar(), 42);
/// ```
///
/// # Panics
///
/// None.
#[macro_export]
macro_rules! retval {
    ($id:ident, $typ:ty) => {
        /// Return the indicated value from `self`.
        #[must_use]
        pub const fn $id(&self) -> $typ {
            self.$id
        }
    };
}

/// Creates a `pub const fn` that returns a reference to the indicated value from `self`.
///
/// This is used to create the `get` functions for the various fields in the atoms.
/// The function name is the same as the field name, and the return type is the same as the field type.
/// See the source for the definition of the struct.
///
/// # Arguments
///
/// - `$id:ident` -- The name of the function to create, the name of the variable to get from `Self`.
/// - `$typ:ty` -- The type of the value to return, this is the same as the type of the variable in `Self`.
///
/// # Examples
///
/// ```ignore
/// struct Foo {
///   bar: u8,
/// }
///
/// impl Foo {
///  // Create a function called `bar` that returns a reference to the value of `self.bar`.
///  retref!(bar, u8);
/// }
///
/// let foo = Foo { bar: 42 };
/// assert_eq!(*foo.bar(), 42);
/// ```
#[macro_export]
macro_rules! retref {
    ($id:ident, $typ:ty) => {
        /// Returns a reference to the indicated value from `self`.
        #[must_use]
        pub const fn $id(&self) -> &$typ {
            &self.$id
        }
    };
}

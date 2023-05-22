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

/// Creates a function that returns a boolean indicating whether a specified flag is set
#[macro_export]
macro_rules! flag {
    ($flag:ident, $val:expr) => {
        /// Returns `true` if the indicated flag is set.
        pub fn $flag(&self) -> bool {
            self.header().flags_to_u32() & $val == $val
        }
    };
}

/// Creates a generic `parse` function for elements that have children.
///
/// This ensures that the function stays the same for all the atoms that have children,
/// and where the actual, needed parsing hasn't been implemented yet.
#[macro_export]
macro_rules! generic_parse_children {
    ($id:ident) => {
        /// Parse an atom from the file. This will skip over the data in the file. This is a generic function for now.
        ///
        /// # Arguments
        ///
        /// * `f` - The file to read from.
        /// * `header` - The header of the atom.
        ///
        /// # Returns
        ///
        /// * `Self` - The parsed atom, which in this case basically means we move the offset ahead.
        pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
            let children: Vec<Atom> = Atom::parse_children(f).unwrap_or_default();
            log::trace!("$id::parse() -- children = {children:?}");

            Ok(Self { header, children })
        }
    };
}

/// Creates a generic `parse` function for elements that have no children.
///
/// This ensures that the function stays the same for all the atoms that have no children,
/// and where the actual, needed parsing hasn't been implemented yet.
#[macro_export]
macro_rules! generic_parse {
    ($id:ident) => {
        /// Parse an atom from the file. This will skip over the data in the file. This is a generic function for now.
        ///
        /// # Arguments
        ///
        /// * `f` - The file to read from.
        /// * `header` - The header of the atom.
        ///
        /// # Returns
        ///
        /// * `Result<Self, &'static str>` - The parsed atom, which in this case basically means we move the offset ahead.
        ///
        /// # Errors
        ///
        /// * `Unable to seek file.` - If the file cannot be seeked.
        pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
            header.parse_version(f);
            header.parse_flags(f);

            let curr_offset = f.offset();
            $crate::let_ok!(
                _offset,
                f.seek(curr_offset + header.data_size),
                "Unable to seek file."
            );
            f.offset_inc(header.data_size);
            log::trace!("$id::parse() -- header = {header:?}");

            Ok(Self { header })
        }
    };
}

/// Reads a versioned variable. If the version is 0, the _short_ value is used, otherwise the _long_ value is used.
///
/// # Arguments
///
/// - `$var:ident` -- The name of the variable in which to store the value.
/// - `$t:ty` -- The type of the variable to be returned, eg. `u64`.
/// - `$s:expr` -- The function to use to read the short (version == 0) value, eg. `f.read_u32()`.
/// - `$l:expr` -- The function to use to read the long (version != 0) value, eg. `f.read_u64()`.
/// - `$header:ident` -- The header of the atom. This is used to get the version information.
///
/// **NOTE:** The `$s` and `$l` expressions _must_ return a `Result` or an `Option`.
/// Also, be mindful of the type into which you are casting, as loss of data or truncation may occur.
///
/// # Examples
///
/// ```ignore
/// read_version!(sample_size, u32, f.read_u16(), f.read_u32(), header);
/// read_version!(offset, i32, f.read_i16(), f.read_u16(), header);
/// ```
#[macro_export]
macro_rules! read_version {
    ($var:ident, $t:ty, $s:expr, $l:expr, $header:ident) => {
        let $var = if $header.version == Some(0) {
            $s.unwrap_or_default() as $t
        } else {
            $l.unwrap_or_default() as $t
        };
    };
}

/// Returns a vector with the indicated type, and the indicated number of elements.
#[macro_export]
macro_rules! vec_with_type {
    ($t:ty, $mp4:expr) => {
        let mut v = Vec<$t>::new();
        for atom in $mp4.atoms() {
            if let Atom::$t(a) = atom {
                v.push(a.clone());
            }
        }
    };
}

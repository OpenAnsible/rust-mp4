//! A versioned integer - either 32 or 64 bits, depending on the version of the atom.

/// Sets the width of the field depending on the version of the atom.
/// This is used for parsing atoms with different versions.
/// Version 0 is 32 bit, whereas version 1 is 64 bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum VersionInt {
    Version0(i32) = 0,
    Version1(i64) = 1,
}

impl VersionInt {
    /// Returns the value of the versioned integer.
    #[must_use]
    pub const fn value(&self) -> i64 {
        match self {
            Self::Version0(v) => *v as i64,
            Self::Version1(v) => *v,
        }
    }
}

impl Default for VersionInt {
    /// Returns the default value of the versioned integer - `Version0(0)`.
    fn default() -> Self {
        Self::Version0(0)
    }
}

impl std::fmt::Display for VersionInt {
    /// Formats the versioned integer as a string. This is the same as calling `value()` and formatting that.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
///
mod tests {
    use super::VersionInt;

    #[test]
    fn test_enum_construction() {
        let v0 = VersionInt::Version0(42);
        assert!(matches!(v0, VersionInt::Version0(_)));
        assert_eq!(v0, VersionInt::Version0(42));

        let v1 = VersionInt::Version1(42);
        assert!(matches!(v1, VersionInt::Version1(_)));
        assert_eq!(v1, VersionInt::Version1(42));
    }

    #[test]
    fn test_enum_matching() {
        let v0 = VersionInt::Version0(42);
        match v0 {
            VersionInt::Version0(n) => assert_eq!(n, 42),
            _ => panic!("Expected Version0"),
        }

        let v1 = VersionInt::Version1(42);
        match v1 {
            VersionInt::Version1(n) => assert_eq!(n, 42),
            _ => panic!("Expected Version1"),
        }
    }
}

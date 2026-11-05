//! Packages repo VCS migration tooling
//!
//! SigmaOS component inspired by Arch Linux's `svntogit-packages` repository.
//! Designed for component isolation, loose coupling, and secure defaults.

use std::fmt;

/// Errors produced by this component. Fail-fast: errors are returned, not panicked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SvntogitPackagesError {
    /// The requested operation is not valid in the current state.
    InvalidState,
    /// Input failed validation.
    InvalidInput(String),
}

impl fmt::Display for SvntogitPackagesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidState => write!(f, "invalid state"),
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
        }
    }
}

impl std::error::Error for SvntogitPackagesError {}

/// Core component type. Holds configuration; performs no I/O by default.
#[derive(Debug, Clone, Default)]
pub struct SvntogitPackages {
    name: String,
}

impl SvntogitPackages {
    /// Create a new component instance. Fails fast on empty names.
    pub fn new(name: impl Into<String>) -> Result<Self, SvntogitPackagesError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(SvntogitPackagesError::InvalidInput("name must not be empty".into()));
        }
        Ok(Self { name })
    }

    /// Component identifier.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Perform the component's primary action. Returns a deterministic result.
    pub fn run(&self) -> Result<String, SvntogitPackagesError> {
        if self.name.is_empty() {
            return Err(SvntogitPackagesError::InvalidState);
        }
        Ok(format!("{}: ok", self.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_name() {
        // Arrange / Act / Assert
        assert!(matches!(SvntogitPackages::new("  "), Err(SvntogitPackagesError::InvalidInput(_))));
    }

    #[test]
    fn run_returns_deterministic_result() {
        let c = SvntogitPackages::new("test").unwrap();
        assert_eq!(c.run().unwrap(), "test: ok");
    }

    #[test]
    fn name_is_accessible() {
        let c = SvntogitPackages::new("abc").unwrap();
        assert_eq!(c.name(), "abc");
    }
}

use std::fmt;

/// Errors that can occur during an apply operation.
#[derive(Debug)]
pub enum ApplyError {
    /// The anchor was not found in the file.
    NoMatch,
    /// The anchor matched multiple locations in the file.
    MultipleMatches,
    /// The computed hash does not match the expected hash.
    HashMismatch,
    /// I/O error with a description.
    IoError(String),
}

impl fmt::Display for ApplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplyError::NoMatch => write!(f, "NO_MATCH"),
            ApplyError::MultipleMatches => write!(f, "MULTIPLE_MATCHES"),
            ApplyError::HashMismatch => write!(f, "HASH_MISMATCH"),
            ApplyError::IoError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ApplyError {}

impl From<anchorscope::AnchorScopeError> for ApplyError {
    fn from(err: anchorscope::AnchorScopeError) -> Self {
        match err {
            anchorscope::AnchorScopeError::NoMatch => ApplyError::NoMatch,
            anchorscope::AnchorScopeError::MultipleMatches => ApplyError::MultipleMatches,
            anchorscope::AnchorScopeError::HashMismatch => ApplyError::HashMismatch,
            // InvalidHashFormat cannot occur: apply computes the hash itself
            // and never accepts an externally supplied one.
            anchorscope::AnchorScopeError::InvalidHashFormat => {
                unreachable!("apply does not accept external hashes")
            }
            anchorscope::AnchorScopeError::IoError(msg) => ApplyError::IoError(msg),
        }
    }
}

pub use crate::error::ApplyError;

/// Result of a successful apply operation.
#[derive(Debug)]
pub struct ApplyResult {
    /// Total number of bytes written to the file.
    pub bytes_written: usize,
    /// xxh3_64 hash of the matched scope (lowercase 16-char hex).
    pub scope_hash: String,
}

/// Apply a replacement to a file at the location identified by an anchor.
///
/// This function performs a read-then-write cycle using AnchorScope:
/// 1. `anchorscope::read()` — match the anchor and obtain the scope hash
/// 2. `anchorscope::write()` — replace the matched scope with the replacement
///
/// Because the hash is obtained from `read()` and passed directly to `write()`,
/// a `HashMismatch` error can only occur if the file is modified between the
/// two calls (e.g., by an external process).
pub fn apply(
    file_path: &str,
    anchor: &[u8],
    replacement: &[u8],
) -> Result<ApplyResult, ApplyError> {
    // Step 1: Read — match anchor and get scope_hash
    let read_result = anchorscope::read(file_path, anchor)?;
    let scope_hash = read_result.scope_hash;

    // Step 2: Write — replace the matched scope with the replacement
    let write_result = anchorscope::write(file_path, anchor, &scope_hash, replacement)?;

    Ok(ApplyResult {
        bytes_written: write_result.bytes_written,
        scope_hash,
    })
}

//! Kernel-specific error types.

use openfang_types::error::OpenFangError;
use thiserror::Error;

/// Kernel error type wrapping OpenFangError with kernel-specific context.
#[derive(Error, Debug)]
pub enum KernelError {
    /// A wrapped OpenFangError.
    #[error(transparent)]
    OpenFang(#[from] OpenFangError),

    /// The kernel failed to boot.
    #[error("Boot failed: {0}")]
    BootFailed(String),
}

/// Alias for kernel results.
pub type KernelResult<T> = Result<T, KernelError>;

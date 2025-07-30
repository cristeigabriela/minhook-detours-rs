//! Errors for `minhook-detours-sys`.

use minhook_detours_sys::{
    MH_ERROR_ALREADY_CREATED, MH_ERROR_ALREADY_INITIALIZED, MH_ERROR_DETOURS_TRANSACTION_BEGIN,
    MH_ERROR_DETOURS_TRANSACTION_COMMIT, MH_ERROR_DISABLED, MH_ERROR_ENABLED,
    MH_ERROR_FUNCTION_NOT_FOUND, MH_ERROR_MEMORY_ALLOC, MH_ERROR_MODULE_NOT_FOUND,
    MH_ERROR_NOT_CREATED, MH_ERROR_NOT_EXECUTABLE, MH_ERROR_NOT_INITIALIZED,
    MH_ERROR_UNABLE_TO_UNINITIALIZE, MH_ERROR_UNSUPPORTED_FUNCTION, MH_STATUS,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("MinHook is already initialized")]
    AlreadyInitialized,
    #[error("MinHook is not initialized yet, or already uninitialized")]
    NotInitialized,
    #[error("MinHook can't be uninitialized due to hooks that failed to be removed")]
    UnableToInitialize,
    #[error("The hook for the specified target function is already created")]
    AlreadyCreated,
    #[error("The hook for the specified target function is not created yet")]
    NotCreated,
    #[error("The hook for the specified target function is already enabled")]
    Enabled,
    #[error("The hook for the specified target function is not enabled yet, or already disabled")]
    Disabled,
    #[error(
        "The specified pointer is invalid. It points the address of non-allocated and/or non-executable region"
    )]
    NotExecutable,
    #[error("Detours failed to begin the hooking transaction")]
    FailedTransactionBegin,
    #[error("Detours failed to commit the hooking transaction")]
    FailedTransactionCommit,
    #[error("The specified target function cannot be hooked")]
    UnsupportedFunction,
    #[error("Failed to allocate memory")]
    FailedAllocatingMemory,
    #[error("The specified module is not loaded")]
    ModuleNotFound,
    #[error("The specified function is not found")]
    FunctionNotFound,

    // -------------------------------------------------------------------------------------------------------
    // Above are the MinHook-native possible errors, following are Rust-level ones. For consistency, even if
    // it may fit, the previous results will not be used.
    // -------------------------------------------------------------------------------------------------------
    #[error("The specified pointer is known to be invalid")]
    InvalidTarget,
}

impl From<MH_STATUS> for Error {
    /// Implement [`Error`] from [`MH_STATUS`].
    ///
    /// # Arguments
    ///
    /// * `value` - [`MH_STATUS`] returned by C API.
    fn from(value: MH_STATUS) -> Self {
        match value {
            MH_ERROR_ALREADY_INITIALIZED => Self::AlreadyInitialized,
            MH_ERROR_NOT_INITIALIZED => Self::NotInitialized,
            MH_ERROR_UNABLE_TO_UNINITIALIZE => Self::UnableToInitialize,
            MH_ERROR_ALREADY_CREATED => Self::AlreadyCreated,
            MH_ERROR_NOT_CREATED => Self::NotCreated,
            MH_ERROR_ENABLED => Self::Enabled,
            MH_ERROR_DISABLED => Self::Disabled,
            MH_ERROR_NOT_EXECUTABLE => Self::NotExecutable,
            MH_ERROR_DETOURS_TRANSACTION_BEGIN => Self::FailedTransactionBegin,
            MH_ERROR_DETOURS_TRANSACTION_COMMIT => Self::FailedTransactionCommit,
            MH_ERROR_UNSUPPORTED_FUNCTION => Self::UnsupportedFunction,
            MH_ERROR_MEMORY_ALLOC => Self::FailedAllocatingMemory,
            MH_ERROR_MODULE_NOT_FOUND => Self::ModuleNotFound,
            MH_ERROR_FUNCTION_NOT_FOUND => Self::FunctionNotFound,
            _ => unreachable!(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

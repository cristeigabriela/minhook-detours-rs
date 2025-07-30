//! Detour Guard.
//!
//! Responsible for instanciating MinHook engine, initializing it, and de-initializing it upon end.
//! For the reason of `#[inline(always)]` spam, please refer [to this GitHub issue](https://github.com/m417z/minhook-detours/issues/1).

use minhook_detours_sys::{
    MH_CreateHook, MH_DisableHook, MH_EnableHook, MH_Initialize, MH_OK, MH_SetThreadFreezeMethod, MH_Uninitialize,
};
use std::{ops::Drop, os::raw::c_void};

use crate::{
    error::{Error, Result},
    guard::thread_freeze::ThreadFreezeMethod,
};

mod thread_freeze;

/// Can be used with [`MH_EnableHook`], ...
const MH_ALL_HOOKS: *mut c_void = std::ptr::null_mut();

/// [`DetourGuard`] is the structure responsible for initializing, and deinitializing the
/// MinHook engine context.
///
/// It should only be constructed once at a time, for the duration of the hooks,
/// otherwise it's going to return an error.
#[derive(Debug)]
pub struct DetourGuard {}

impl DetourGuard {
    #[inline(always)]
    pub fn new() -> Result<Self> {
        // Attempt to initialize MinHook engine.
        let status = unsafe { MH_Initialize() };

        // If the status is [`MH_OK`], return an instance of the [`DetourGuard`].
        if status == MH_OK {
            return Ok(Self::default());
        }

        // If the `status` is not [`MH_OK`], return an error from it.
        Err(Error::from(status))
    }

    /// Attempt to do a graceful close of the [`DetourGuard`].
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the close was succesful.
    /// - `Err(minhook_detours_rs::error::Error)` if the deinitialization didn't succeed.
    #[inline(always)]
    pub fn try_close(&mut self) -> Result<()> {
        // Also responsible for disabling all current hooks, and then removing them.
        let status = unsafe { MH_Uninitialize() };

        // If the status is [`MH_OK`], we succeeded in closing the guard.
        if status == MH_OK {
            // We succesfully disposed of ourselves!
            return Ok(());
        }

        // If the `status` is not [`MH_OK`], return an error from it.
        Err(Error::from(status))
    }

    /// Consume [`DetourGuard`] attempting to do a graceful close of the [`DetourGuard`].
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the close was succesful. In the process, it also does [`std::mem::forget`] to prevent [`std::ops::Drop::drop`] from being called.
    /// - `Err(minhook_detours_rs::error::Error)` if the deinitialization didn't succeed.
    #[inline(always)]
    pub fn close(mut self) -> Result<()> {
        // Attempt to close.
        self.try_close()?;

        // Make sure destructor doesn't run.
        std::mem::forget(self);
        Ok(())
    }

    #[inline(always)]
    pub fn set_thread_freeze_method(
        &mut self,
        thread_freeze_method: ThreadFreezeMethod,
    ) -> Result<()> {
        let status = unsafe { MH_SetThreadFreezeMethod(thread_freeze_method.into()) };

        if status == MH_OK {
            // We succesfully changed the method!
            return Ok(());
        }

        Err(Error::from(status))
    }

    #[inline(always)]
    pub fn create_hook(&mut self, target: *mut c_void, detour: *mut c_void) -> Result<*mut c_void> {
        let mut original = std::ptr::null_mut() as *mut c_void;

        // Only responsible for registering a hook in the engine's structure, but does nothing
        // without the hook being enabled. Refer to [`DetourGuard::enable_hook`].
        let status = unsafe { MH_CreateHook(target as _, detour as _, &mut original as _) };

        if status == MH_OK {
            // We succesfully registered a hook!
            return Ok(original);
        }

        Err(Error::from(status))
    }

    #[inline(always)]
    pub fn enable_hook(&mut self, target: *mut c_void) -> Result<()> {
        // Although it would be a valid API usage, you should instead refer to
        // [`DetourGuard::enable_all_hooks`] to not introduce multiple ways of
        // achieving the same goal.
        if target.is_null() {
            return Err(Error::InvalidTarget);
        }

        let status = unsafe { MH_EnableHook(target) };

        if status == MH_OK {
            // We succesfully enabled a hook!
            return Ok(());
        }

        Err(Error::from(status))
    }

    #[inline(always)]
    pub fn create_and_enable_hook(
        &mut self,
        target: *mut c_void,
        detour: *mut c_void,
    ) -> Result<*mut c_void> {
        let result = self.create_hook(target, detour)?;
        self.enable_hook(target)?;
        Ok(result)
    }

    #[inline(always)]
    pub fn enable_all_hooks(&mut self) -> Result<()> {
        let status = unsafe { MH_EnableHook(MH_ALL_HOOKS) };

        if status == MH_OK {
            // We succesfully enabled all hooks!
            return Ok(());
        }

        Err(Error::from(status))
    }

    #[inline(always)]
    pub fn disable_hook(&mut self, target: *mut c_void) -> Result<()> {
        // Although it would be a valid API usage, you should instead refer to
        // [`DetourGuard::disable_all_hooks`] to not introduce multiple ways of
        // achieving the same goal.
        if target.is_null() {
            return Err(Error::InvalidTarget);
        }

        let status = unsafe { MH_DisableHook(target) };

        if status == MH_OK {
            // We succesfully disabled a hook!
            return Ok(());
        }

        Err(Error::from(status))
    }

    #[inline(always)]
    pub fn disable_all_hooks(&mut self) -> Result<()> {
        let status = unsafe { MH_DisableHook(MH_ALL_HOOKS) };

        if status == MH_OK {
            // We succesfully disabled all hooks!
            return Ok(());
        }

        Err(Error::from(status))
    }
}

impl Drop for DetourGuard {
    fn drop(&mut self) {
        if let Err(e) = self.try_close() {
            eprintln!("DetourGuard drop failed: {e:?}");
        }
    }
}

impl Default for DetourGuard {
    fn default() -> Self {
        Self {}
    }
}

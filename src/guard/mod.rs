//! Detour Guard.
//! 
//! Responsible for instanciating MinHook engine, initializing it, and de-initializing it upon end.

use minhook_detours_sys::{MH_Initialize, MH_Uninitialize, MH_OK};
use std::ops::Drop;

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct DetourGuard { }

impl DetourGuard {
    pub fn new() -> Result<Self> {
        // Attempt to initialize MinHook engine.
        let status = unsafe { MH_Initialize() };

        // If the status is [`MH_OK`], return an instance of the [`DetourGuard`].
        if status == MH_OK {
            return Ok(Self {});
        }

        // If the `status` is not [`MH_OK`], return an error from it.
        Err(Error::from(status))
    }

    pub fn try_close(&mut self) -> Result<()> {
        let status = unsafe { MH_Uninitialize() };

        // If the status is [`MH_OK`], we succeeded in closing the guard.
        if status == MH_OK {
            // We succesfully disposed of ourselves!
            return Ok(());
        }

        // If the `status` is not [`MH_OK`], return an error from it.
        Err(Error::from(status))
    }

    pub fn close(mut self) -> Result<()> {
        // Attempt to close.
        self.try_close()?;

        // Make sure destructor doesn't run.
        Ok(std::mem::forget(self))
    }

}

impl Drop for DetourGuard {
    fn drop(&mut self) {
         if let Err(e) = self.try_close() {
            eprintln!("DetourGuard drop failed: {e:?}");
         }
    }
}
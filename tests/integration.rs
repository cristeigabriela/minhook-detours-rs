
use minhook_detours_rs::{
    error::Result,
    guard::DetourGuard,
};
use serial_test::serial;

// The `#[serial]` attribute is used to make sure the tests don't run in parallel, which could lead to
// the creation of multiple [`DetourGuard`]-s at the same time, which is unsupported behavior.

#[test]
#[serial]
fn add_two_hook() -> Result<()> {
    // Generate a simple DetourGuard.
    let mut guard = DetourGuard::new()?;

    fn add_two(x: i32, y: i32) -> i64 {
        (x + y) as i64
    }

    fn add_two_hook(x: i32, y: i32) -> i64 {
        (x - y) as i64
    }

    // If `original` is null, there must be an issue.
    let original = guard.create_and_enable_hook(add_two as _, add_two_hook as _)?;
    assert_ne!(original, std::ptr::null_mut());

    // If the hook was succesfully applied, then the function [`add_two`]
    // should instead substract the two arguments, resulting in 0.
    assert_eq!(add_two(2, 2), 0);

    Ok(())
}

#[test]
#[serial]
fn two_sequential_guards() -> Result<()> {
    // First guard
    {
        let mut guard = DetourGuard::new()?;

        fn return_number() -> u32 {
            42
        }

        fn return_number_hook() -> u32 {
            1337
        }

        // If `original` is null, there must be an issue.
        let original = guard.create_and_enable_hook(return_number as _, return_number_hook as _)?;
        assert_ne!(original, std::ptr::null_mut());

        // If the hook was succesfully applied, then the function [`return_number`]
        // should return 1337 instead of 42.
        assert_eq!(return_number(), 1337);
    }

    // Second guard
    {
        let mut guard = DetourGuard::new()?;

        fn return_number_2() -> u32 {
            1337
        }

        fn return_number_2_hook() -> u32 {
            42
        }

        // If `original` is null, there must be an issue.
        let original =
            guard.create_and_enable_hook(return_number_2 as _, return_number_2_hook as _)?;
        assert_ne!(original, std::ptr::null_mut());

        // If the hook was succesfully applied, then the function [`return_number_2`]
        // should return 42 instead of 1337.
        assert_eq!(return_number_2(), 42);
    }

    Ok(())
}

#[test]
#[serial]
fn hook_then_disable() -> Result<()> {
    unsafe {
        let mut guard = DetourGuard::new()?;

        unsafe extern "system" fn add_two(lhs: i32, rhs: i32) -> i64 {
            (lhs + rhs) as i64
        }

        unsafe extern "system" fn add_two_hook(lhs: i32, rhs: i32) -> i64 {
            (lhs - rhs) as i64
        }

        let original = guard.create_and_enable_hook(add_two as _, add_two_hook as _)?;

        // If `original` is null, there must be an issue.
        assert_ne!(original, std::ptr::null_mut());

        // If the hook was succesfully applied, then the function [`add_two`]
        // should instead substract the two arguments, resulting in 0.
        assert_eq!(add_two(2, 2), 0);

        // Disable hook.
        guard.disable_hook(add_two as _)?;

        // If the hook was succesfully disabled, then the function [`add_two`]
        // should return it's original return value.
        assert_eq!(add_two(2, 2), 4);
    }

    Ok(())
}

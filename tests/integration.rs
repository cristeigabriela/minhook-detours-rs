use minhook_detours_rs::{error::Result, guard::DetourGuard};
use serial_test::serial;

// The `#[serial]` attribute is used to make sure the tests don't run in parallel, which could lead to
// the creation of multiple [`DetourGuard`]-s at the same time, which is unsupported behavior.

#[test]
#[serial]
fn add_two_hook() -> Result<()> {
    // Generate a simple DetourGuard.
    let mut guard = DetourGuard::new()?;

    // The type of the hooked function, and of the detour.
    type FunctionType = fn(i32, i32) -> i64;

    fn add_two(x: i32, y: i32) -> i64 {
        (x + y) as i64
    }

    fn add_two_hook(x: i32, y: i32) -> i64 {
        (x - y) as i64
    }

    let _ = guard.create_and_enable_hook::<FunctionType>(add_two as _, add_two_hook as _)?;

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

        // The type of the hooked function, and of the detour.
        type FunctionType = fn() -> u32;

        fn return_number() -> u32 {
            42
        }

        fn return_number_hook() -> u32 {
            1337
        }

        let _ = guard
            .create_and_enable_hook::<FunctionType>(return_number as _, return_number_hook as _)?;

        // If the hook was succesfully applied, then the function [`return_number`]
        // should return 1337 instead of 42.
        assert_eq!(return_number(), 1337);
    }

    // Second guard
    {
        let mut guard = DetourGuard::new()?;

        // The type of the hooked function, and of the detour.
        type FunctionType = fn() -> u32;

        fn return_number_2() -> u32 {
            1337
        }

        fn return_number_2_hook() -> u32 {
            42
        }

        // If `original` is null, there must be an issue.
        let _ = guard.create_and_enable_hook::<FunctionType>(
            return_number_2 as _,
            return_number_2_hook as _,
        )?;

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

        // The type of the hooked function, and of the detour.
        type FunctionType = unsafe extern "system" fn(i32, i32) -> i64;

        unsafe extern "system" fn add_two(lhs: i32, rhs: i32) -> i64 {
            (lhs + rhs) as i64
        }

        unsafe extern "system" fn add_two_hook(lhs: i32, rhs: i32) -> i64 {
            (lhs - rhs) as i64
        }

        let _ = guard.create_and_enable_hook::<FunctionType>(add_two as _, add_two_hook as _)?;

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

#[test]
#[serial]
fn complex_type_test() -> Result<()> {
    let mut guard = DetourGuard::new()?;

    // The type of the hooked function, and of the detour.
    type FunctionType = fn() -> String;

    fn return_string() -> String {
        "Hello, world!".into()
    }

    fn return_string_hook() -> String {
        "Bye, world!".into()
    }

    let _ = guard
        .create_and_enable_hook::<FunctionType>(return_string as _, return_string_hook as _)?;

    // If the hook was succesfully applied, then the function [`return_string`]
    // should return the value specified by [`return_string_hook`].
    assert_eq!(return_string(), "Bye, world!");

    Ok(())
}

#[test]
#[serial]
fn standard_original_usage() -> Result<()> {
    let mut guard = DetourGuard::new()?;

    unsafe {
        // The type of the hooked function, and of the detour.
        type FunctionType = fn(String, String) -> String;

        // Variable holding reference to original.
        static mut ORIGINAL: Option<&FunctionType> = None;

        fn return_joined_strings(x: String, y: String) -> String {
            format!("{x}, {y}!").into()
        }

        fn return_joined_strings_hook(_x: String, _y: String) -> String {
            let x = "Bye".to_owned();
            let y = "World".to_owned();

            unsafe {
                let original = ORIGINAL.unwrap();
                original(x, y)
            }
        }

        let original = guard.create_and_enable_hook::<FunctionType>(
            return_joined_strings as _,
            return_joined_strings_hook as _,
        )?;
        ORIGINAL = Some(original);

        // If the hook was succesfully applied, then the function [`return_joined_strings`]
        // should return the value specified by [`return_joined_strings_hook`].
        assert_eq!(return_joined_strings("a".into(), "b".into()), "Bye, World!");
    }

    Ok(())
}

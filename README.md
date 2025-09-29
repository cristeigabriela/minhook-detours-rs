# minhook-detours-rs

Rustic wrapper of [minhook-detours-sys](https://github.com/metalbear-co/minhook-detours-sys) bindings.

# Example

```rs
let mut guard = DetourGuard::new()?;

// The type of the hooked function, and of the detour.
type FunctionType = unsafe extern "system" fn(i32, i32) -> i64;

unsafe extern "system" fn add_two(lhs: i32, rhs: i32) -> i64 {
    (lhs + rhs) as i64
}

unsafe extern "system" fn add_two_hook(lhs: i32, rhs: i32) -> i64 {
    (lhs - rhs) as i64
}

let original = guard.create_and_enable_hook::<FunctionType>(add_two as _, add_two_hook as _)?;
```

# License
[License: BSD-2-Clause](./LICENSE)

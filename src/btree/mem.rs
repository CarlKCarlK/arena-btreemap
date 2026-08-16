use core::{mem, ptr};

#[cfg(feature = "std")]
fn abort() -> ! {
    std::process::abort()
}

#[cfg(not(feature = "std"))]
fn abort() -> ! {
    // In no_std, use intrinsic abort. This requires the wasm32 target
    // to have the `abort` intrinsic, which it does.
    struct Abort;
    impl Drop for Abort {
        fn drop(&mut self) {
            panic!("abort() called in no_std context");
        }
    }
    let _abort = Abort;
    panic!("abort in no_std");
}

/// This replaces the value behind the `v` unique reference by calling the
/// relevant function.
///
/// If a panic occurs in the `change` closure, the entire process will be aborted.
#[allow(dead_code)] // keep as illustration and for future use
#[inline]
pub(super) fn take_mut<T>(v: &mut T, change: impl FnOnce(T) -> T) {
    replace(v, |value| (change(value), ()))
}

/// This replaces the value behind the `v` unique reference by calling the
/// relevant function, and returns a result obtained along the way.
///
/// If a panic occurs in the `change` closure, the entire process will be aborted.
#[inline]
pub(super) fn replace<T, R>(v: &mut T, change: impl FnOnce(T) -> (T, R)) -> R {
    struct PanicGuard;
    impl Drop for PanicGuard {
        fn drop(&mut self) {
            abort()
        }
    }
    let guard = PanicGuard;
    let value = unsafe { ptr::read(v) };
    let (new_value, ret) = change(value);
    unsafe {
        ptr::write(v, new_value);
    }
    mem::forget(guard);
    ret
}

#[doc(hidden)]
#[macro_export]
macro_rules! no_arg_shim {
    ($name: ident, $func: ident, $wid: ident, $cb: ident) => {
        unsafe extern "C" fn shim(wid: *mut wrap::$name, data: *mut std::os::raw::c_void) {
            let wid: Ptr<$name> = Ptr::from_raw(wid as _);
            let a = data as *mut Box<dyn FnMut(Ptr<$name>)>;
            let f: &mut (dyn FnMut(Ptr<$name>)) = &mut **a;
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(wid)));
        }
        let a: *mut Box<dyn FnMut(Ptr<$name>)> = Box::into_raw(Box::new(Box::new($cb)));
        let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
        let callback: Option<
            unsafe extern "C" fn(arg1: *mut wrap::$name, arg2: *mut ::std::os::raw::c_void),
        > = Some(shim);
        let wid: Ptr<$name> = $wid.cast_into();
        wrap::$func(wid.as_mut_raw_ptr() as _, callback, data);
    };
}

pub use no_arg_shim;

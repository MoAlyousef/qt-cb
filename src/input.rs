use crate::inputsys as wrap;
use crate::prelude::InputExt;
use cpp_core::{CastInto, Ptr, Ref};
use qt_core::{QBox, QString};
use qt_widgets::QLineEdit;

fn on_text_changed<F: FnMut(Ptr<QLineEdit>, Ref<QString>) + 'static>(
    btn: impl CastInto<Ptr<QLineEdit>>,
    cb: F,
) {
    unsafe {
        unsafe extern "C" fn shim(
            wid: *mut wrap::QLineEdit,
            st: *mut std::os::raw::c_void,
            data: *mut std::os::raw::c_void,
        ) {
            let wid: Ptr<QLineEdit> = Ptr::from_raw(wid as _);
            let a = data as *mut Box<dyn FnMut(Ptr<QLineEdit>, Ref<QString>)>;
            let f: &mut (dyn FnMut(Ptr<QLineEdit>, Ref<QString>)) = &mut **a;
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(wid, Ref::from_raw(st as *const QString).unwrap())
            }));
        }
        let a: *mut Box<dyn FnMut(Ptr<QLineEdit>, Ref<QString>)> =
            Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
        let callback: Option<
            unsafe extern "C" fn(
                arg1: *mut wrap::QLineEdit,
                st: *mut ::std::os::raw::c_void,
                arg2: *mut ::std::os::raw::c_void,
            ),
        > = Some(shim);
        let btn: Ptr<QLineEdit> = btn.cast_into();
        wrap::QLineEdit_onTextChanged(btn.as_mut_raw_ptr() as _, callback, data);
    }
}

fn on_return_pressed<F: FnMut(Ptr<QLineEdit>) + 'static>(
    btn: impl CastInto<Ptr<QLineEdit>>,
    cb: F,
) {
    unsafe {
        crate::utils::no_arg_shim!(QLineEdit, QLineEdit_onReturnPressed, btn, cb);
    }
}

macro_rules! impl_input_ext {
    ($name: ident) => {
        impl InputExt for QBox<$name> {
            fn on_text_changed<F: FnMut(Ptr<QLineEdit>, Ref<QString>) + 'static>(&self, cb: F) {
                on_text_changed(self, cb);
            }
            fn on_return_pressed<F: FnMut(Ptr<QLineEdit>) + 'static>(&self, cb: F) {
                on_return_pressed(self, cb);
            }
        }
    };
}

impl_input_ext!(QLineEdit);

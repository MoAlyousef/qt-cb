use cpp_core::{CastInto, Ptr};
use qt_core::QBox;
use qt_widgets::{QAbstractButton, QPushButton, QCheckBox, QRadioButton};
use crate::buttonsys as wrap;
use crate::prelude::ButtonExt;

fn on_clicked<F: FnMut(Ptr<QAbstractButton>, bool) + 'static>(
    btn: impl CastInto<Ptr<QAbstractButton>>,
    cb: F,
) {
    unsafe {
        unsafe extern "C" fn shim(
            wid: *mut wrap::QAbstractButton,
            checked: i32,
            data: *mut std::os::raw::c_void,
        ) {
            let wid: Ptr<QAbstractButton> = Ptr::from_raw(wid as _);
            let a = data as *mut Box<dyn FnMut(Ptr<QAbstractButton>, bool)>;
            let f: &mut (dyn FnMut(Ptr<QAbstractButton>, bool)) = &mut **a;
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(wid, checked != 0)));
        }
        let a: *mut Box<dyn FnMut(Ptr<QAbstractButton>, bool)> =
            Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
        let callback: Option<
            unsafe extern "C" fn(
                arg1: *mut wrap::QAbstractButton,
                checked: i32,
                arg2: *mut ::std::os::raw::c_void,
            ),
        > = Some(shim);
        let btn: Ptr<QAbstractButton> = btn.cast_into();
        wrap::QAbstractButton_onClicked(btn.as_mut_raw_ptr() as _, callback, data);
    }
}

fn on_pressed<F: FnMut(Ptr<QAbstractButton>) + 'static>(
    btn: impl CastInto<Ptr<QAbstractButton>>,
    cb: F,
) {
    unsafe {
        unsafe extern "C" fn shim(
            wid: *mut wrap::QAbstractButton,
            data: *mut std::os::raw::c_void,
        ) {
            let wid: Ptr<QAbstractButton> = Ptr::from_raw(wid as _);
            let a = data as *mut Box<dyn FnMut(Ptr<QAbstractButton>)>;
            let f: &mut (dyn FnMut(Ptr<QAbstractButton>)) = &mut **a;
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(wid)));
        }
        let a: *mut Box<dyn FnMut(Ptr<QAbstractButton>)> =
            Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
        let callback: Option<
            unsafe extern "C" fn(
                arg1: *mut wrap::QAbstractButton,
                arg2: *mut ::std::os::raw::c_void,
            ),
        > = Some(shim);
        let btn: Ptr<QAbstractButton> = btn.cast_into();
        wrap::QAbstractButton_onPressed(btn.as_mut_raw_ptr() as _, callback, data);
    }
}

macro_rules! impl_btn_ext {
    ($name: ident) => {
        impl ButtonExt for QBox<$name> {
            fn on_clicked<F: FnMut(Ptr<QAbstractButton>, bool) + 'static>(&self, cb: F) {
                on_clicked(self, cb);
            }
            fn on_pressed<F: FnMut(Ptr<QAbstractButton>) + 'static>(&self, cb: F) {
                on_pressed(self, cb);
            }
        }
    }
}


impl_btn_ext!(QCheckBox);
impl_btn_ext!(QPushButton);
impl_btn_ext!(QRadioButton);
impl_btn_ext!(QAbstractButton);


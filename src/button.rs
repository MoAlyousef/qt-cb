use crate::buttonsys as wrap;
use crate::prelude::ButtonExt;
use cpp_core::{CastInto, Ptr};
use qt_core::QBox;
use qt_widgets::{QAbstractButton, QCheckBox, QPushButton, QRadioButton};

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

fn on_toggled<F: FnMut(Ptr<QAbstractButton>, bool) + 'static>(
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
        wrap::QAbstractButton_onToggled(btn.as_mut_raw_ptr() as _, callback, data);
    }
}

fn on_pressed<F: FnMut(Ptr<QAbstractButton>) + 'static>(
    btn: impl CastInto<Ptr<QAbstractButton>>,
    cb: F,
) {
    unsafe {
        crate::utils::no_arg_shim!(QAbstractButton, QAbstractButton_onPressed, btn, cb);
    }
}

fn on_released<F: FnMut(Ptr<QAbstractButton>) + 'static>(
    btn: impl CastInto<Ptr<QAbstractButton>>,
    cb: F,
) {
    unsafe {
        crate::utils::no_arg_shim!(QAbstractButton, QAbstractButton_onReleased, btn, cb);
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
            fn on_toggled<F: FnMut(Ptr<QAbstractButton>, bool) + 'static>(&self, cb: F) {
                on_toggled(self, cb);
            }
            fn on_released<F: FnMut(Ptr<QAbstractButton>) + 'static>(&self, cb: F) {
                on_released(self, cb);
            }
        }
    };
}

impl_btn_ext!(QCheckBox);
impl_btn_ext!(QPushButton);
impl_btn_ext!(QRadioButton);
impl_btn_ext!(QAbstractButton);

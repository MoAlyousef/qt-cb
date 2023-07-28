use crate::prelude::ButtonExt;
use cpp_core::{CastInto, Ptr};
use qt_core::{QBox, SlotNoArgs, SlotOfBool};
use qt_widgets::{QAbstractButton, QCheckBox, QPushButton, QRadioButton};

fn on_clicked<F: 'static + FnMut(Ptr<QAbstractButton>, bool)>(
    wid: impl CastInto<Ptr<QAbstractButton>>,
    mut cb: F,
) {
    unsafe {
        let wid = wid.cast_into();
        wid.clicked().connect(&SlotOfBool::new(wid, move |b| {
            cb(wid, b);
        }));
    }
}

fn on_toggled<F: FnMut(Ptr<QAbstractButton>, bool) + 'static>(
    wid: impl CastInto<Ptr<QAbstractButton>>,
    mut cb: F,
) {
    unsafe {
        let wid = wid.cast_into();
        wid.toggled().connect(&SlotOfBool::new(wid, move |b| {
            cb(wid, b);
        }));
    }
}

fn on_pressed<F: FnMut(Ptr<QAbstractButton>) + 'static>(
    wid: impl CastInto<Ptr<QAbstractButton>>,
    mut cb: F,
) {
    unsafe {
        let wid = wid.cast_into();
        wid.pressed().connect(&SlotNoArgs::new(wid, move || {
            cb(wid);
        }));
    }
}

fn on_released<F: FnMut(Ptr<QAbstractButton>) + 'static>(
    wid: impl CastInto<Ptr<QAbstractButton>>,
    mut cb: F,
) {
    unsafe {
        let wid = wid.cast_into();
        wid.released().connect(&SlotNoArgs::new(wid, move || {
            cb(wid);
        }));
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

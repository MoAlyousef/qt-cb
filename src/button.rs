use crate::prelude::ButtonExt;
use cpp_core::{CastInto, Ptr};
use qt_core::{QBox, SlotNoArgs, SlotOfBool};
use qt_widgets::{QAbstractButton, QCheckBox, QPushButton, QRadioButton};

macro_rules! impl_btn_ext {
    ($name: ident) => {
        impl ButtonExt for QBox<$name> {
            unsafe fn on_clicked<F: FnMut(Ptr<QAbstractButton>, bool) + 'static>(&self, mut cb: F) {
                let wid: Ptr<QAbstractButton> = self.cast_into();
                wid.clicked().connect(&SlotOfBool::new(wid, move |b| {
                    cb(wid, b);
                }));
            }
            unsafe fn on_pressed<F: FnMut(Ptr<QAbstractButton>) + 'static>(&self, mut cb: F) {
                let wid: Ptr<QAbstractButton> = self.cast_into();
                wid.pressed().connect(&SlotNoArgs::new(wid, move || {
                    cb(wid);
                }));
            }
            unsafe fn on_toggled<F: FnMut(Ptr<QAbstractButton>, bool) + 'static>(&self, mut cb: F) {
                let wid: Ptr<QAbstractButton> = self.cast_into();
                wid.toggled().connect(&SlotOfBool::new(wid, move |b| {
                    cb(wid, b);
                }));
            }
            unsafe fn on_released<F: FnMut(Ptr<QAbstractButton>) + 'static>(&self, mut cb: F) {
                let wid: Ptr<QAbstractButton> = self.cast_into();
                wid.released().connect(&SlotNoArgs::new(wid, move || {
                    cb(wid);
                }));
            }
        }
    };
}

impl_btn_ext!(QCheckBox);
impl_btn_ext!(QPushButton);
impl_btn_ext!(QRadioButton);
impl_btn_ext!(QAbstractButton);

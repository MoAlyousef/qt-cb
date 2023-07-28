use crate::prelude::ButtonExt;
use cpp_core::{CastInto, Ptr};
use qt_core::{QBox, SlotNoArgs, SlotOfBool};
use qt_widgets::{QAbstractButton, QCheckBox, QPushButton, QRadioButton};
use crate::utils;

macro_rules! impl_ext {
    ($name: ident) => {
        impl ButtonExt for QBox<$name> {
            unsafe fn on_clicked<F: FnMut(Ptr<QAbstractButton>, bool) + 'static>(&self, mut cb: F) {
                utils::connect_1a!(QAbstractButton, self, clicked, SlotOfBool, cb);
            }
            unsafe fn on_pressed<F: FnMut(Ptr<QAbstractButton>) + 'static>(&self, mut cb: F) {
                utils::connect_0a!(QAbstractButton, self, pressed, cb);
            }
            unsafe fn on_toggled<F: FnMut(Ptr<QAbstractButton>, bool) + 'static>(&self, mut cb: F) {
                utils::connect_1a!(QAbstractButton, self, toggled, SlotOfBool, cb);
            }
            unsafe fn on_released<F: FnMut(Ptr<QAbstractButton>) + 'static>(&self, mut cb: F) {
                utils::connect_0a!(QAbstractButton, self, released, cb);
            }
        }
    };
}

impl_ext!(QCheckBox);
impl_ext!(QPushButton);
impl_ext!(QRadioButton);
impl_ext!(QAbstractButton);

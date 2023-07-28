use crate::prelude::ButtonExt;
use crate::utils;
use qt_core::{QBox, SlotNoArgs, SlotOfBool};
use qt_widgets::{QAbstractButton, QCheckBox, QPushButton, QRadioButton};

macro_rules! impl_ext {
    ($name: ident) => {
        impl ButtonExt for QBox<$name> {
            unsafe fn on_clicked<F: FnMut(&Self, bool) + 'static>(&self, mut cb: F) {
                utils::connect_1a!(QAbstractButton, self, clicked, SlotOfBool, cb);
            }
            unsafe fn on_pressed<F: FnMut(&Self) + 'static>(&self, mut cb: F) {
                utils::connect_0a!(QAbstractButton, self, pressed, cb);
            }
            unsafe fn on_toggled<F: FnMut(&Self, bool) + 'static>(&self, mut cb: F) {
                utils::connect_1a!(QAbstractButton, self, toggled, SlotOfBool, cb);
            }
            unsafe fn on_released<F: FnMut(&Self) + 'static>(&self, mut cb: F) {
                utils::connect_0a!(QAbstractButton, self, released, cb);
            }
        }
    };
}

impl_ext!(QCheckBox);
impl_ext!(QPushButton);
impl_ext!(QRadioButton);
impl_ext!(QAbstractButton);

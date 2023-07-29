use crate::prelude::ButtonExt;
use crate::utils;
use qt_core::{QBox, SlotNoArgs, SlotOfBool};
use qt_widgets::{QAbstractButton, QCheckBox, QPushButton, QRadioButton};

macro_rules! impl_ext {
    ($name: ident) => {
        impl ButtonExt for QBox<$name> {
            unsafe fn connect_clicked<F: FnMut(&Self, bool) + 'static>(&self, mut cb: F) {
                utils::connect_1a!(self, clicked, SlotOfBool, cb);
            }
            unsafe fn connect_pressed<F: FnMut(&Self) + 'static>(&self, mut cb: F) {
                utils::connect_0a!(self, pressed, cb);
            }
            unsafe fn connect_toggled<F: FnMut(&Self, bool) + 'static>(&self, mut cb: F) {
                utils::connect_1a!(self, toggled, SlotOfBool, cb);
            }
            unsafe fn connect_released<F: FnMut(&Self) + 'static>(&self, mut cb: F) {
                utils::connect_0a!(self, released, cb);
            }
        }
    };
}

impl_ext!(QCheckBox);
impl_ext!(QPushButton);
impl_ext!(QRadioButton);
impl_ext!(QAbstractButton);

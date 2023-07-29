use crate::prelude::InputExt;
use crate::utils;
use cpp_core::Ref;
use qt_core::{QBox, QString, SlotNoArgs, SlotOfQString};
use qt_widgets::QLineEdit;

macro_rules! impl_ext {
    ($name: ident) => {
        impl InputExt for QBox<$name> {
            unsafe fn connect_text_changed<F: FnMut(&Self, Ref<QString>) + 'static>(
                &self,
                mut cb: F,
            ) {
                utils::connect_1a!(self, text_changed, SlotOfQString, cb);
            }
            unsafe fn connect_return_pressed<F: FnMut(&Self) + 'static>(&self, mut cb: F) {
                utils::connect_0a!(self, return_pressed, cb);
            }
        }
    };
}

impl_ext!(QLineEdit);

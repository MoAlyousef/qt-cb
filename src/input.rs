use crate::prelude::InputExt;
use cpp_core::{CastInto, Ptr, Ref};
use qt_core::{QBox, QString, SlotNoArgs, SlotOfQString};
use qt_widgets::QLineEdit;
use crate::utils;

macro_rules! impl_input_ext {
    ($name: ident) => {
        impl InputExt for QBox<$name> {
            unsafe fn on_text_changed<F: FnMut(Ptr<QLineEdit>, Ref<QString>) + 'static>(
                &self,
                mut cb: F,
            ) {
                utils::on_signal_1a!(QLineEdit, self, text_changed, SlotOfQString, cb);
            }
            unsafe fn on_return_pressed<F: FnMut(Ptr<QLineEdit>) + 'static>(&self, mut cb: F) {
                utils::on_signal!(QLineEdit, self, return_pressed, cb);
            }
        }
    };
}

impl_input_ext!(QLineEdit);

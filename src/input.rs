use crate::prelude::InputExt;
use cpp_core::{CastInto, Ptr, Ref};
use qt_core::{QBox, QString, SlotNoArgs, SlotOfQString};
use qt_widgets::QLineEdit;

macro_rules! impl_input_ext {
    ($name: ident) => {
        impl InputExt for QBox<$name> {
            unsafe fn on_text_changed<F: FnMut(Ptr<QLineEdit>, Ref<QString>) + 'static>(
                &self,
                mut cb: F,
            ) {
                let wid: Ptr<QLineEdit> = self.cast_into();
                wid.text_changed()
                    .connect(&SlotOfQString::new(wid, move |b| {
                        cb(wid, b);
                    }));
            }
            unsafe fn on_return_pressed<F: FnMut(Ptr<QLineEdit>) + 'static>(&self, mut cb: F) {
                let wid: Ptr<QLineEdit> = self.cast_into();
                wid.return_pressed().connect(&SlotNoArgs::new(wid, move || {
                    cb(wid);
                }));
            }
        }
    };
}

impl_input_ext!(QLineEdit);

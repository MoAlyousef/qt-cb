use crate::prelude::InputExt;
use cpp_core::{CastInto, Ptr, Ref};
use qt_core::{QBox, QString, SlotNoArgs, SlotOfQString};
use qt_widgets::QLineEdit;

fn on_text_changed<F: FnMut(Ptr<QLineEdit>, Ref<QString>) + 'static>(
    wid: impl CastInto<Ptr<QLineEdit>>,
    mut cb: F,
) {
    unsafe {
        let wid = wid.cast_into();
        wid.text_changed()
            .connect(&SlotOfQString::new(wid, move |b| {
                cb(wid, b);
            }));
    }
}

fn on_return_pressed<F: FnMut(Ptr<QLineEdit>) + 'static>(
    wid: impl CastInto<Ptr<QLineEdit>>,
    mut cb: F,
) {
    unsafe {
        let wid = wid.cast_into();
        wid.return_pressed().connect(&SlotNoArgs::new(wid, move || {
            cb(wid);
        }));
    }
}

macro_rules! impl_input_ext {
    ($name: ident) => {
        impl InputExt for QBox<$name> {
            fn on_text_changed<F: FnMut(Ptr<QLineEdit>, Ref<QString>) + 'static>(&self, cb: F) {
                on_text_changed(self, cb);
            }
            fn on_return_pressed<F: FnMut(Ptr<QLineEdit>) + 'static>(&self, cb: F) {
                on_return_pressed(self, cb);
            }
        }
    };
}

impl_input_ext!(QLineEdit);

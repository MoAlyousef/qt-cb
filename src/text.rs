use crate::prelude::TextExt;
use cpp_core::{CastInto, Ptr};
use qt_core::{QBox, SlotNoArgs, SlotOfBool};
use qt_widgets::{QTextEdit, QTextBrowser};
use crate::utils;

macro_rules! impl_ext {
    ($name: ident) => {
        impl TextExt for QBox<$name> {
            unsafe fn on_redo_available<F: FnMut(Ptr<QTextEdit>, bool) + 'static>(&self, mut cb: F) {
                utils::connect_1a!(QTextEdit, self, redo_available, SlotOfBool, cb);
            }
            unsafe fn on_selection_changed<F: FnMut(Ptr<QTextEdit>) + 'static>(&self, mut cb: F) {
                utils::connect_0a!(QTextEdit, self, selection_changed, cb);
            }
            unsafe fn on_text_changed<F: FnMut(Ptr<QTextEdit>) + 'static>(&self, mut cb: F) {
                utils::connect_0a!(QTextEdit, self, text_changed, cb);
            }
            unsafe fn on_undo_available<F: FnMut(Ptr<QTextEdit>, bool) + 'static>(&self, mut cb: F) {
                utils::connect_1a!(QTextEdit, self, undo_available, SlotOfBool, cb);
            }
        }
    };
}

impl_ext!(QTextEdit);
impl_ext!(QTextBrowser);

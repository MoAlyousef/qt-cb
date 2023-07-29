use crate::prelude::TextExt;
use crate::utils;
use qt_core::{QBox, SlotNoArgs, SlotOfBool};
use qt_widgets::{QTextBrowser, QTextEdit};

macro_rules! impl_ext {
    ($name: ident) => {
        impl TextExt for QBox<$name> {
            unsafe fn connect_redo_available<F: FnMut(&Self, bool) + 'static>(&self, mut cb: F) {
                utils::connect_1a!(QTextEdit, self, redo_available, SlotOfBool, cb);
            }
            unsafe fn connect_selection_changed<F: FnMut(&Self) + 'static>(&self, mut cb: F) {
                utils::connect_0a!(QTextEdit, self, selection_changed, cb);
            }
            unsafe fn connect_text_changed<F: FnMut(&Self) + 'static>(&self, mut cb: F) {
                utils::connect_0a!(QTextEdit, self, text_changed, cb);
            }
            unsafe fn connect_undo_available<F: FnMut(&Self, bool) + 'static>(&self, mut cb: F) {
                utils::connect_1a!(QTextEdit, self, undo_available, SlotOfBool, cb);
            }
        }
    };
}

impl_ext!(QTextEdit);
impl_ext!(QTextBrowser);

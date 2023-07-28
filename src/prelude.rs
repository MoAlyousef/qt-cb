use cpp_core::Ref;
use qt_core::{QPtr, QString};

pub trait ButtonExt {
    unsafe fn on_clicked<F: FnMut(&Self, bool) + 'static>(&self, cb: F);
    unsafe fn on_pressed<F: FnMut(&Self) + 'static>(&self, cb: F);
    unsafe fn on_toggled<F: FnMut(&Self, bool) + 'static>(&self, cb: F);
    unsafe fn on_released<F: FnMut(&Self) + 'static>(&self, cb: F);
}

pub trait InputExt {
    unsafe fn on_text_changed<F: FnMut(&Self, Ref<QString>) + 'static>(&self, cb: F);
    unsafe fn on_return_pressed<F: FnMut(&Self) + 'static>(&self, cb: F);
}

pub trait TextExt {
    unsafe fn on_redo_available<F: FnMut(&Self, bool) + 'static>(&self, cb: F);
    unsafe fn on_selection_changed<F: FnMut(&Self) + 'static>(&self, cb: F);
    unsafe fn on_text_changed<F: FnMut(&Self) + 'static>(&self, cb: F);
    unsafe fn on_undo_available<F: FnMut(&Self, bool) + 'static>(&self, cb: F);
}

pub trait MenuExt {
    unsafe fn on_triggered<F: FnMut(&Self, QPtr<qt_widgets::QAction>) + 'static>(&self, cb: F);
}

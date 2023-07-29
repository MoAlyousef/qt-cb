use cpp_core::Ref;
use qt_core::{QPtr, QString};

pub trait ButtonExt {
    unsafe fn connect_clicked<F: FnMut(&Self, bool) + 'static>(&self, cb: F);
    unsafe fn connect_pressed<F: FnMut(&Self) + 'static>(&self, cb: F);
    unsafe fn connect_toggled<F: FnMut(&Self, bool) + 'static>(&self, cb: F);
    unsafe fn connect_released<F: FnMut(&Self) + 'static>(&self, cb: F);
}

pub trait InputExt {
    unsafe fn connect_text_changed<F: FnMut(&Self, Ref<QString>) + 'static>(&self, cb: F);
    unsafe fn connect_return_pressed<F: FnMut(&Self) + 'static>(&self, cb: F);
}

pub trait TextExt {
    unsafe fn connect_redo_available<F: FnMut(&Self, bool) + 'static>(&self, cb: F);
    unsafe fn connect_selection_changed<F: FnMut(&Self) + 'static>(&self, cb: F);
    unsafe fn connect_text_changed<F: FnMut(&Self) + 'static>(&self, cb: F);
    unsafe fn connect_undo_available<F: FnMut(&Self, bool) + 'static>(&self, cb: F);
}

pub trait MenuExt {
    unsafe fn connect_triggered<F: FnMut(&Self, QPtr<qt_widgets::QAction>) + 'static>(&self, cb: F);
}

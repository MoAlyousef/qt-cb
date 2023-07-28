use cpp_core::{Ptr, Ref};
use qt_core::QString;
use qt_widgets::{QAbstractButton, QLineEdit};

pub trait ButtonExt {
    unsafe fn on_clicked<F: FnMut(Ptr<QAbstractButton>, bool) + 'static>(&self, cb: F);
    unsafe fn on_pressed<F: FnMut(Ptr<QAbstractButton>) + 'static>(&self, cb: F);
    unsafe fn on_toggled<F: FnMut(Ptr<QAbstractButton>, bool) + 'static>(&self, cb: F);
    unsafe fn on_released<F: FnMut(Ptr<QAbstractButton>) + 'static>(&self, cb: F);
}

pub trait InputExt {
    unsafe fn on_text_changed<F: FnMut(Ptr<QLineEdit>, Ref<QString>) + 'static>(&self, cb: F);
    unsafe fn on_return_pressed<F: FnMut(Ptr<QLineEdit>) + 'static>(&self, cb: F);
}

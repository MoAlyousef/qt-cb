use cpp_core::{Ptr, Ref};
use qt_core::{QString, QPtr};

pub trait ButtonExt {
    unsafe fn on_clicked<F: FnMut(Ptr<qt_widgets::QAbstractButton>, bool) + 'static>(&self, cb: F);
    unsafe fn on_pressed<F: FnMut(Ptr<qt_widgets::QAbstractButton>) + 'static>(&self, cb: F);
    unsafe fn on_toggled<F: FnMut(Ptr<qt_widgets::QAbstractButton>, bool) + 'static>(&self, cb: F);
    unsafe fn on_released<F: FnMut(Ptr<qt_widgets::QAbstractButton>) + 'static>(&self, cb: F);
}

pub trait InputExt {
    unsafe fn on_text_changed<F: FnMut(Ptr<qt_widgets::QLineEdit>, Ref<QString>) + 'static>(&self, cb: F);
    unsafe fn on_return_pressed<F: FnMut(Ptr<qt_widgets::QLineEdit>) + 'static>(&self, cb: F);
}

pub trait TextExt {
    unsafe fn on_redo_available<F: FnMut(Ptr<qt_widgets::QTextEdit>, bool) + 'static>(&self, cb: F);
    unsafe fn on_selection_changed<F: FnMut(Ptr<qt_widgets::QTextEdit>) + 'static>(&self, cb: F);
    unsafe fn on_text_changed<F: FnMut(Ptr<qt_widgets::QTextEdit>) + 'static>(&self, cb: F);
    unsafe fn on_undo_available<F: FnMut(Ptr<qt_widgets::QTextEdit>, bool) + 'static>(&self, cb: F);
}

pub trait MenuExt {
    unsafe fn on_triggered<F: FnMut(Ptr<qt_widgets::QMenu>, QPtr<qt_widgets::QAction>) + 'static>(&self, cb: F) where Self: Sized;
}
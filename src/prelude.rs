use cpp_core::Ptr;
use qt_widgets::QAbstractButton;

pub trait ButtonExt {
    fn on_clicked<F: FnMut(Ptr<QAbstractButton>, bool) + 'static>(&self, cb: F);
    fn on_pressed<F: FnMut(Ptr<QAbstractButton>) + 'static>(&self, cb: F);
}
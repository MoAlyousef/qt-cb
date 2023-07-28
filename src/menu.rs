use crate::prelude::MenuExt;
use cpp_core::{CastInto, Ptr};
use qt_core::{QBox, QPtr};
use qt_widgets::{QMenu, QAction, SlotOfQAction};
use crate::utils;

macro_rules! impl_ext {
    ($name: ident) => {
        impl MenuExt for QBox<$name> {
            unsafe fn on_triggered<F: FnMut(Ptr<QMenu>, QPtr<QAction>) + 'static>(&self, mut cb: F) {
                utils::connect_1a!($name, self, triggered, SlotOfQAction, cb);
            }
        }
    };
}

// impl_ext!(QMenuBar);
impl_ext!(QMenu);

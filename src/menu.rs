use crate::prelude::MenuExt;
use crate::utils;
use qt_core::{QBox, QPtr};
use qt_widgets::{QAction, QMenu, QMenuBar, SlotOfQAction};

macro_rules! impl_ext {
    ($name: ident) => {
        impl MenuExt for QBox<$name> {
            unsafe fn connect_triggered<F: FnMut(&Self, QPtr<QAction>) + 'static>(
                &self,
                mut cb: F,
            ) {
                utils::connect_1a!($name, self, triggered, SlotOfQAction, cb);
            }
        }
    };
}

impl_ext!(QMenuBar);
impl_ext!(QMenu);

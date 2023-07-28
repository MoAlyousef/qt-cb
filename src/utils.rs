#[doc(hidden)]
#[macro_export]
macro_rules! connect_0a {
    ($base: ty, $wid: ident, $signal: ident, $cb: ident) => {
        let wid = $wid.as_ptr();
        wid.$signal()
            .connect(&SlotNoArgs::new(wid, move || $cb(&QBox::new(wid))));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! connect_1a {
    ($base: ty, $wid: ident, $signal: ident, $slot: ty, $cb: ident) => {
        let wid = $wid.as_ptr();
        wid.$signal()
            .connect(&<$slot>::new(wid, move |arg| $cb(&QBox::new(wid), arg)));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! connect_2a {
    ($base: ty, $wid: ident, $signal: ident, $slot: ty, $cb: ident) => {
        let wid = $wid.as_ptr();
        wid.$signal().connect(&<$slot>::new(wid, move |arg1, arg2| {
            $cb(&QBox::new(wid), arg1, arg2)
        }));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! connect_3a {
    ($base: ty, $wid: ident, $signal: ident, $slot: ty, $cb: ident) => {
        let wid = $wid.as_ptr();
        wid.$signal()
            .connect(&<$slot>::new(wid, move |arg1, arg2, arg3| {
                $cb(&QBox::new(wid), arg1, arg2, arg3)
            }));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! connect_4a {
    ($base: ty, $wid: ident, $signal: ident, $slot: ty, $cb: ident) => {
        let wid = $wid.as_ptr();
        wid.$signal()
            .connect(&<$slot>::new(wid, move |arg1, arg2, arg3, arg4| {
                $cb(&QBox::new(wid), arg1, arg2, arg3, arg4)
            }));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! connect_5a {
    ($base: ty, $wid: ident, $signal: ident, $slot: ty, $cb: ident) => {
        let wid = $wid.as_ptr();
        wid.$signal()
            .connect(&<$slot>::new(wid, move |arg1, arg2, arg3, arg4, arg5| {
                $cb(&QBox::new(wid), arg1, arg2, arg3, arg4, arg5)
            }));
    };
}

pub use connect_0a;
pub use connect_1a;
pub use connect_2a;
pub use connect_3a;
pub use connect_4a;
pub use connect_5a;
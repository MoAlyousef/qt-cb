#[doc(hidden)]
#[macro_export]
macro_rules! connect_0a {
    ($base: ty, $wid: ident, $signal: ident, $cb: ident) => {
        let wid: Ptr<$base> = $wid.cast_into();
        wid.$signal().connect(&SlotNoArgs::new(wid, move || $cb(wid)));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! connect_1a {
    ($base: ty, $wid: ident, $signal: ident, $slot: ty, $cb: ident) => {
        let wid: Ptr<$base> = $wid.cast_into();
        wid.$signal().connect(&<$slot>::new(wid, move |arg| $cb(wid, arg)));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! connect_2a {
    ($base: ty, $wid: ident, $signal: ident, $slot: ty, $cb: ident) => {
        let wid: Ptr<$base> = $wid.cast_into();
        wid.$signal().connect(&<$slot>::new(wid, move |arg1, arg2| $cb(wid, arg1, arg2)));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! connect_3a {
    ($base: ty, $wid: ident, $signal: ident, $slot: ty, $cb: ident) => {
        let wid: Ptr<$base> = $wid.cast_into();
        wid.$signal().connect(&<$slot>::new(wid, move |arg1, arg2, arg3| $cb(wid, arg1, arg2, arg3)));
    };
}


pub use connect_0a;
pub use connect_1a;
pub use connect_2a;
pub use connect_3a;

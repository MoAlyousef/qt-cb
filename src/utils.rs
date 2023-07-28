#[doc(hidden)]
#[macro_export]
macro_rules! on_signal_1a {
    ($base: ty, $wid: ident, $signal: ident, $slot: ty, $cb: ident) => {
        let wid: Ptr<$base> = $wid.cast_into();
        wid.$signal().connect(&<$slot>::new(wid, move |arg| $cb(wid, arg)));
    };
}

pub use on_signal_1a;

#[doc(hidden)]
#[macro_export]
macro_rules! on_signal {
    ($base: ty, $wid: ident, $signal: ident, $cb: ident) => {
        let wid: Ptr<$base> = $wid.cast_into();
        wid.$signal().connect(&SlotNoArgs::new(wid, move || $cb(wid)));
    };
}

pub use on_signal;
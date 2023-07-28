# qt-cb

Provides several convenience closure taking methods to [rust-qt](https://github.com/rust-qt/ritual). 

## Rationale
Even though Qt's signal and slot mechanism is quite powerful and allows for very dynamic code and programming model, defining slots, especially on the Rust side, can be verbose and filled with boilerplate. 
You might just need to quickly debug something or add some simple functionality to a widget without having to go through the ritual of defining slots.

For example, compare the following 2 programs. Both programs do the same thing, the first uses this crate, and uses closures without having to define slots or deriving SlotNoArgs/SlotOfBool/SlotOfQString etc. The second doesn't use closures, it defines slots for the required signals, it requires importing SlotNoArgs/SlotOfBool/SlotOfQString boilerplate, and using the correct Slot type.

The first program (with closures):
```rust,no_run
// with closures
use qt_cb::prelude::*;
use qt_core::qs;
use qt_widgets::{
    QApplication, QCheckBox, QHBoxLayout, QLineEdit, QPushButton, QVBoxLayout, QWidget,
};

fn main() {
    QApplication::init(|_| unsafe {
        QApplication::set_style_q_string(&qs("Fusion"));
        let win = QWidget::new_0a();
        win.set_fixed_size_2a(400, 300);
        let vbox = QVBoxLayout::new_1a(&win);
        let ed = QLineEdit::new();
        ed.set_placeholder_text(&qs("Enter name"));
        ed.on_text_changed(|_ed, txt| {
            println!("current lineedit text: {}", txt.to_std_string());
        });
        vbox.add_widget(&ed);
        let hbox = QHBoxLayout::new_0a();
        vbox.add_layout_1a(&hbox);
        let checkbox = QCheckBox::new();
        hbox.add_widget(&checkbox);
        checkbox.set_text(&qs("Check me!"));
        checkbox.on_clicked(|b, checked| {
            println!(
                "{} is {}checked",
                b.text().to_std_string(),
                if checked { "" } else { "un" }
            );
        });
        let button = QPushButton::new();
        hbox.add_widget(&button);
        button.set_text(&qs("Greet!"));
        button.on_pressed(move |_b| {
            println!("Hello {}", ed.text().to_std_string());
        });
        win.show();
        QApplication::exec()
    })
}
```

The second program (no closures):
```rust,no_run
// no closures
use cpp_core::{Ptr, Ref, StaticUpcast};
use qt_core::{qs, slot, QBox, QObject, QString, SlotNoArgs, SlotOfBool, SlotOfQString};
use qt_widgets::{
    QApplication, QCheckBox, QHBoxLayout, QLineEdit, QPushButton, QVBoxLayout, QWidget,
};
use std::rc::Rc;

struct Form {
    win: QBox<QWidget>,
    ed: QBox<QLineEdit>,
    checkbox: QBox<QCheckBox>,
    button: QBox<QPushButton>,
}

impl StaticUpcast<QObject> for Form {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.win.as_ptr().static_upcast()
    }
}

impl Form {
    fn new() -> Rc<Form> {
        unsafe {
            let win = QWidget::new_0a();
            win.set_fixed_size_2a(400, 300);
            let vbox = QVBoxLayout::new_1a(&win);
            let ed = QLineEdit::new();
            ed.set_placeholder_text(&qs("Enter name"));
            vbox.add_widget(&ed);
            let hbox = QHBoxLayout::new_0a();
            vbox.add_layout_1a(&hbox);
            let checkbox = QCheckBox::new();
            hbox.add_widget(&checkbox);
            checkbox.set_text(&qs("Check me!"));
            let button = QPushButton::new();
            hbox.add_widget(&button);
            button.set_text(&qs("Greet!"));
            win.show();

            let this = Rc::new(Self {
                win,
                ed,
                checkbox,
                button,
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        self.ed
            .text_changed()
            .connect(&self.slot_on_lineedit_text_changed());
        self.button
            .pressed()
            .connect(&self.slot_on_button_pressed());
        self.checkbox
            .clicked()
            .connect(&self.slot_on_checkbox_clicked());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_button_pressed(self: &Rc<Self>) {
        println!("Hello {}", self.ed.text().to_std_string());
    }

    #[slot(SlotOfBool)]
    unsafe fn on_checkbox_clicked(self: &Rc<Self>, checked: bool) {
        println!(
            "{} is {}checked",
            self.checkbox.text().to_std_string(),
            if checked { "" } else { "un" }
        );
    }

    #[slot(SlotOfQString)]
    unsafe fn on_lineedit_text_changed(self: &Rc<Self>, txt: Ref<QString>) {
        println!("current lineedit text: {}", txt.to_std_string());
    }
}

fn main() {
    QApplication::init(|_| unsafe {
        let _form = Form::new();
        QApplication::exec()
    })
}
```

Notice the:
```rust,ignore
    ed.on_text_changed(|_ed, txt| {
        println!("current lineedit text: {}", txt.to_std_string());
    });
    checkbox.on_clicked(|b, checked| {
        println!(
            "{} is {}checked",
            b.text().to_std_string(),
            if checked { "" } else { "un" }
        );
    });
    button.on_pressed(move |_b| {
        println!("Hello {}", ed.text().to_std_string());
    });
```

Instead of:
```rust,ignore
    self.ed
        .text_changed()
        .connect(&self.slot_on_lineedit_text_changed());
    self.button
        .pressed()
        .connect(&self.slot_on_button_pressed());
    self.checkbox
        .clicked()
        .connect(&self.slot_on_checkbox_clicked());

    #[slot(SlotNoArgs)]
    unsafe fn on_button_pressed(self: &Rc<Self>) {
        println!("Hello {}", self.ed.text().to_std_string());
    }

    #[slot(SlotOfBool)]
    unsafe fn on_checkbox_clicked(self: &Rc<Self>, checked: bool) {
        println!(
            "{} is {}checked",
            self.checkbox.text().to_std_string(),
            if checked { "" } else { "un" }
        );
    }

    #[slot(SlotOfQString)]
    unsafe fn on_lineedit_text_changed(self: &Rc<Self>, txt: Ref<QString>) {
        println!("current lineedit text: {}", txt.to_std_string());
    }
```

## Implementation details
qt-cb uses functionality already provided in rust-qt. It instantiates the necessary Slot, and passes a closure directly to it. It also passes the widget itself into the closure, so you get access to the widget as well in your closure:
```rust,ignore
impl InputExt for QBox<QLineEdit> {
    unsafe fn on_text_changed<F: FnMut(&Self, Ref<QString>) + 'static>(&self, mut cb: F) {
        let wid = self.as_ptr();
        wid.text_changed()
            .connect(&SlotOfQString::new(wid, move |b| {
                cb(&QBox::new(wid), b);
            }));
    }
}
```

## Usage
```toml
[dependencies]
qt-cb = "0.1.0"
# other rust-qt packages
qt_core = "0.5.0"
qt_widgets = "0.5.0"
# other deps
```

## Requirements
This crate has the same requirements as rust-qt.
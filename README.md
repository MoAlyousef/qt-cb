# qt-cb

This crate provides several traits which define convenience methods for [qt_widgets](https://github.com/rust-qt/ritual) types. These methods take the form `connect_<signal>` which take a closure. The closure's arguments are the widget invoking the signal, as well as the signal's arguments.
If you're familiar with the Gtk-rs api, this should be familiar to you as well.

In essence, it instantiates the necessary Slot, and passes a closure directly to it. It also passes the widget itself into the closure, so you get access to the widget as well in your closure.

i.e it gives you:
```rust,ignore
checkbox.connect_clicked(|checkbox, checked| {
    // access checkbox
    // handle signal argument checked
});
```

instead of either:
```rust,ignore
checkbox.clicked().connect(&SlotOfBool::new({ 
    let checkbox = checkbox.as_ptr(); 
    move |checked| {
        // access checkbox
        // handle signal argument checked
    }
}));
```
or:
```rust,ignore
struct SomeStruct {
    checkbox: QBox<QCheckBox>,
}

impl StaticUpcast<QObject> for SomeStruct {
    // snipped
}

impl SomeStruct {
    unsafe fn init(self: &Rc<Self>) {
        self.checkbox.clicked().connect(&self.slot_on_checkbox_clicked());
    }
    #[slot(SlotOfBool)]
    unsafe fn connect_checkbox_clicked(self: &Rc<Self>, checked: bool) {
        // access self.checkbox
        // handle signal argument checked
    }
}
```

## Requirements
This crate has the same requirements as rust-qt.

## Usage
```toml
[dependencies]
qt-cb = "0.1.0"
# other rust-qt packages
qt_core = "0.5.0"
qt_widgets = "0.5.0"
# other deps
```

In your Rust source code, `use qt_cb::prelude::*;`:

```rust,no_run
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
        ed.connect_text_changed(|_ed, txt| {
            println!("current lineedit text: {}", txt.to_std_string());
        });
        vbox.add_widget(&ed);
        let hbox = QHBoxLayout::new_0a();
        vbox.add_layout_1a(&hbox);
        let checkbox = QCheckBox::new();
        hbox.add_widget(&checkbox);
        checkbox.set_text(&qs("Check me!"));
        checkbox.connect_clicked(|b, checked| {
            println!(
                "{} is {}checked",
                b.text().to_std_string(),
                if checked { "" } else { "un" }
            );
        });
        let button = QPushButton::new();
        hbox.add_widget(&button);
        button.set_text(&qs("Greet!"));
        button.connect_pressed(move |_b| {
            println!("Hello {}", ed.text().to_std_string());
        });
        win.show();
        QApplication::exec()
    })
}
```

## Rationale
Defining slots, especially on the Rust side, can be verbose. 
You might just need to quickly debug something or add some simple functionality to a widget without having to go through the ritual of defining slots.
There are 2 main ways of defining slots in rust-qt, described briefly in the intro:
(Both following programs do the same thing as the first program in the README which is using this crate)

1- Using the slot macro, this requires defining a type which also implements `StaticUpcast<QObject>`:
```rust,no_run
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

2- The second way of defining slots, is to instantiate Slot objects, which correspond to the arguments provided by the Signal, i.e SlotOfBool, SlotOfQString etc. The closure doesn't expose the widget in the args, so it needs to be captured manually (either using an Rc smart pointer, or by getting the `Ptr<Widget>`):
```rust,no_run
use qt_core::{qs, SlotNoArgs, SlotOfBool, SlotOfQString};
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
        ed.text_changed().connect(&SlotOfQString::new(&ed, |txt| {
            println!("current lineedit text: {}", txt.to_std_string());
        }));
        vbox.add_widget(&ed);
        let hbox = QHBoxLayout::new_0a();
        vbox.add_layout_1a(&hbox);
        let checkbox = QCheckBox::new();
        hbox.add_widget(&checkbox);
        checkbox.set_text(&qs("Check me!"));
        checkbox.clicked().connect(&SlotOfBool::new(&checkbox, {
            let checkbox = checkbox.as_ptr();
            move |checked| {
                println!(
                    "{} is {}checked",
                    checkbox.text().to_std_string(),
                    if checked { "" } else { "un" }
                );
            }
        }));
        let button = QPushButton::new();
        hbox.add_widget(&button);
        button.set_text(&qs("Greet!"));
        button.pressed().connect(&SlotNoArgs::new(&button, move || {
            println!("Hello {}", ed.text().to_std_string());
        }));
        win.show();
        QApplication::exec()
    })
}
```


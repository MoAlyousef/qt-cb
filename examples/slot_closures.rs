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

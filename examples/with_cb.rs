use qt_core::qs;
use qt_widgets::{
    QApplication, QCheckBox, QHBoxLayout, QLineEdit, QPushButton, QVBoxLayout, QWidget,
};
use qt_cb::prelude::ButtonExt;

fn main() {
    QApplication::init(|_| unsafe {
        QApplication::set_style_q_string(&qs("Fusion"));
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

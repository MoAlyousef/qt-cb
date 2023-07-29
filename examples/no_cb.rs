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
    unsafe fn connect_button_pressed(self: &Rc<Self>) {
        println!("Hello {}", self.ed.text().to_std_string());
    }

    #[slot(SlotOfBool)]
    unsafe fn connect_checkbox_clicked(self: &Rc<Self>, checked: bool) {
        println!(
            "{} is {}checked",
            self.checkbox.text().to_std_string(),
            if checked { "" } else { "un" }
        );
    }

    #[slot(SlotOfQString)]
    unsafe fn connect_lineedit_text_changed(self: &Rc<Self>, txt: Ref<QString>) {
        println!("current lineedit text: {}", txt.to_std_string());
    }
}

fn main() {
    QApplication::init(|_| unsafe {
        let _form = Form::new();
        QApplication::exec()
    })
}

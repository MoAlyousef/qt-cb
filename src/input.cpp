#include "input.h"
#include <QLineEdit>
#include <QWidget>

void QLineEdit_onTextChanged(QLineEdit *s, void (*cb)(QLineEdit *, void *, void *), void *data) {
    QLineEdit::connect(s, &QLineEdit::textChanged,
                       [=](const QString &st) { cb(s, (void *)&st, data); });
}

void QLineEdit_onReturnPressed(QLineEdit *s, void (*cb)(QLineEdit *, void *), void *data) {
    QLineEdit::connect(s, &QLineEdit::returnPressed, [=]() { cb(s, data); });
}
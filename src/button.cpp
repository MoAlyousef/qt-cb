#include "button.h"
#include <QWidget>
#include <QAbstractButton>

void QAbstractButton_onClicked(QAbstractButton *btn, void (*cb)(QAbstractButton *, int, void *), void *data) {          
    QAbstractButton::connect(btn, &QAbstractButton::clicked, [=](bool checked) { cb(btn, checked, data); });
}                                                                                              

void QAbstractButton_onPressed(QAbstractButton *btn, void (*cb)(QAbstractButton *, void *), void *data) {
    QAbstractButton::connect(btn, &QAbstractButton::pressed, [=]() { cb(btn, data); });
}
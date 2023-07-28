#include "button.h"
#include <QAbstractButton>
#include <QWidget>

void QAbstractButton_onClicked(QAbstractButton *btn, void (*cb)(QAbstractButton *, int, void *),
                               void *data) {
    QAbstractButton::connect(btn, &QAbstractButton::clicked,
                             [=](bool checked) { cb(btn, checked, data); });
}

void QAbstractButton_onPressed(QAbstractButton *btn, void (*cb)(QAbstractButton *, void *),
                               void *data) {
    QAbstractButton::connect(btn, &QAbstractButton::pressed, [=]() { cb(btn, data); });
}

void QAbstractButton_onToggled(QAbstractButton *btn, void (*cb)(QAbstractButton *, int, void *),
                               void *data) {
    QAbstractButton::connect(btn, &QAbstractButton::toggled,
                             [=](bool checked) { cb(btn, checked, data); });
}

void QAbstractButton_onReleased(QAbstractButton *btn, void (*cb)(QAbstractButton *, void *),
                                void *data) {
    QAbstractButton::connect(btn, &QAbstractButton::released, [=]() { cb(btn, data); });
}
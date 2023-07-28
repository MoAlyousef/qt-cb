#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct QAbstractButton QAbstractButton;

void QAbstractButton_onClicked(QAbstractButton *btn, void (*)(QAbstractButton *, int, void *),
                               void *data);

void QAbstractButton_onPressed(QAbstractButton *btn, void (*)(QAbstractButton *, void *),
                               void *data);

void QAbstractButton_onToggled(QAbstractButton *btn, void (*)(QAbstractButton *, int, void *),
                               void *data);

void QAbstractButton_onReleased(QAbstractButton *btn, void (*)(QAbstractButton *, void *),
                                void *data);

#ifdef __cplusplus
}
#endif
#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct QLineEdit QLineEdit;

void QLineEdit_onTextChanged(QLineEdit *s, void (*)(QLineEdit *, void *, void *), void *data);

void QLineEdit_onReturnPressed(QLineEdit *s, void (*)(QLineEdit *, void *), void *data);

#ifdef __cplusplus
}
#endif

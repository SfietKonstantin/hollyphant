#pragma once

#include <QObject>

namespace hollyphant {

class ErrorFormatter : public QObject
{
public:
    explicit ErrorFormatter(QObject *parent = nullptr);
    static QString formatErrorUnexpected();
    static QString formatErrorMasApplicationRegister(const QString &instance);
    static QString formatErrorDatabase();
};

} // namespace hollyphant
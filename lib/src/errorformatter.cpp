#include "errorformatter.h"


namespace hollyphant {

ErrorFormatter::ErrorFormatter(QObject *parent)
    : QObject(parent)
{
}

QString ErrorFormatter::formatErrorUnexpected()
{
    return tr("Unexpected error. Please file a bug report");
}

QString ErrorFormatter::formatErrorMasApplicationRegister(const QString &instance)
{
    return tr("Could not register application in %1").arg(instance);
}

QString ErrorFormatter::formatErrorDatabase()
{
    return tr("Database error. This is unexpected. Please file a bug report");
}

} // namespace hollyphant

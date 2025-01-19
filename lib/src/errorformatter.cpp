#include "errorformatter.h"


namespace hollyphant {

ErrorFormatter::ErrorFormatter(QObject *parent)
    : QObject(parent)
{
}

QString ErrorFormatter::formatErrorMasApplicationRegister(const QString &instance)
{
    return tr("Could not register application in %1").arg(instance);
}

} // namespace hollyphant

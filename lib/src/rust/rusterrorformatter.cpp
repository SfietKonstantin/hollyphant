#include "rusterrorformatter.h"

#include "../errorformatter.h"

using namespace hollyphant;

rust::cxxbridge1::String format_error_unexpected()
{
    const auto formatted = ErrorFormatter::formatErrorUnexpected().toUtf8();
    return rust::cxxbridge1::String(formatted.data(), formatted.size());
}

rust::cxxbridge1::String format_error_mas_application_register(rust::cxxbridge1::Str instance)
{
    const auto qInstance = QString::fromUtf8(instance.data(), static_cast<int>(instance.size()));
    const auto formatted = ErrorFormatter::formatErrorMasApplicationRegister(qInstance).toUtf8();
    return rust::cxxbridge1::String(formatted.data(), formatted.size());
}

rust::cxxbridge1::String format_error_database()
{
    const auto formatted = ErrorFormatter::formatErrorDatabase().toUtf8();
    return rust::cxxbridge1::String(formatted.data(), formatted.size());
}
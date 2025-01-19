#include <hollyphant/statusitem.h>

namespace hollyphant {

StatusItem::StatusItem(QObject *parent)
    : QObject(parent)
{
}

qmlext::Item *StatusItem::item() const
{
    return m_item;
}

void StatusItem::setItem(qmlext::Item *item)
{
    if (m_item != item) {
        // Disconnect from previous item
        if (m_item != nullptr) {
            m_item->disconnect(this);
        }

        m_item = item;

        // Connect to new item
        if (m_item != nullptr) {
            connect(m_item, &qmlext::Item::valueChanged, this, &StatusItem::handleValueChanged);
        }

        emit itemChanged();
    }
}

StatusItem::Status StatusItem::status() const
{
    return m_status;
}

const QVariant &StatusItem::value() const
{
    return m_value;
}

const QString &StatusItem::message() const
{
    return m_message;
}

const QString &StatusItem::details() const
{
    return m_details;
}

void StatusItem::handleValueChanged()
{
    if (m_item == nullptr) {
        return;
    }

    const auto value = m_item->value().toMap();
    if (value.contains("status")) {
        const auto status = value.value("status").toString();
        if (status == "in-progress") {
            setStatus(InProgress);
        } else if (status == "success") {
            setStatus(Success);
        } else if (status == "error") {
            setStatus(Error);
        } else {
            setStatus(Unknown);
        }
    }

    if (value.contains("value")) {
        setValue(value.value("value"));
    } else {
        setValue(QVariant());
    }

    if (value.contains("message")) {
        setMessage(value.value("message").toString());
    } else {
        setMessage(QString());
    }

    if (value.contains("details")) {
        setDetails(value.value("details").toString());
    } else {
        setDetails(QString());
    }
}

void StatusItem::setStatus(Status status)
{
    if (m_status != status) {
        m_status = status;
        emit statusChanged();
    }
}

void StatusItem::setValue(QVariant value)
{
    if (m_value != value) {
        m_value = std::move(value);
        emit valueChanged();
    }
}

void StatusItem::setMessage(QString message)
{
    if (m_message != message) {
        m_message = std::move(message);
        emit messageChanged();
    }
}

void StatusItem::setDetails(QString details)
{
    if (m_details != details) {
        m_details = std::move(details);
        emit detailsChanged();
    }
}

} // namespace hollyphant

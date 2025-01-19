#include <hollyphant/statusitem.h>
#include <qmlext/listmodel.h>

namespace hollyphant {

StatusItem::StatusItem(QObject *parent)
    : QObject(parent)
{
}

QObject *StatusItem::item() const
{
    return m_item;
}

void StatusItem::setItem(QObject *item)
{
    if (m_item != item) {
        // Disconnect from previous item
        if (m_item != nullptr) {
            m_item->disconnect(this);
        }

        m_item = item;

        // Connect to new item
        if (m_item != nullptr) {
            auto *extItem = qobject_cast<qmlext::Item *>(m_item);
            if (extItem != nullptr) {
                connect(extItem, &qmlext::Item::valueChanged, this, &StatusItem::handleItemValueChanged);
            }
            auto *extListModel = qobject_cast<qmlext::ListModel *>(m_item);
            if (extListModel != nullptr) {
                connect(extListModel, &qmlext::ListModel::valueChanged, this, &StatusItem::handleListModelValueChanged);
            }
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

void StatusItem::handleItemValueChanged()
{
    auto extItem = qobject_cast<qmlext::Item *>(m_item);
    if (extItem == nullptr) {
        return;
    }

    handleValueChanged(extItem->value());
}

void StatusItem::handleListModelValueChanged()
{
    auto extListModel = qobject_cast<qmlext::ListModel *>(m_item);
    if (extListModel == nullptr) {
        return;
    }

    handleValueChanged(extListModel->value());
}

void StatusItem::handleValueChanged(const QVariant &value)
{
    const auto mapValue = value.toMap();
    if (mapValue.contains("status")) {
        const auto status = mapValue.value("status").toString();
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

    if (mapValue.contains("value")) {
        setValue(mapValue.value("value"));
    } else {
        setValue(QVariant());
    }

    if (mapValue.contains("message")) {
        setMessage(mapValue.value("message").toString());
    } else {
        setMessage(QString());
    }

    if (mapValue.contains("details")) {
        setDetails(mapValue.value("details").toString());
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

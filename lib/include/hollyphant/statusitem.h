#pragma once

#include <qmlext/item.h>

namespace hollyphant {

class StatusItem : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QObject *item READ item WRITE setItem NOTIFY itemChanged)
    Q_PROPERTY(Status status READ status NOTIFY statusChanged)
    Q_PROPERTY(QVariant value READ value NOTIFY valueChanged)
    Q_PROPERTY(QString message READ message NOTIFY messageChanged)
    Q_PROPERTY(QString details READ details NOTIFY detailsChanged)
public:
    enum Status {
        Unknown, //
        InProgress,
        Success,
        Error
    };
    Q_ENUM(Status)

    explicit StatusItem(QObject *parent = nullptr);
    QObject *item() const;
    Q_INVOKABLE void setItem(QObject *item);
    Status status() const;
    const QVariant &value() const;
    const QString &message() const;
    const QString &details() const;

signals:
    void itemChanged();
    void statusChanged();
    void valueChanged();
    void messageChanged();
    void detailsChanged();

private:
    void handleItemValueChanged();
    void handleListModelValueChanged();
    void handleValueChanged(const QVariant &value);
    void setStatus(Status status);
    void setValue(QVariant value);
    void setMessage(QString message);
    void setDetails(QString details);
    QObject *m_item{nullptr};
    Status m_status{Unknown};
    QVariant m_value;
    QString m_message;
    QString m_details;
};

} // namespace hollyphant
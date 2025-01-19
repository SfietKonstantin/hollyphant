#include <hollyphant/main.h>

#include "../loggingeventprocessor.h"
#include <QTimer>
#include <utility>

using namespace qmlext;
using namespace qmlext::event;

namespace hollyphant {

class DelayedPublisher : public QObject
{
    Q_OBJECT
public:
    DelayedPublisher(EventProcessor::EventPublisher publisher, Event event, QVariant key, QVariant value)
        : QObject(nullptr)
        , m_publisher(std::move(publisher))
        , m_event(event)
        , m_key(std::move(key))
        , m_value(std::move(value))
    {
    }

    Q_INVOKABLE void trigger()
    {
        m_publisher(m_event, m_key, m_value);
        deleteLater();
    }

private:
    EventProcessor::EventPublisher m_publisher;
    Event m_event;
    QVariant m_key;
    QVariant m_value;
};

namespace {

class ShimEventProcessor : public EventProcessor
{
public:
    void execute(EventPublisher eventPublisher, const QVariant &key, const QVariant &args) override
    {
        if (key == QVariantMap{{"context", "init"}}) {
            auto publisher = new DelayedPublisher(std::move(eventPublisher), Event(EventType::Set), key,
                                                  QVariantMap{{"status", "success"}, {"value", "no-account"}});
            QTimer::singleShot(500, publisher, &DelayedPublisher::trigger);
            return;
        }
        if (key == QVariantMap{{"context", "new-account"}, {"service", "mastodon"}, {"action", "open-browser"}}) {
            eventPublisher(Event(EventType::Set), key, QVariantMap{{"status", "in-progress"}});

            auto publisher =
                    new DelayedPublisher(std::move(eventPublisher), Event(EventType::Set), key,
                                         QVariantMap{{"status", "success"}, {"value", "https://mastodon.social"}});
            QTimer::singleShot(1000, publisher, &DelayedPublisher::trigger);
            return;
        }
    }
};

} // namespace

std::unique_ptr<EventProcessor> createEventProcessor()
{
    return std::make_unique<LoggingEventProcessor>(std::make_unique<ShimEventProcessor>());
}

} // namespace hollyphant

#include "main.moc"
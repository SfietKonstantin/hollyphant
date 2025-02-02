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

void publishSet(const EventProcessor::EventPublisher &eventPublisher, const QVariant &key, const QVariant &value)
{
    eventPublisher(Event(EventType::Set), key, value);
}

void delayPublish(const EventProcessor::EventPublisher &eventPublisher, Event event, QVariant key, QVariant value)
{
    auto publisher = new DelayedPublisher(eventPublisher, std::move(event), std::move(key), std::move(value));
    QTimer::singleShot(1000, publisher, &DelayedPublisher::trigger);
}

void delayPublishSet(const EventProcessor::EventPublisher &eventPublisher, QVariant key, QVariant value)
{
    delayPublish(eventPublisher, Event(EventType::Set), std::move(key), std::move(value));
}

void delayPublishAppend(const EventProcessor::EventPublisher &eventPublisher, QVariant key, QVariant value)
{
    delayPublish(eventPublisher, Event(EventType::Append), std::move(key), std::move(value));
}

void publishInProgress(const EventProcessor::EventPublisher &eventPublisher, const QVariant &key)
{
    publishSet(eventPublisher, key, QVariantMap{{"status", "in-progress"}});
}

void delayPublishTestError(const EventProcessor::EventPublisher &eventPublisher, const QVariant &key)
{
    delayPublish(eventPublisher, Event(EventType::Set), key,
                 QVariantMap{{"status", "error"},
                             {"message", "This is a test error"},
                             {"details", "This is a long test error description.\nSome error happened."}});
}

class ShimEventProcessor : public EventProcessor
{
public:
    void execute(EventPublisher eventPublisher, const QVariant &key, const QVariant &args) override
    {
        auto accountsKey = QVariantMap{{"context", "accounts"}, {"action", "list"}};

        if (key == "test") {
            delayPublishTestError(eventPublisher, key);
            return;
        }
        if (key == QVariantMap{{"context", "init"}}) {
            delayPublishSet(eventPublisher, key, QVariantMap{{"status", "success"}, {"value", "no-account"}});
            return;
        }
        if (key == accountsKey) {
            publishInProgress(eventPublisher, key);
            delayPublishSet(eventPublisher, key, QVariantMap{{"status", "success"}});
            return;
        }
        if (key == QVariantMap{{"context", "new-account"}, {"service", "mastodon"}, {"action", "prelogin"}}) {
            publishInProgress(eventPublisher, key);

            delayPublishSet(
                    eventPublisher, key,
                    QVariantMap{{"status", "success"}, {"value", QVariantMap{{"url", "https://mastodon.social"}}}});
            return;
        }
        if (key == QVariantMap{{"context", "new-account"}, {"service", "mastodon"}, {"action", "login"}}) {
            publishInProgress(eventPublisher, key);

            delayPublishSet(eventPublisher, key, QVariantMap{{"status", "success"}, {"value", QVariant()}});

            auto newAccount = QVariantMap{{"kind", "mastodon"},
                                          {"name", args.toMap()["name"]},
                                          {"instance", "https://mastodon.social"}};
            delayPublishAppend(eventPublisher, accountsKey, newAccount);
            return;
        }
    }
};

} // namespace

void initHollyphant()
{
}

std::unique_ptr<EventProcessor> createEventProcessor()
{
    return std::make_unique<LoggingEventProcessor>(std::make_unique<ShimEventProcessor>());
}

} // namespace hollyphant

#include "main.moc"

#include "loggingeventprocessor.h"

#include <QLoggingCategory>

Q_DECLARE_LOGGING_CATEGORY(logEventProcessor)
Q_LOGGING_CATEGORY(logEventProcessor, "hollyphant.event.processor")

using namespace qmlext::event;

namespace hollyphant {

namespace {

class LoggingEventPublisher
{
public:
    explicit LoggingEventPublisher(EventProcessor::EventPublisher publisher)
        : m_publisher(std::move(publisher))
    {
    }

    void operator()(const Event &event, const QVariant &key, const QVariant &value) const
    {
        qCDebug(logEventProcessor) << "Sending event" << static_cast<int>(event.type()) << key << value;
        m_publisher(event, key, value);
    }

private:
    EventProcessor::EventPublisher m_publisher;
};

} // namespace

LoggingEventProcessor::LoggingEventProcessor(std::unique_ptr<EventProcessor> underlying)
    : m_underlying(std::move(underlying))
{
}

void LoggingEventProcessor::execute(EventPublisher eventPublisher, const QVariant &key, const QVariant &args)
{
    qDebug(logEventProcessor) << "Execute requested" << key << args;
    auto loggingPublisher = LoggingEventPublisher(std::move(eventPublisher));
    m_underlying->execute(std::move(loggingPublisher), key, args);
}

} // namespace hollyphant

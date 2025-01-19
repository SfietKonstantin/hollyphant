#pragma once

#include <memory>
#include <qmlext/event/eventprocessor.h>

namespace hollyphant {

class LoggingEventProcessor : public qmlext::event::EventProcessor
{
public:
    explicit LoggingEventProcessor(std::unique_ptr<EventProcessor> underlying);
    void execute(EventPublisher eventPublisher, const QVariant &key, const QVariant &args) override;

private:
    std::unique_ptr<EventProcessor> m_underlying;
};

} // namespace hollyphant

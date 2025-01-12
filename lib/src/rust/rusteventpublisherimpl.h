#pragma once

#include <qmlext/event/json/jsoneventprocessor.h>

class RustEventPublisherImpl
{
public:
    explicit RustEventPublisherImpl(qmlext::event::json::JsonEventProcessor::EventPublisher eventPublisher);
    qmlext::event::json::JsonEventProcessor::EventPublisher m_eventPublisher;
};
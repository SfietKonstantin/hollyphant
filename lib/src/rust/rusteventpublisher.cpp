#include "rusteventpublisher.h"
#include "rusteventpublisherimpl.h"

using namespace qmlext::event;
using namespace qmlext::event::json;

namespace {

void publish(JsonEventProcessor::EventPublisher &publisher, const Event &event, const std::string &key,
             const std::string &value)
{
    publisher(event, QByteArray::fromStdString(key), QByteArray::fromStdString(value));
}

} // namespace

RustEventPublisherImpl::RustEventPublisherImpl(JsonEventProcessor::EventPublisher eventPublisher)
    : m_eventPublisher(std::move(eventPublisher))
{
}

RustEventPublisher::RustEventPublisher(std::unique_ptr<RustEventPublisherImpl> impl)
    : m_impl(std::move(impl))
{
}

void RustEventPublisher::publish_set(const std::string &key, const std::string &value) const
{
    publish(m_impl->m_eventPublisher, Event(EventType::Set), key, value);
}


RustEventPublisher::~RustEventPublisher() = default;

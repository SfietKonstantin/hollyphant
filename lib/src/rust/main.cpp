#include <hollyphant/main.h>

#include "../loggingeventprocessor.h"
#include "rusteventpublisher.h"
#include "rusteventpublisherimpl.h"
#include <QDebug>
#include <QStandardPaths>
#include <hollyphant-ffi/src/lib.rs.h>
#include <qmlext/event/json/jsoneventadapter.h>
#include <qmlext/event/json/jsoneventprocessor.h>

using namespace qmlext;
using namespace qmlext::event;
using namespace qmlext::event::json;
using namespace rust;

namespace hollyphant {

namespace {

String findDatabasePath()
{
    auto dbPath = QStandardPaths::locate(QStandardPaths::AppDataLocation, "hollyphant.db").toUtf8();
    return String(dbPath.data(), dbPath.size());
}

class RustEventProcessorWrapper : public JsonEventProcessor
{
public:
    explicit RustEventProcessorWrapper()
        : m_processor(hollyphant_event_processor_new(findDatabasePath()))
    {
    }
    void execute(EventPublisher eventPublisher, QByteArray key, QByteArray args) override
    {
        auto rustPublisherImpl = std::make_unique<RustEventPublisherImpl>(eventPublisher);
        m_processor->execute(std::make_unique<RustEventPublisher>(std::move(rustPublisherImpl)), key.toStdString(),
                             args.toStdString());
    }

private:
    Box<RustEventProcessor> m_processor;
};

} // namespace

void initHollyphant()
{
    hollyphant_init();
}


std::unique_ptr<EventProcessor> createEventProcessor()
{
    auto processor = std::make_unique<RustEventProcessorWrapper>();
    return std::make_unique<LoggingEventProcessor>(std::make_unique<JsonEventAdapter>(std::move(processor)));
}

} // namespace hollyphant

#include <hollyphant/main.h>

#include <QDebug>
#include <hollyphant/hollyphant.h>

using namespace qmlext;
using namespace qmlext::event;

namespace hollyphant {

namespace {

class ShimEventProcessor : public EventProcessor
{
public:
    void execute(EventPublisher eventPublisher, const QVariant &key, const QVariant &args) override
    {
        qDebug() << "Execute requested" << key << args;
        if (key == QVariantMap{{"action", "init"}}) {
            eventPublisher(Event(EventType::Set), key, "NoAccount");
        }
    }
};

} // namespace

std::unique_ptr<EventProcessor> createEventProcessor()
{
    return std::make_unique<ShimEventProcessor>();
}

} // namespace hollyphant

#include <hollyphant/main.h>

using namespace qmlext;
using namespace qmlext::event;

namespace hollyphant {

namespace {

class ShimEventProcessor : public EventProcessor
{
public:
    void execute(EventPublisher eventPublisher, const QVariant &id, const QVariant &args) override
    {
    }
};

} // namespace

std::unique_ptr<EventProcessor> createEventProcessor()
{
    return std::make_unique<ShimEventProcessor>();
}

} // namespace hollyphant
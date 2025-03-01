#pragma once

#include <memory>
#include <qmlext/event/eventprocessor.h>

namespace hollyphant {

void initHollyphant();
std::unique_ptr<qmlext::event::EventProcessor> createEventProcessor();

} // namespace hollyphant
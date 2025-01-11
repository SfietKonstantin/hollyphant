#pragma once

#include <QtCore/QObject>

namespace hollyphant {

class Hollyphant
{
    Q_GADGET
public:
    enum class InitialState {
        NoAccount,
        HasAccount
    };
    Q_ENUM(InitialState)

    Hollyphant() = delete;
};

} // namespace hollyphant

#include <QtQml/QQmlEngine>
#include <QtQml/qqml.h>
#include <hollyphant/main.h>
#include <qmlext/eventbus.h>
#include <qmlext/item.h>

#ifdef WITH_SAILFISH
#include <sailfishapp.h>
#endif

static QObject *createEventBus(QQmlEngine *engine, QJSEngine *jsEngine)
{
    Q_UNUSED(jsEngine)
    return new qmlext::EventBus(hollyphant::createEventProcessor(), engine);
}

int main(int argc, char *argv[])
{
    qmlRegisterType<qmlext::Item>("harbour.hollyphant", 1, 0, "ExtItem");
    qmlRegisterSingletonType<qmlext::EventBus>("harbour.hollyphant", 1, 0, "ExtEventBus", createEventBus);

#ifdef WITH_SAILFISH
    return SailfishApp::main(argc, argv);
#else
    Q_UNUSED(argc)
    Q_UNUSED(argv)
    return 0;
#endif
}

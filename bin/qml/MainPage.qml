import QtQuick 2.0
import Sailfish.Silica 1.0
import harbour.hollyphant 1.0

Page {
    id: container

    Welcome {
        visible: hasAccountStatusItem.value === "no-account"
    }

    PageBusyIndicator {
        running: initialStatusItem.status !== StatusItem.Success
    }

    StatusItem {
        id: hasAccountStatusItem
        item: ValueItem {
            eventBus: EventBus
            key: {
                "context": "init"
            }

            Component.onCompleted: execute()
        }
    }
}

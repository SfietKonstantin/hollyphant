import QtQuick 2.0
import Sailfish.Silica 1.0
import harbour.hollyphant 1.0
import "pages"

ApplicationWindow {
    // cover: Qt.resolvedUrl("cover/CoverPage.qml")
    allowedOrientations: Orientation.All

    StatusItem {
        id: item
        item: ValueItem {
            eventBus: EventBus
            key: {
                "context": "init"
            }

            Component.onCompleted: execute()
        }

        onValueChanged: {
            if (value === "no-account") {
                pageStack.push(welcomePage, {}, PageStackAction.Immediate)
            }
        }
    }

    Component {
        id: welcomePage
        WelcomePage {}
    }
}

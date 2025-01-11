import QtQuick 2.0
import Sailfish.Silica 1.0
import harbour.hollyphant 1.0
import "pages"
import "constants/states.js" as States

ApplicationWindow {
    // cover: Qt.resolvedUrl("cover/CoverPage.qml")
    allowedOrientations: Orientation.All

    ExtItem {
        id: item
        eventBus: EventBus
        key: {
            "action": "init"
        }
        onValueChanged: {
            if (value === States.NoAccount) {
                pageStack.push(welcomePage, {}, PageStackAction.Immediate)
            }
        }

        Component.onCompleted: {
            item.execute("load")
        }
    }

    Component {
        id: welcomePage
        WelcomePage {}
    }
}

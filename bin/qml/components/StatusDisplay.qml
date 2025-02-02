import QtQuick 2.0
import harbour.hollyphant 1.0

Item {
    id: container
    property StatusItem statusItem
    default property alias content: successItem.children
    height: childrenRect.height

    ErrorItem {
        visible: container.statusItem.status === StatusItem.Error
        statusItem: container.statusItem
    }

    Item {
        id: successItem
        anchors.left: parent.left
        anchors.right: parent.right
        height: childrenRect.height
        visible: container.statusItem.status === StatusItem.Success
    }
}

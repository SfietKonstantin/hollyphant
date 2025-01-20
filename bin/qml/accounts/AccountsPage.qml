import QtQuick 2.0
import Sailfish.Silica 1.0
import harbour.hollyphant 1.0

Page {
    id: container

    SilicaListView {
        anchors.fill: parent
        header: PageHeader {
            title: qsTr("Accounts")
        }
        model: ValueListModel {
            id: listModel
            eventBus: EventBus
            key: {
                "context": "accounts",
                "action": "list"
            }
            Component.onCompleted: execute()
        }

        ViewPlaceholder {
            enabled: modelStatusItem.status !== StatusItem.InProgress
                     && listModel.count === 0
            text: qsTr("No account")
            hintText: qsTr("Pull down to add an account")
        }

        PullDownMenu {
            visible: modelStatusItem.status !== StatusItem.InProgress
            MenuItem {
                text: qsTr("Add account")
                onClicked: {
                    pageStack.push(Qt.resolvedUrl("NewAccountDialog.qml"))
                }
            }
        }
    }

    StatusItem {
        id: modelStatusItem
        item: listModel
    }
}

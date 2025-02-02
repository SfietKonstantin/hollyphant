import QtQuick 2.0
import Sailfish.Silica 1.0
import harbour.hollyphant 1.0

Page {
    id: accountsPage

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

        delegate: ListItem {
            contentHeight: Theme.itemSizeMedium

            Label {
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.leftMargin: Theme.horizontalPageMargin
                anchors.rightMargin: Theme.horizontalPageMargin
                anchors.verticalCenter: parent.verticalCenter
                text: model.value.name
                elide: Text.ElideRight
            }
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
                    pageStack.push(newAccountDialog)
                }
            }
        }
    }

    StatusItem {
        id: modelStatusItem
        item: listModel
    }

    Component {
        id: newAccountDialog

        NewAccountDialog {}
    }
}

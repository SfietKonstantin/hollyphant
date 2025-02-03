import QtQuick 2.0
import Sailfish.Silica 1.0
import harbour.hollyphant 1.0
import "../components"

Dialog {
    id: container
    function load() {
        var args = {
            "name": nameField.text,
            "code": codeField.text
        }
        loginItem.execute(args)
    }

    function _handleAccept() {
        if (loginStatus.status === StatusItem.Success
                && status == DialogStatus.Opened) {
            canAccept = true
            accept()
        }
    }

    canAccept: false
    acceptDestination: accountsPage
    acceptDestinationAction: PageStackAction.Pop

    onStatusChanged: _handleAccept()

    BusyLabel {
        text: "Logging into Mastodon"
        running: loginStatus.status === StatusItem.InProgress
    }

    SilicaFlickable {
        anchors.fill: parent
        contentHeight: column.height
        visible: loginStatus.status === StatusItem.Error

        Column {
            id: column
            anchors.left: parent.left
            anchors.right: parent.right
            spacing: Theme.paddingMedium

            DialogHeader {
                title: qsTr("New Mastodon account")
            }

            ErrorItem {
                statusItem: loginStatus
            }
        }
    }

    StatusItem {
        id: loginStatus
        onStatusChanged: container._handleAccept()
        item: ValueItem {
            id: loginItem
            eventBus: EventBus
            key: {
                "context": "new-account",
                "service": "mastodon",
                "action": "login"
            }
        }
    }
}

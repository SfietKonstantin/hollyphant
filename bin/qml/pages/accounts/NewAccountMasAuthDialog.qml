import QtQuick 2.0
import Sailfish.Silica 1.0
import harbour.hollyphant 1.0

Dialog {
    id: container
    function load(instance) {
        console.debug("Load triggered")
        masOpenBrowserItem.execute(instance)
    }

    canAccept: openBrowserStatus.status === StatusItem.Success
               && codeField.text.length > 0

    BusyLabel {
        text: "Registering application in Mastodon instance"
        running: openBrowserStatus.status !== StatusItem.Success
                 && openBrowserStatus.status !== StatusItem.Error
    }

    SilicaFlickable {
        anchors.fill: parent
        contentHeight: column.height
        visible: openBrowserStatus.status === StatusItem.Success

        Column {
            id: column
            anchors.left: parent.left
            anchors.right: parent.right
            spacing: Theme.paddingMedium

            DialogHeader {
                id: header
                dialog: container
                title: qsTr("New account")
            }

            Label {
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.leftMargin: Theme.horizontalPageMargin
                anchors.rightMargin: Theme.horizontalPageMargin

                text: qsTr("Please enter the code you have received from Mastodon in the field below")
                wrapMode: Text.WordWrap
            }

            TextField {
                id: codeField
                label: qsTr("Login code")
                placeholderText: qsTr("Login code")
            }
        }

        ScrollDecorator {}
    }

    StatusItem {
        id: openBrowserStatus
        item: ValueItem {
            id: masOpenBrowserItem
            eventBus: EventBus
            key: {
                "context": "new-account",
                "service": "mastodon",
                "action": "open-browser"
            }
        }
        onValueChanged: {
            Qt.openUrlExternally(value)
        }
    }
}

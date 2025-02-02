import QtQuick 2.0
import Sailfish.Silica 1.0
import harbour.hollyphant 1.0
import "../components"

Dialog {
    id: container
    property string name
    property string instance

    function load() {
        var args = {
            "name": nameField.text,
            "instance": masInstanceField.text
        }
        preLoginItem.execute(args)
    }

    canAccept: preLoginStatus.status === StatusItem.Success
               && codeField.text.length > 0
    acceptDestination: finalizationPage
    onAccepted: acceptDestinationInstance.load()

    BusyLabel {
        text: "Registering application in Mastodon instance"
        running: preLoginStatus.status === StatusItem.InProgress
    }

    SilicaFlickable {
        anchors.fill: parent
        contentHeight: column.height
        visible: preLoginStatus.status !== StatusItem.InProgress

        Column {
            id: column
            anchors.left: parent.left
            anchors.right: parent.right
            spacing: Theme.paddingMedium

            DialogHeader {
                id: header
                dialog: container
                title: qsTr("New Mastodon account")
            }

            StatusDisplay {
                anchors.left: parent.left
                anchors.right: parent.right
                statusItem: preLoginStatus

                Column {
                    anchors.left: parent.left
                    anchors.right: parent.right
                    spacing: Theme.paddingMedium

                    Label {
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.leftMargin: Theme.horizontalPageMargin
                        anchors.rightMargin: Theme.horizontalPageMargin
                        color: Theme.highlightColor

                        text: qsTr("Please enter the code you have received from Mastodon in the field below")
                        wrapMode: Text.WordWrap
                    }

                    TextField {
                        id: codeField
                        label: qsTr("Login code")
                        placeholderText: qsTr("Login code")
                        inputMethodHints: Qt.ImhDigitsOnly
                        focus: true

                        EnterKey.iconSource: "image://theme/icon-m-enter-accept"
                        EnterKey.onClicked: container.accept()
                    }
                }
            }
        }

        ScrollDecorator {}
    }

    StatusItem {
        id: preLoginStatus
        item: ValueItem {
            id: preLoginItem
            eventBus: EventBus
            key: {
                "context": "new-account",
                "service": "mastodon",
                "action": "prelogin"
            }
        }
        onValueChanged: {
            Qt.openUrlExternally(value)
        }
    }

    Component {
        id: finalizationPage
        NewAccountMasAuthFinalizationPage {}
    }
}

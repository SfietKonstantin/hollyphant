import QtQuick 2.0
import Sailfish.Silica 1.0

Dialog {
    id: container
    canAccept: {
        switch (serviceCombo.currentIndex) {
        case 0:
            return masServerField.text.length > 0
        case 1:
            return bskyUsernameField.text.length > 0
                    && bskyPasswordField.text.length > 0
        default:
            return false
        }
    }

    acceptDestination: {
        switch (serviceCombo.currentIndex) {
        case 0:
            return Qt.resolvedUrl("NewAccountMasAuthDialog.qml")
        case 1:
            return Qt.resolvedUrl("NewAccountBskyDialog.qml")
        default:
            return null
        }
    }

    onAccepted: {
        switch (serviceCombo.currentIndex) {
        case 0:
            acceptDestinationInstance.load(masServerField.text)
            break
        }
    }

    SilicaFlickable {
        anchors.fill: parent
        contentHeight: column.height

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

            ComboBox {
                id: serviceCombo
                label: qsTr("Service")

                menu: ContextMenu {
                    MenuItem {
                        text: "Mastodon"
                    }
                    MenuItem {
                        text: "Bluesky"
                    }
                }
            }

            TextField {
                id: masServerField
                visible: serviceCombo.currentIndex === 0
                label: qsTr("Mastodon server")
                placeholderText: qsTr("Mastodon server")
                EnterKey.iconSource: "image://theme/icon-m-enter-accept"
                EnterKey.onClicked: container.accept()
            }

            TextField {
                id: bskyUsernameField
                visible: serviceCombo.currentIndex === 1
                label: qsTr("Username")
                placeholderText: qsTr("Username")

                EnterKey.iconSource: "image://theme/icon-m-enter-next"
                EnterKey.onClicked: bskyPasswordField.focus = true
            }

            PasswordField {
                id: bskyPasswordField
                visible: serviceCombo.currentIndex === 1
                EnterKey.iconSource: "image://theme/icon-m-enter-accept"
                EnterKey.onClicked: container.accept()
            }
        }

        ScrollDecorator {}
    }
}

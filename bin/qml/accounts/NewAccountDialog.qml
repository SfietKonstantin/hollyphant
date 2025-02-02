import QtQuick 2.0
import Sailfish.Silica 1.0

Dialog {
    id: container
    canAccept: {
        switch (serviceCombo.currentIndex) {
        case 0:
            return nameField.text.length > 0 && masInstanceField.text.length > 0
        case 1:
            return nameField.text.length > 0
                    && bskyUsernameField.text.length > 0
                    && bskyPasswordField.text.length > 0
        default:
            return false
        }
    }

    acceptDestination: {
        switch (serviceCombo.currentIndex) {
        case 0:
            return newMasAccountDialog
        case 1:
            return Qt.resolvedUrl("NewAccountBskyDialog.qml")
        default:
            return null
        }
    }

    onAccepted: {
        switch (serviceCombo.currentIndex) {
        case 0:
            acceptDestinationInstance.load()
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

            TextField {
                id: nameField
                label: qsTr("Name")
                placeholderText: qsTr("Account name")
                focus: true

                EnterKey.iconSource: "image://theme/icon-m-enter-next"
                EnterKey.onClicked: {
                    switch (serviceCombo.currentIndex) {
                    case 0:
                        masInstanceField.focus = true
                        break
                    case 1:
                        bskyUsernameField.focus = true
                        break
                    }
                }
            }

            ComboBox {
                id: serviceCombo
                label: qsTr("Service")

                menu: ContextMenu {
                    MenuItem {
                        text: qsTr("Mastodon")
                    }
                    MenuItem {
                        text: qsTr("Bluesky")
                    }
                }
            }

            TextField {
                id: masInstanceField
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

    Component {
        id: newMasAccountDialog
        NewAccountMasAuthDialog {
            name: nameField.text
            instance: masInstanceField.text
        }
    }
}

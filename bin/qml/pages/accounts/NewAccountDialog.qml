import QtQuick 2.0
import Sailfish.Silica 1.0
import harbour.hollyphant 1.0

Dialog {
    id: container
    canAccept: blueskyUsernameField.text.length > 0
               && blueskyPasswordField.text.length > 0

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

            ButtonLayout {
                visible: serviceCombo.currentIndex === 0
                anchors.left: parent.left
                anchors.right: parent.right

                Button {
                    text: qsTr("Open browser to login")
                    onClicked: mastodonItem.execute("oauth")
                }
            }

            TextField {
                id: blueskyUsernameField
                visible: serviceCombo.currentIndex === 1
                label: qsTr("Username")
                placeholderText: qsTr("Username")

                EnterKey.iconSource: "image://theme/icon-m-enter-next"
                EnterKey.onClicked: blueskyPasswordField.focus = true
            }

            PasswordField {
                visible: serviceCombo.currentIndex === 1
                id: blueskyPasswordField
                EnterKey.iconSource: "image://theme/icon-m-enter-accept"
                EnterKey.onClicked: dialog.accept()
            }
        }

        ScrollDecorator {}
    }

    ExtItem {
        id: mastodonItem
        eventBus: EventBus
        key: {
            "action": "account",
            "type": "mastodon"
        }
    }
}

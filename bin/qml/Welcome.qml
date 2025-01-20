import QtQuick 2.0
import Sailfish.Silica 1.0

SilicaFlickable {
    anchors.fill: parent

    Column {
        anchors.left: parent.left
        anchors.right: parent.right
        spacing: Theme.paddingLarge

        PageHeader {
            title: qsTr("Hollyphant")
        }

        Label {
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.leftMargin: Theme.horizontalPageMargin
            anchors.rightMargin: Theme.horizontalPageMargin
            color: Theme.highlightColor

            text: qsTr("Welcome to Hollyphant. Hollyphant is a multi-account Mastodon and Bluesky client. To continue, please add an account in the accounts settings.")
            wrapMode: Text.WordWrap
        }

        ButtonLayout {
            id: buttonsLayout
            anchors.left: parent.left
            anchors.right: parent.right

            Button {
                text: qsTr("Open account settings")
                onClicked: {
                    pageStack.push(Qt.resolvedUrl("accounts/AccountsPage.qml"))
                }
            }
        }
    }

    ScrollDecorator {}
}

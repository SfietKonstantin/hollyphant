import QtQuick 2.0
import Sailfish.Silica 1.0
import harbour.hollyphant 1.0
import "accounts"

Page {
    id: container
    SilicaFlickable {
        anchors.fill: parent
        contentHeight: Math.max(
                           container.height,
                           column.height + Theme.paddingLarge + buttonsLayout.height)

        Column {
            id: column
            anchors.left: parent.left
            anchors.right: parent.right
            spacing: Theme.paddingMedium

            PageHeader {
                title: qsTr("Hollyphant")
            }

            Label {
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.leftMargin: Theme.horizontalPageMargin
                anchors.rightMargin: Theme.horizontalPageMargin

                text: qsTr("Welcome to Hollyphant. Hollyphant is a multi-account Mastodon and Bluesky client. To continue, please register an account")
                wrapMode: Text.WordWrap
            }
        }

        ButtonLayout {
            id: buttonsLayout
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.bottom: parent.bottom
            anchors.bottomMargin: Theme.paddingLarge

            Button {
                text: qsTr("Add new account")
                onClicked: pageStack.push(newAccountDialog)
            }
        }

        ScrollDecorator {}
    }

    Component {
        id: newAccountDialog
        NewAccountDialog {}
    }
}

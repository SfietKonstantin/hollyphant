import QtQuick 2.0
import Sailfish.Silica 1.0

Dialog {
    id: container
    canAccept: false

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

            Label {
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.leftMargin: Theme.horizontalPageMargin
                anchors.rightMargin: Theme.horizontalPageMargin

                text: qsTr("Bluesky is not yet supported.")
                wrapMode: Text.WordWrap
            }
        }

        ScrollDecorator {}
    }
}

import QtQuick 2.0
import Sailfish.Silica 1.0
import harbour.hollyphant 1.0

Column {
    id: container
    property StatusItem statusItem
    anchors.left: parent.left
    anchors.right: parent.right
    spacing: Theme.paddingLarge

    Icon {
        anchors.centerIn: parent
        source: "image://icon-l-attention"
    }

    Label {
        id: messageLabel
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.leftMargin: Theme.horizontalPageMargin
        anchors.rightMargin: Theme.horizontalPageMargin
        color: Theme.highlightColor
        text: statusItem.message
        wrapMode: Text.WordWrap
    }

    Label {
        id: detailsLabel
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.leftMargin: Theme.horizontalPageMargin
        anchors.rightMargin: Theme.horizontalPageMargin
        color: Theme.highlightColor
        text: statusItem.details
        wrapMode: Text.WordWrap
        font.pixelSize: Theme.fontSizeSmall
    }
}
import QtQuick 2.0
import Sailfish.Silica 1.0

Page {
    PageHeader {
        title: qsTr("Hollyphant")
    }

    BusyIndicator {
        size: BusyIndicatorSize.Large
        anchors.centerIn: parent
        running: true
    }
}

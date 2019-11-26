import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

ColumnLayout {
    id: channelTab

    ListView {
        id: listView
        x: 0
        y: 0
        width: 110
        height: 160
        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

        model: remote

        delegate: Item {
            x: 5
            width: 80
            height: 40
            Row {
                id: row1
                Rectangle {
                    width: 40
                    height: 40
                    color: colorCode
                }

                Button {
                    text: name
                    onClicked: remote.launch_app(appId)
                    font.bold: true
                    anchors.verticalCenter: parent.verticalCenter
                }
                spacing: 10
            }
        }
    }
}

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
        highlightRangeMode: ListView.ApplyRange
        Layout.fillHeight: true
        Layout.fillWidth: true
        keyNavigationWraps: true
        spacing: 5
        Layout.alignment: Qt.AlignHLeft | Qt.AlignVCenter

        ScrollBar.vertical: ScrollBar {
            active: true
        }

        model: remote
        /* model: ListModel { */
        /*     // TODO load this dynamically from the RokuRemote */
        /*     Component.onCompleted: { */
        /*         append({"name": "Netflix", "colorCode": "red", "appId": 12}) */
        /*         append({"name": "YouTube", "colorCode": "purple", "appId": 837}) */
        /*         append({"name": "Twitch", "colorCode": "red", "appId": 50539}) */
        /*     } */
        /* } */

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

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

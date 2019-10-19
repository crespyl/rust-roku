import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.0

import QtGraphicalEffects 1.12

import RokuRemote 0.1

ApplicationWindow {
    flags: Qt.WA_TranslucentBackground | Qt.TransparentMode

    visible: true
    width: 400
    height: 400
    title: "Roku Remote"
    id: window

    color: "#aa000000"

    RokuRemote {
        id: remote
        //name: '<Not Connected>'
    }

    Frame {
        id: frame
        x: 8
        y: 8
        width: 384
        height: 384

        ColumnLayout {
            width: parent.width
            height: 211
            // Declare a nested element (child of root)
            Image {
                id: triangle

                // reference the parent
                x: (parent.width - width) / 2
                y: 40
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                source: 'triangle_red.png'
            }

            RowLayout {
                id: rowLayout
                Layout.rightMargin: 5
                Layout.leftMargin: 5
                Layout.bottomMargin: 5
                Layout.topMargin: 5
                spacing: 9.9
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                Button {
                    text: 'Find Roku'
                    onClicked: remote.find_roku()
                }

                Text {
                    // un-named element

                    // reference element by id

                    // reference root element
                    width: root.width

                    color: 'white'
                    horizontalAlignment: Text.AlignHCenter
                    text: remote.name
                }
            }

            GridLayout {
                id: gridLayout
                columnSpacing: 10
                rowSpacing: 10
                Layout.topMargin: 20
                flow: GridLayout.TopToBottom
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                Layout.fillHeight: false
                Layout.fillWidth: false
                Layout.rowSpan: 1
                rows: 7
                columns: 3

                Button {
                    text: 'Home'
                    onClicked: remote.home()
                    Layout.row: 0
                    Layout.column: 2
                }

                Button {
                    text: 'Back'
                    onClicked: remote.back()
                    Layout.row: 0
                    Layout.column: 0
                }

                Button {
                    text: 'Up'
                    onClicked: remote.up()
                    Layout.row: 1
                    Layout.column: 1
                }

                Button {
                    text: 'Down'
                    onClicked: remote.down()
                    Layout.row: 3
                    Layout.column: 1
                }

                Button {
                    text: 'OK'
                    onClicked: remote.select()
                    Layout.row: 2
                    Layout.column: 1
                }

                Button {
                    text: 'Left'
                    onClicked: remote.left()
                    Layout.row: 2
                    Layout.column: 0
                }

                Button {
                    text: 'Right'
                    onClicked: remote.right()
                    Layout.row: 2
                    Layout.column: 2
                }

                Button {
                    text: 'Replay'
                    onClicked: remote.instant_replay()
                    Layout.row: 5
                    Layout.column: 0
                }

                Button {
                    text: 'Info'
                    onClicked: remote.info()
                    Layout.row: 5
                    Layout.column: 2
                }

                Button {
                    text: 'Rev'
                    onClicked: remote.rev()
                    Layout.row: 6
                    Layout.column: 0
                }

                Button {
                    text: 'Play/Pause'
                    onClicked: remote.play()
                    Layout.row: 6
                    Layout.column: 1
                }

                Button {
                    text: 'Fwd'
                    onClicked: remote.fwd()
                    Layout.row: 6
                    Layout.column: 2
                }
            }
        }
    }
}

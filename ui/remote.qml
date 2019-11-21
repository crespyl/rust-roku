import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.0

import QtGraphicalEffects 1.12

import RokuRemote 0.1

ApplicationWindow {
    flags: Qt.WA_TranslucentBackground | Qt.TransparentMode

    visible: true
    width: 275
    height: 330
    minimumWidth: 275
    maximumWidth: 275
    minimumHeight: 330
    maximumHeight: 330
    title: "Roku Remote"
    id: window

    //color: "#aa000000"

    RokuRemote {
        id: remote
        name: '<Not Connected>'
        status: 'Idle'
    }

    Action {
        id: a_right
        text: "Right"
        shortcut: "Right"
        onTriggered: remote.right()
    }

    Action {
        id: a_left
        text: "Left"
        shortcut: "Left"
        onTriggered: remote.left()
    }

    Action {
        id: a_up
        text: "Up"
        shortcut: "Up"
        onTriggered: remote.up()
    }

    Action {
        id: a_down
        text: "Down"
        shortcut: "Down"
        onTriggered: remote.down()
    }

    Action {
        id: a_select
        text: "Ok"
        shortcut: "Return"
        onTriggered: remote.select()
    }

    Action {
        id: a_back
        text: "Back"
        shortcut: "Backspace"
        onTriggered: remote.back()
    }

    Action {
        id: a_home
        text: "Home"
        shortcut: "Home"
        onTriggered: remote.home()
    }

    Action {
        id: a_instant_replay
        text: "Replay"
        shortcut: "\\"
        onTriggered: remote.instant_replay()
    }

    Action {
        id: a_info
        text: "Info"
        shortcut: "i"
        onTriggered: remote.info()
    }

    Action {
        id: a_rev
        text: "Rev"
        shortcut: "<"
        onTriggered: remote.rev()
    }

    Action {
        id: a_play
        text: "Play"
        shortcut: "p"
        onTriggered: remote.play()
    }

    Action {
        id: a_fwd
        text: "Fwd"
        shortcut: ">"
        onTriggered: remote.fwd()
    }

    Action {
        id: a_netflix
        text: "Netflix"
        onTriggered: remote.launch_netflix()
    }

    Action {
        id: a_youtube
        text: "Youtube"
        onTriggered: remote.launch_youtube()
    }

    Action {
        id: a_twitch
        text: "Twitch"
        onTriggered: remote.launch_twitch()
    }

    Frame {
        id: frame
        anchors.rightMargin: 5
        anchors.leftMargin: 5
        anchors.bottomMargin: 5
        anchors.topMargin: 5
        anchors.fill: parent

        ColumnLayout {
            anchors.bottom: parent.bottom
            anchors.bottomMargin: 0
            anchors.rightMargin: 0
            anchors.leftMargin: 0
            anchors.topMargin: 0
            anchors.right: parent.right
            anchors.left: parent.left
            anchors.top: parent.top

            RowLayout {
                id: rowLayout
                Layout.fillWidth: true
                Layout.minimumHeight: 45
                Layout.rightMargin: 5
                Layout.leftMargin: 5
                Layout.bottomMargin: 5
                Layout.topMargin: 5
                spacing: 10
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                Button {
                    text: 'Find Roku'
                    onClicked: {
                        remote.find_roku()
                    }
                }

                BusyIndicator {
                    running: remote.status == "Searching"
                    visible: remote.status == "Searching"
                }

                Text {
                    color: 'white'
                    horizontalAlignment: Text.AlignHCenter
                    text: remote.status == "Idle" ? remote.name : "Searching..."
                }
            }

            GridLayout {
                id: gridLayout
                width: 100
                height: 100
                Layout.fillWidth: true
                transformOrigin: Item.Center
                Layout.fillHeight: true
                Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
                rows: 6
                columns: 3

                Button {
                    id: b_back
                    text: a_back.text
                    action: a_back
                    Layout.row: 0
                }

                Button {
                    id: b_up
                    text: a_up.text
                    action: a_up
                    Layout.row: 1
                    Layout.column: 1
                }

                Button {
                    id: b_home
                    text: a_home.text
                    action: a_home
                    Layout.row: 0
                    Layout.column: 2
                }

                Button {
                    id: b_left
                    text: a_left.text
                    action: a_left
                    Layout.row: 2
                    Layout.column: 0
                }

                Button {
                    id: b_select
                    text: a_select.text
                    action: a_select

                    Layout.row: 2
                    Layout.column: 1
                }

                Button {
                    id: b_right
                    text: a_right.text
                    action: a_right
                    Layout.row: 2
                    Layout.column: 2
                }

                Button {
                    id: b_replay
                    text: a_instant_replay.text
                    action: a_instant_replay
                    Layout.row: 4
                    Layout.column: 0
                }

                Button {
                    id: b_down
                    text: a_down.text
                    action: a_down
                    Layout.row: 3
                    Layout.column: 1
                }

                Button {
                    id: b_info
                    text: qsTr("Info")
                    Layout.row: 4
                    Layout.column: 2
                    action: a_info
                }

                Button {
                    id: b_rev
                    text: qsTr("Rev")
                    action: a_rev
                }

                Button {
                    id: b_play
                    text: qsTr("Play")
                    action: a_play
                }

                Button {
                    id: b_fwd
                    text: qsTr("Fwd")
                    action: a_fwd
                }
            }

            RowLayout {
                id: channelButtons
                width: 100
                height: 100
                Layout.bottomMargin: 5
                Layout.alignment: Qt.AlignHCenter | Qt.AlignTop

                Button {
                    id: b_netflix
                    text: a_netflix.text
                    action: a_netflix
                }

                Button {
                    id: b_youtube
                    text: a_youtube.text
                    action: a_youtube
                }

                Button {
                    id: b_twitch
                    text: a_twitch.text
                    action: a_twitch
                }
            }
        }
    }
}

import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

import QtGraphicalEffects 1.12

/* import RokuRemote 0.1 */

ApplicationWindow {
    flags: Qt.WA_TranslucentBackground | Qt.TransparentMode

    visible: true
    title: "Roku Remote"
    id: window
    width: frame.width+frame.padding*2
    height: frame.height+frame.padding*2
    color: "#333333"

    /* maximumWidth: frame.width+frame.padding*2 */
    /* minimumWidth: frame.width+frame.padding*2 */
    /* maximumHeight: frame.height+frame.padding*2 */
    /* minimumHeight: frame.height+frame.padding*2 */

    //color: "#aa000000"

    Item {
        id: remote
        /* name: '<Not Connected>' */
        /* status: 'Idle' */
    }

    Frame {
        id: frame
        x: padding
        y: padding
        width: contentWidth+padding*2
        height: contentHeight + padding
        transformOrigin: Item.Center
        padding: 5

        ColumnLayout {
            id: l_mainColumn

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

            TabBar {
                id: tabbar
                width: parent.width
                currentIndex: 0
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                TabButton {
                    text: qsTr("Remote")
                }
                TabButton {
                    text: qsTr("Channels")
                }
            }

            SwipeView {
                id: swipeView
                Layout.preferredHeight: 200
                Layout.bottomMargin: 5
                bottomPadding: 1
                Layout.fillHeight: true
                Layout.fillWidth: true
                currentIndex: tabbar.currentIndex
                clip: true

                ColumnLayout {
                    id: remoteTab
                    width: 200
                    height: 200
                    spacing: 2
                    transformOrigin: Item.Center

                    GridLayout {
                        x: 0
                        y: 0
                        width: 100
                        height: 100
                        Layout.fillWidth: true
                        transformOrigin: Item.Center
                        Layout.fillHeight: true
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
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
                            text: a_info.text
                            Layout.row: 4
                            Layout.column: 2
                            action: a_info
                        }

                        Button {
                            id: b_rev
                            text: a_rev.text
                            action: a_rev
                        }

                        Button {
                            id: b_play
                            text: a_play.text
                            action: a_play
                        }

                        Button {
                            id: b_fwd
                            text: a_fwd.text
                            action: a_fwd
                        }
                    }
                }

                ColumnLayout {
                    id: channelTab

                    RowLayout {
                        id: channelButtons

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
}

/*##^##
Designer {
    D{i:13;anchors_height:100;anchors_width:100}D{i:12;anchors_height:200;anchors_width:200}
D{i:26;anchors_width:238}
}
##^##*/

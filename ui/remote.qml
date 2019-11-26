import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

import QtGraphicalEffects 1.12

import RokuRemote 0.1

ApplicationWindow {
    flags: Qt.WA_TranslucentBackground | Qt.TransparentMode

    visible: true
    title: "Roku Remote"
    id: window
    width: frame.width+frame.padding*2
    height: frame.height+frame.padding*2
    //color: "#333333"

    RokuRemote {
        id: remote
        name: '<Not Connected>'
        status: 'Idle'

        onCountChanged: {
            console.log("got "+count+" channels")
        }
    }

    Frame {
        id: frame
        x: padding
        y: padding
        width: contentWidth+padding*2
        height: contentHeight + padding
        anchors.verticalCenter: parent.verticalCenter
        anchors.horizontalCenter: parent.horizontalCenter
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
                    text: qsTr("Find Roku")
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
                    text: remote.status == "Idle" ? remote.name : qsTr("Searching...")
                }
            }

            TabBar {
                id: tabbar
                width: parent.width
                currentIndex: swipeView.currentIndex
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
                Layout.minimumWidth: 240
                Layout.minimumHeight: 182
                Layout.bottomMargin: 5
                bottomPadding: 1
                Layout.fillHeight: true
                Layout.fillWidth: true
                currentIndex: tabbar.currentIndex
                clip: true

                RemoteTab {
                    id: remoteTab
                }

                ChannelTab {
                    id: channelTab
                    // bind minimum size for the channel tab to parent, since
                    // remoteTab is loaded first, this lets us keep the frame
                    // the same size it was on load. This doesn't work if we
                    // bind to remoteTab.width, since the value gets reset to 0
                    // when the tab content is not visible
                    Layout.minimumHeight: parent.height
                    Layout.minimumWidth: parent.width
                }
            }
        }
    }

    Action {
        id: a_right
        text: qsTr("Right")
        shortcut: "Right"
        onTriggered: remote.right()
    }

    Action {
        id: a_left
        text: qsTr("Left")
        shortcut: "Left"
        onTriggered: remote.left()
    }

    Action {
        id: a_up
        text: qsTr("Up")
        shortcut: "Up"
        onTriggered: remote.up()
    }

    Action {
        id: a_down
        text: qsTr("Down")
        shortcut: "Down"
        onTriggered: remote.down()
    }

    Action {
        id: a_select
        text: qsTr("Ok")
        shortcut: "Return"
        onTriggered: remote.select()
    }

    Action {
        id: a_back
        text: qsTr("Back")
        shortcut: "Backspace"
        onTriggered: remote.back()
    }

    Action {
        id: a_home
        text: qsTr("Home")
        shortcut: "Home"
        onTriggered: remote.home()
    }

    Action {
        id: a_instant_replay
        text: qsTr("Replay")
        shortcut: "\\"
        onTriggered: remote.instant_replay()
    }

    Action {
        id: a_info
        text: qsTr("Info")
        shortcut: "i"
        onTriggered: remote.info()
    }

    Action {
        id: a_rev
        text: qsTr("Rev")
        shortcut: "<"
        onTriggered: remote.rev()
    }

    Action {
        id: a_play
        text: qsTr("Play")
        shortcut: "p"
        onTriggered: remote.play()
    }

    Action {
        id: a_fwd
        text: qsTr("Fwd")
        shortcut: ">"
        onTriggered: remote.fwd()
    }
}

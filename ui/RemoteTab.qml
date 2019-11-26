import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

ColumnLayout {
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

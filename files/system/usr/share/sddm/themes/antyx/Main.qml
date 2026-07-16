import QtQuick 2.15
import QtQuick.Controls 2.15
import SddmComponents 2.0

Rectangle {
    id: root
    width: 1920; height: 1080
    color: "#08060d"

    Image {
        anchors.fill: parent
        source: "background.svg"
        fillMode: Image.PreserveAspectCrop
        smooth: true
    }

    Rectangle {
        anchors.fill: parent
        color: "#52000000"
    }

    Column {
        anchors.centerIn: parent
        width: Math.min(parent.width * 0.34, 560)
        spacing: 20

        Image {
            anchors.horizontalCenter: parent.horizontalCenter
            width: 170; height: 170
            source: "logo.svg"
            fillMode: Image.PreserveAspectFit
        }

        Text {
            anchors.horizontalCenter: parent.horizontalCenter
            text: "ANTYX-OS"
            color: "#ffffff"
            font.pixelSize: 54
            font.bold: true
            font.letterSpacing: 7
        }

        Text {
            anchors.horizontalCenter: parent.horizontalCenter
            text: "SECURE. STABLE. GAMING."
            color: "#ba8cff"
            font.pixelSize: 15
            font.bold: true
            font.letterSpacing: 4
        }

        TextField {
            id: username
            width: parent.width
            height: 58
            placeholderText: "Benutzername"
            text: userModel.lastUser
            color: "#ffffff"
            placeholderTextColor: "#9d91af"
            background: Rectangle {
                radius: 10
                color: "#dd15111e"
                border.color: username.activeFocus ? "#9957ff" : "#4b405b"
                border.width: 1
            }
            Keys.onReturnPressed: password.forceActiveFocus()
        }

        TextField {
            id: password
            width: parent.width
            height: 58
            placeholderText: "Passwort"
            echoMode: TextInput.Password
            color: "#ffffff"
            placeholderTextColor: "#9d91af"
            background: Rectangle {
                radius: 10
                color: "#dd15111e"
                border.color: password.activeFocus ? "#9957ff" : "#4b405b"
                border.width: 1
            }
            Keys.onReturnPressed: sddm.login(username.text, password.text, session.index)
        }

        ComboBox {
            id: session
            width: parent.width
            model: sessionModel
            textRole: "name"
            currentIndex: sessionModel.lastIndex
        }

        Button {
            width: parent.width
            height: 58
            text: "ANMELDEN"
            onClicked: sddm.login(username.text, password.text, session.index)
            background: Rectangle {
                radius: 10
                color: parent.down ? "#7436dc" : "#9555ff"
            }
            contentItem: Text {
                text: parent.text
                color: "#ffffff"
                font.bold: true
                font.letterSpacing: 2
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }

        Text {
            anchors.horizontalCenter: parent.horizontalCenter
            text: sddm.lastError
            color: "#ff7897"
            visible: text.length > 0
        }
    }
}

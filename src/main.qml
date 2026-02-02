import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material

ApplicationWindow {
    visible: true
    width: 1000
    height: 600
    title: "Blick"
    Material.theme: Material.Dark
    Material.accent: Material.Blue

    component BlickToolButton: ToolButton {
        icon.width: 24
        icon.height: 24
        Layout.fillWidth: false
    }

    header: ToolBar {
        RowLayout {
            anchors.fill: parent
            spacing: 2
            BlickToolButton {
                icon.source: "icons/zoom-in.png"
                onClicked: {
                    chapterText.font.pixelSize += 4
                    console.debug("New font pixel size is ", chapterText.font.pixelSize)
                }
            }
            BlickToolButton {
                icon.source: "icons/zoom-out.png"
                onClicked: {
                    chapterText.font.pixelSize -= 4
                    console.debug("New font pixel size is ", chapterText.font.pixelSize)
                }
            }
            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true
            }
        }
    }
    
    SplitView {
        anchors.fill: parent
        orientation: Qt.Horizontal
        
        ListView {
            SplitView.preferredWidth: 250
            model: epubModel.chapters
            delegate: ItemDelegate {
                text: modelData
                onClicked: epubModel.load_chapter(index)
            }
        }
        
        ScrollView {
            SplitView.fillWidth: true
            TextArea {
                id: chapterText
                text: epubModel.current_content
                readOnly: true
                textFormat: TextEdit.RichText
                wrapMode: Text.WordWrap
            }
        }
    }
}


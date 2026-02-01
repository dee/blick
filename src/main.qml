import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

ApplicationWindow {
    visible: true
    width: 1000
    height: 600
    title: "Blick"
    
    SplitView {
        anchors.fill: parent
        orientation: Qt.Horizontal
        
        ListView {
            SplitView.preferredWidth: 250
            model: epubModel.chapters
            delegate: ItemDelegate {
                text: modelData
                width: parent.width
                onClicked: epubModel.load_chapter(index)
            }
        }
        
        ScrollView {
            SplitView.fillWidth: true
            TextArea {
                text: epubModel.current_content
                readOnly: true
                textFormat: TextEdit.RichText
                wrapMode: Text.WordWrap
            }
        }
    }
}


import QtQuick
import Quickshell
import Quickshell.Io
import qs.Common
import qs.Widgets
import qs.Services
import qs.Modules.Plugins

PluginComponent {
    id: root

    property bool isEnabled: false
    property string statusLabel: isEnabled ? "Connesso" : "Disattivato"

    ccWidgetIcon: "mouse"
    ccWidgetPrimaryText: "Lan Mouse"
    ccWidgetSecondaryText: statusLabel
    ccWidgetIsActive: isEnabled

    onCcWidgetToggled: {
        toggleProcess.running = true
    }

    Timer {
        id: pollTimer
        interval: 2500
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: {
            if (!statusProcess.running && !toggleProcess.running) {
                statusProcess.running = true
            }
        }
    }

    Process {
        id: statusProcess
        command: ["sh", "-c", "lan-mouse cli list 2>/dev/null | grep -q 'active: true'"]
        running: false
        onExited: function(exitCode, exitStatus) {
            root.isEnabled = (exitCode === 0)
            root.statusLabel = root.isEnabled ? "Connesso" : "Disattivato"
        }
    }

    Process {
        id: toggleProcess
        command: ["sh", "-c", "if lan-mouse cli list 2>/dev/null | grep -q 'active: true'; then lan-mouse cli deactivate 0; else lan-mouse cli activate 0; fi"]
        running: false
        onExited: function(exitCode, exitStatus) {
            statusProcess.running = true
        }
    }

    horizontalBarPill: Component {
        DankIcon {
            name: "mouse"
            size: root.iconSize
            color: root.isEnabled ? Theme.primary : Theme.surfaceVariantText
            anchors.verticalCenter: parent.verticalCenter
        }
    }

    verticalBarPill: Component {
        DankIcon {
            name: "mouse"
            size: root.iconSize
            color: root.isEnabled ? Theme.primary : Theme.surfaceVariantText
            anchors.horizontalCenter: parent.horizontalCenter
        }
    }

    pillClickAction: function() {
        toggleProcess.running = true
    }
}

import QtQuick
import Quickshell
import qs.Common
import qs.Widgets
import qs.Services
import qs.Modules.Plugins

PluginComponent {
    id: root

    readonly property bool hasWindow: {
        if (CompositorService.isNiri) {
            return NiriService.windows.length > 0;
        }
        return true;
    }

    pillClickAction: function() {
        Quickshell.execDetached(["niri", "msg", "action", "close-window"]);
    }

    pillRightClickAction: function() {
        Quickshell.execDetached(["niri", "msg", "action", "close-window"]);
    }

    horizontalBarPill: Component {
        DankIcon {
            name: "close"
            size: root.iconSize
            color: root.hasWindow ? Theme.error : Theme.surfaceVariantText
            anchors.verticalCenter: parent.verticalCenter
        }
    }

    verticalBarPill: Component {
        DankIcon {
            name: "close"
            size: root.iconSize
            color: root.hasWindow ? Theme.error : Theme.surfaceVariantText
            anchors.horizontalCenter: parent.horizontalCenter
        }
    }
}

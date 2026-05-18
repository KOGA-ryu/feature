#pragma once

#include <QString>
#include <QVector>

namespace DexSettingsShortcuts {

struct ShortcutCommandRecord {
    QString command;
    QString shortcut;
    QString notes;
};

struct ShortcutCommandTab {
    QString tabName;
    QVector<ShortcutCommandRecord> records;
};

QVector<ShortcutCommandTab> shortcutCommandTabs();

} // namespace DexSettingsShortcuts

#include "settings_shortcuts_dialog.h"

#include <QDialog>
#include <QHeaderView>
#include <QLabel>
#include <QPushButton>
#include <QTableWidget>
#include <QTableWidgetItem>
#include <QTabWidget>
#include <QVBoxLayout>

#include "settings_shortcuts_model.h"

namespace DexSettingsShortcuts {
namespace {

QTableWidgetItem *readOnlyItem(const QString &text) {
    auto *item = new QTableWidgetItem(text);
    item->setFlags(item->flags() & ~Qt::ItemIsEditable);
    return item;
}

QWidget *buildTab(const ShortcutCommandTab &tab) {
    auto *page = new QWidget;
    auto *layout = new QVBoxLayout(page);
    layout->setContentsMargins(10, 10, 10, 10);
    layout->setSpacing(8);

    auto *table = new QTableWidget(tab.records.size(), 3);
    table->setHorizontalHeaderLabels({"command", "shortcut / command", "notes"});
    table->verticalHeader()->hide();
    table->setEditTriggers(QAbstractItemView::NoEditTriggers);
    table->setSelectionBehavior(QAbstractItemView::SelectRows);
    table->setSelectionMode(QAbstractItemView::SingleSelection);
    table->horizontalHeader()->setSectionResizeMode(0, QHeaderView::ResizeToContents);
    table->horizontalHeader()->setSectionResizeMode(1, QHeaderView::ResizeToContents);
    table->horizontalHeader()->setSectionResizeMode(2, QHeaderView::Stretch);
    table->setWordWrap(true);

    for (int row = 0; row < tab.records.size(); ++row) {
        const ShortcutCommandRecord &record = tab.records.at(row);
        table->setItem(row, 0, readOnlyItem(record.command));
        table->setItem(row, 1, readOnlyItem(record.shortcut));
        table->setItem(row, 2, readOnlyItem(record.notes));
    }
    table->resizeRowsToContents();
    layout->addWidget(table, 1);
    return page;
}

} // namespace

void showShortcutsDialog(QWidget *parent) {
    auto *dialog = new QDialog(parent);
    dialog->setAttribute(Qt::WA_DeleteOnClose);
    dialog->setWindowTitle("Commands and Hotkeys");
    dialog->resize(860, 520);

    auto *layout = new QVBoxLayout(dialog);
    layout->setContentsMargins(12, 12, 12, 12);
    layout->setSpacing(10);

    auto *summary = new QLabel("Settings command reference. OpenAI Codex entries include only verified shortcuts or local workflow commands with scope notes.");
    summary->setWordWrap(true);
    layout->addWidget(summary);

    auto *tabs = new QTabWidget;
    for (const ShortcutCommandTab &tab : shortcutCommandTabs()) {
        tabs->addTab(buildTab(tab), tab.tabName);
    }
    layout->addWidget(tabs, 1);

    auto *close = new QPushButton("Close");
    close->setObjectName("statsContextAction");
    QObject::connect(close, &QPushButton::clicked, dialog, &QDialog::accept);
    layout->addWidget(close, 0, Qt::AlignRight);
    dialog->show();
}

} // namespace DexSettingsShortcuts

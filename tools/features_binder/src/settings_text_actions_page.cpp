#include "settings_text_actions_page.h"

#include <QScrollArea>
#include <QPushButton>
#include <QVBoxLayout>

#include "binder_page_helpers.h"
#include "settings_shortcuts_dialog.h"
#include "text_action_proof_panel.h"

namespace DexSettingsPages {

QWidget *buildTextActionsSettingsPage() {
    auto *page = new QWidget;
    auto *layout = new QVBoxLayout(page);
    layout->setContentsMargins(0, 0, 0, 0);
    layout->setSpacing(0);

    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);

    auto *body = new QWidget;
    auto *bodyLayout = new QVBoxLayout(body);
    bodyLayout->setContentsMargins(12, 12, 12, 12);
    bodyLayout->setSpacing(10);

    auto *utility = DexBinderPages::makeStatsSection("settings utilities", true);
    auto *utilityLayout = static_cast<QVBoxLayout *>(utility->layout());
    utilityLayout->addWidget(DexBinderPages::makeStatsText(
        "Open a compact reference for binder commands, text-action commands, and verified Codex shortcuts."));
    auto *shortcuts = new QPushButton("Commands / Hotkeys");
    shortcuts->setObjectName("primaryAction");
    QObject::connect(shortcuts, &QPushButton::clicked, page, [page]() {
        DexSettingsShortcuts::showShortcutsDialog(page);
    });
    utilityLayout->addWidget(shortcuts, 0, Qt::AlignLeft);
    bodyLayout->addWidget(utility);

    bodyLayout->addWidget(new TextActionProofPanel);
    bodyLayout->addStretch(1);

    scroll->setWidget(body);
    layout->addWidget(scroll, 1);
    return page;
}

} // namespace DexSettingsPages

#include "settings_text_actions_page.h"

#include <QScrollArea>
#include <QVBoxLayout>

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
    bodyLayout->addWidget(new TextActionProofPanel);
    bodyLayout->addStretch(1);

    scroll->setWidget(body);
    layout->addWidget(scroll, 1);
    return page;
}

} // namespace DexSettingsPages

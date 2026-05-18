#include "right_context_render_helpers.h"

#include <QLabel>
#include <QPushButton>
#include <QVBoxLayout>

#include "render_helpers.h"

namespace DexRightContext {

void addWrappedLine(QVBoxLayout *contentLayout, const QString &text) {
    auto *label = makeLabel(text, "smallLabel");
    label->setWordWrap(true);
    contentLayout->addWidget(label);
}

void addContextSection(QVBoxLayout *contentLayout, const QString &title) {
    contentLayout->addWidget(makeLabel(title, "sectionLabel"));
}

QPushButton *makeContextActionButton(const QString &action, int width, const QString &tooltip, bool enabled) {
    auto *button = new QPushButton(action);
    button->setObjectName("statsContextAction");
    button->setMinimumWidth(width);
    button->setMaximumWidth(width);
    button->setMaximumHeight(20);
    button->setToolTip(tooltip.isEmpty() ? action : tooltip);
    button->setEnabled(enabled);
    return button;
}

} // namespace DexRightContext

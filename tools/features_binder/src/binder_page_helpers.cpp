#include "binder_page_helpers.h"

#include <QScrollArea>

#include "ui_rules.h"

namespace DexBinderPages {

QWidget *buildBinderPage(std::function<void(QVBoxLayout *)> populate) {
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
    bodyLayout->setContentsMargins(18, 16, 18, 16);
    bodyLayout->setSpacing(8);
    populate(bodyLayout);
    bodyLayout->addStretch(1);

    scroll->setWidget(body);
    layout->addWidget(scroll, 1);
    return page;
}

QWidget *buildCompactPage(std::function<void(QVBoxLayout *)> populate) {
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
    bodyLayout->setContentsMargins(10, 8, 10, 8);
    bodyLayout->setSpacing(6);
    populate(bodyLayout);
    bodyLayout->addStretch(1);

    scroll->setWidget(body);
    layout->addWidget(scroll, 1);
    return page;
}

void addSection(QVBoxLayout *layout, const QString &title, const QStringList &lines) {
    layout->addWidget(dex_ui::make_section(title, lines));
}

} // namespace DexBinderPages

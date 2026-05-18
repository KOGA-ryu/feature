#include "binder_page_helpers.h"

#include <QHBoxLayout>
#include <QSizePolicy>

#include "ui_rules.h"

namespace DexBinderPages {

QFrame *makeStatsSection(const QString &title, bool subtle) {
    auto *section = new QFrame;
    section->setObjectName(subtle ? "statsSubtleSection" : "statsSection");
    section->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Minimum);
    auto *sectionLayout = new QVBoxLayout(section);
    sectionLayout->setContentsMargins(8, 5, 8, 5);
    sectionLayout->setSpacing(3);
    sectionLayout->addWidget(dex_ui::make_label(title, "statsSectionTitle"));
    auto *rule = new QFrame;
    rule->setFrameShape(QFrame::HLine);
    rule->setObjectName("statsRule");
    sectionLayout->addWidget(rule);
    return section;
}

QLabel *makeStatsText(const QString &text, const char *objectName) {
    auto *label = dex_ui::make_label(text, objectName);
    label->setWordWrap(true);
    return label;
}

QFrame *makeStatsRow(const QStringList &cells, bool risk, bool routeTone) {
    auto *row = new QFrame;
    row->setObjectName(risk ? "statsRiskRow" : "statsRow");
    row->setMinimumHeight(12);
    auto *rowLayout = new QHBoxLayout(row);
    rowLayout->setContentsMargins(6, 0, 6, 0);
    rowLayout->setSpacing(6);
    for (int index = 0; index < cells.size(); ++index) {
        const QString cell = cells.at(index);
        const bool routeCell = index == cells.size() - 1;
        const bool positiveRoute = cell.contains("direct", Qt::CaseInsensitive);
        const char *style = risk || (routeTone && routeCell && !positiveRoute)
            ? "statsRiskText"
            : ((routeTone && routeCell) ? "statsGoodText" : "statsTinyLabel");
        auto *label = makeStatsText(cell, style);
        label->setMinimumWidth(0);
        label->setSizePolicy(QSizePolicy::Ignored, QSizePolicy::Minimum);
        label->setToolTip(cell);
        rowLayout->addWidget(label, index == 0 ? 3 : 1);
    }
    return row;
}

QFrame *makeStatsMiniCell(const QString &label, const QString &value) {
    auto *cell = new QFrame;
    cell->setObjectName("statsMiniCell");
    cell->setMinimumHeight(16);
    auto *layout = new QVBoxLayout(cell);
    layout->setContentsMargins(6, 1, 6, 1);
    layout->setSpacing(0);
    layout->addWidget(makeStatsText(label));
    layout->addWidget(makeStatsText(value));
    return cell;
}

} // namespace DexBinderPages

#include "ui_rules.h"

#include <QFrame>
#include <QHBoxLayout>
#include <QSizePolicy>
#include <QVBoxLayout>

namespace dex_ui {

QLabel *make_label(const QString &text, const char *object_name) {
    auto *label = new QLabel(text);
    if (object_name) {
        label->setObjectName(object_name);
    }
    if (object_name && QString::fromLatin1(object_name) == "sectionLabel") {
        label->setFixedHeight(metrics::section_label_height);
    }
    label->setTextInteractionFlags(Qt::TextSelectableByMouse);
    return label;
}

QLabel *make_page_title(const QString &text) {
    return make_label(text, "reportTitle");
}

QLabel *make_muted_label(const QString &text) {
    auto *label = make_label(text, "mutedLabel");
    label->setWordWrap(true);
    return label;
}

QLabel *make_badge(const QString &text, const QString &kind) {
    auto *label = make_label(text, "riskBadge");
    label->setProperty("kind", risk_kind_token(kind));
    label->setAlignment(Qt::AlignCenter);
    label->setSizePolicy(QSizePolicy::Minimum, QSizePolicy::Fixed);
    return label;
}

QWidget *make_section_container(const QString &title, const QString &subtitle) {
    auto *section = new QFrame;
    section->setObjectName("reportEntry");
    section->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Minimum);

    auto *section_layout = new QVBoxLayout(section);
    section_layout->setContentsMargins(16, metrics::section_padding, 16, metrics::section_padding);
    section_layout->setSpacing(5);
    section_layout->addWidget(make_page_title(title));
    if (!subtitle.isEmpty()) {
        section_layout->addWidget(make_muted_label(subtitle));
    }
    return section;
}

QWidget *make_section(const QString &title, const QStringList &lines) {
    auto *section = make_section_container(title);
    auto *section_layout = static_cast<QVBoxLayout *>(section->layout());
    if (lines.isEmpty()) {
        section_layout->addWidget(make_muted_label("No data recorded."));
    } else {
        for (const QString &line : lines) {
            auto *label = make_label(line, "smallLabel");
            label->setWordWrap(true);
            section_layout->addWidget(label);
        }
    }
    return section;
}

QWidget *make_compact_row(const QStringList &cells) {
    auto *row = new QFrame;
    row->setObjectName("compactRow");
    row->setMinimumHeight(metrics::compact_row_height);
    row->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Minimum);

    auto *layout = new QHBoxLayout(row);
    layout->setContentsMargins(10, 4, 10, 4);
    layout->setSpacing(8);
    for (int index = 0; index < cells.size(); ++index) {
        auto *label = make_label(cells.at(index), index == 0 ? "smallLabel" : "mutedLabel");
        label->setWordWrap(index == 0);
        label->setToolTip(cells.at(index));
        layout->addWidget(label, index == 0 ? 3 : 1);
    }
    return row;
}

QWidget *make_file_inventory_row(
    const QString &path,
    const QString &size_bucket,
    const QString &approximate_size,
    const QString &role,
    const QString &risk,
    const QString &recommended_action) {
    auto *row = new QFrame;
    row->setObjectName("compactRow");
    row->setMinimumHeight(metrics::compact_row_height + 12);
    row->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Minimum);

    auto *outer = new QVBoxLayout(row);
    outer->setContentsMargins(10, 5, 10, 5);
    outer->setSpacing(4);

    auto *path_label = make_label(path, "smallLabel");
    path_label->setWordWrap(true);
    path_label->setToolTip(path);
    outer->addWidget(path_label);

    auto *meta = new QHBoxLayout;
    meta->setSpacing(7);
    const QStringList meta_cells = {size_bucket, approximate_size, role};
    for (const QString &cell : meta_cells) {
        auto *label = make_label(cell, "mutedLabel");
        label->setToolTip(cell);
        meta->addWidget(label, 1);
    }
    meta->addWidget(make_badge(risk, risk));
    auto *action = make_label(recommended_action, "smallLabel");
    action->setToolTip(recommended_action);
    meta->addWidget(action, 2);
    outer->addLayout(meta);
    return row;
}

} // namespace dex_ui

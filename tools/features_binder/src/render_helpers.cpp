#include "render_helpers.h"

#include <QLayout>
#include <QWidget>

#include <algorithm>

#include "project_registry.h"
#include "ui_rules.h"

QString appStyleSheet() {
    return dex_ui::app_qss();
}

QLabel *makeLabel(const QString &text, const char *objectName) {
    return dex_ui::make_label(text, objectName);
}

void clearLayout(QLayout *layout) {
    while (auto *item = layout->takeAt(0)) {
        if (auto *childLayout = item->layout()) {
            clearLayout(childLayout);
            delete childLayout;
        }
        if (auto *widget = item->widget()) {
            widget->deleteLater();
        }
        delete item;
    }
}

QJsonArray toJsonArray(const QStringList &items) {
    QJsonArray values;
    for (const QString &item : items) {
        values.push_back(item);
    }
    return values;
}

QStringList compactLines(const QStringList &lines, const QString &emptyText) {
    return lines.isEmpty() ? QStringList{emptyText} : lines;
}

QString featurePackGroupSummary(const DexProjects::ProjectRegistryEntry &project) {
    return QString("logic %1 | sim %2 | ui %3 | workflows %4")
        .arg(project.featurePackLogic)
        .arg(project.featurePackSim)
        .arg(project.featurePackUi)
        .arg(project.featurePackWorkflows);
}

QString projectBinderTemplate(const DexProjects::ProjectRegistryEntry &project) {
    return project.binderTemplate.isEmpty() ? QString("repo_default_binder_v1") : project.binderTemplate;
}

QString summarizePaths(const QStringList &paths, int maxVisible) {
    if (paths.isEmpty()) {
        return "none";
    }
    QStringList visible;
    const int count = std::min(maxVisible, static_cast<int>(paths.size()));
    for (int i = 0; i < count; ++i) {
        visible.push_back(paths.at(i));
    }
    QString summary = QString::number(paths.size()) + " path";
    if (paths.size() != 1) {
        summary += "s";
    }
    summary += ": " + visible.join(" | ");
    if (paths.size() > maxVisible) {
        summary += " | ...";
    }
    return summary;
}

QString pathCountLabel(const QStringList &paths) {
    if (paths.isEmpty()) {
        return "none";
    }
    return QString::number(paths.size()) + (paths.size() == 1 ? " path" : " paths");
}

QString compactCommandLabel(QString command) {
    command.replace("/Users/kogaryu/dev/dex_home_final", "$FINAL");
    command.replace("/Users/kogaryu/dev/dex_home", "$DEX_HOME");
    constexpr qsizetype maxLength = 72;
    if (command.size() <= maxLength) {
        return command;
    }
    return command.left(maxLength - 3) + "...";
}

QString compactPathLabel(QString path, qsizetype maxLength) {
    path.replace("/Users/kogaryu/dev/dex_home_final", "$FINAL");
    path.replace("/Users/kogaryu/dev/dex_home", "$DEX_HOME");
    if (path.size() <= maxLength) {
        return path;
    }
    const qsizetype leftLength = std::max<qsizetype>(8, (maxLength - 3) / 2);
    const qsizetype rightLength = std::max<qsizetype>(8, maxLength - 3 - leftLength);
    return path.left(leftLength) + "..." + path.right(rightLength);
}

QString compactReceiptLabel(const QString &value, qsizetype maxLength) {
    if (value.size() <= maxLength) {
        return value;
    }
    const qsizetype leftLength = std::max<qsizetype>(8, (maxLength - 3) / 2);
    const qsizetype rightLength = std::max<qsizetype>(8, maxLength - 3 - leftLength);
    return value.left(leftLength) + "..." + value.right(rightLength);
}

#pragma once

#include <QJsonArray>
#include <QLabel>
#include <QString>
#include <QStringList>

class QLayout;

namespace DexProjects {
struct ProjectRegistryEntry;
}

QString appStyleSheet();
QLabel *makeLabel(const QString &text, const char *objectName = nullptr);
void clearLayout(QLayout *layout);
QJsonArray toJsonArray(const QStringList &items);
QStringList compactLines(const QStringList &lines, const QString &emptyText = "none");
QString featurePackGroupSummary(const DexProjects::ProjectRegistryEntry &project);
QString projectBinderTemplate(const DexProjects::ProjectRegistryEntry &project);
QString summarizePaths(const QStringList &paths, int maxVisible = 3);
QString pathCountLabel(const QStringList &paths);
QString compactCommandLabel(QString command);
QString compactPathLabel(QString path, qsizetype maxLength = 48);
QString compactReceiptLabel(const QString &value, qsizetype maxLength = 44);

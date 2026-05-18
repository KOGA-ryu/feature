#pragma once

#include <QFrame>
#include <QLabel>
#include <QString>
#include <QStringList>
#include <QVBoxLayout>
#include <QWidget>

#include <functional>

namespace DexBinderPages {

QWidget *buildBinderPage(std::function<void(QVBoxLayout *)> populate);
QWidget *buildCompactPage(std::function<void(QVBoxLayout *)> populate);
void addSection(QVBoxLayout *layout, const QString &title, const QStringList &lines);
void addFileInventoryLens(QVBoxLayout *layout);
QFrame *makeStatsSection(const QString &title, bool subtle = false);
QLabel *makeStatsText(const QString &text, const char *objectName = "statsTinyLabel");
QFrame *makeStatsRow(const QStringList &cells, bool risk = false, bool routeTone = true);
QFrame *makeStatsMiniCell(const QString &label, const QString &value);

} // namespace DexBinderPages

#pragma once

#include <QString>
#include <QStringList>

QStringList repoBinderTopTabs();
QStringList topTabsFor(bool repoMode);
QStringList detailLensTabsFor(const QString &topTab, bool repoMode = false);
QString detailLensShortLabel(const QString &lens);

#pragma once

#include <QString>
#include <QWidget>

#include <functional>

namespace DexProjectRailRows {

QWidget *makeProjectRow(
    const QString &projectId,
    const QString &name,
    const QString &path,
    const QString &status,
    bool active,
    std::function<void(QString)> onSelected = {});

QWidget *makeWorkerRow(
    const QString &workerId,
    const QString &role,
    const QString &displayName,
    const QString &status,
    bool selected,
    std::function<void(QString)> onSelected = {});

} // namespace DexProjectRailRows

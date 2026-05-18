#pragma once

#include <QString>
#include <QStringList>
#include <QVector>

#include "repo_cleanup_queue_state.h"
#include "repo_diff_scan_state.h"

namespace DexRepoCleanupQueue {

QStringList sortedUniqueCleanupPaths(QStringList values);
QString cleanupIssueForContract(const QString &contractId);
QString cleanupActionForContract(const QString &contractId);
QString cleanupStatusForContract(const QString &contractId, const QString &violationStatus);
QString fallbackSeverityForContract(const QString &contractId, const QString &severity);
QStringList fallbackPathsForContract(
    const QString &contractId,
    const DexRepoDiffScan::RepoDiffScanState &scan);
bool containsContract(const QVector<CleanupQueueItem> &items, const QString &contractId);
CleanupQueueItem makeCleanupItem(
    int sequence,
    const QString &contractId,
    const QString &issue,
    const QString &severity,
    const QString &status,
    const QString &action,
    const QStringList &paths);

} // namespace DexRepoCleanupQueue

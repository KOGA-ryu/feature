#pragma once

#include <QString>
#include <QStringList>
#include <QVector>

#include "repo_contract_check_state.h"
#include "repo_diff_scan_state.h"

namespace DexRepoCleanupQueue {

struct CleanupQueueItem {
    QString itemId;
    QString sourceContract;
    QString issue;
    QString severity;
    QString status;
    QString recommendedAction;
    QStringList paths;
    int pathCount = 0;
};

struct RepoCleanupQueueState {
    QString queueStatus = "no_data";
    QVector<CleanupQueueItem> items;
    QStringList warnings;
    int highCount = 0;
    int mediumCount = 0;
    int actionRequiredCount = 0;
    bool available = false;
};

RepoCleanupQueueState deriveRepoCleanupQueue(
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts);

} // namespace DexRepoCleanupQueue

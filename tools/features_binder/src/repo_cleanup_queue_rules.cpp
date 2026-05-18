#include "repo_cleanup_queue_rules.h"

#include <algorithm>

namespace DexRepoCleanupQueue {

QStringList sortedUniqueCleanupPaths(QStringList values) {
    values.removeAll("");
    std::sort(values.begin(), values.end());
    values.erase(std::unique(values.begin(), values.end()), values.end());
    return values;
}

QString cleanupIssueForContract(const QString &contractId) {
    if (contractId == "PROTECTED-001") {
        return "protected paths touched";
    }
    if (contractId == "GENERATED-001") {
        return "generated paths touched";
    }
    if (contractId == "UNKNOWN-001") {
        return "unknown paths touched";
    }
    if (contractId == "TEST-001") {
        return "test obligations pending";
    }
    if (contractId == "PROOF-001") {
        return "proof obligations pending";
    }
    return "contract follow-up required";
}

QString cleanupActionForContract(const QString &contractId) {
    if (contractId == "PROTECTED-001") {
        return "stop_for_approval";
    }
    if (contractId == "GENERATED-001") {
        return "verify_generator";
    }
    if (contractId == "UNKNOWN-001") {
        return "classify_path";
    }
    if (contractId == "TEST-001") {
        return "run_tests";
    }
    if (contractId == "PROOF-001") {
        return "attach_proof";
    }
    return "review_contract";
}

QString cleanupStatusForContract(const QString &contractId, const QString &violationStatus) {
    if (contractId == "PROTECTED-001" || violationStatus == "fail") {
        return "blocked";
    }
    if (contractId == "TEST-001" || contractId == "PROOF-001" || violationStatus == "action_required") {
        return "action_required";
    }
    return "open";
}

QString fallbackSeverityForContract(const QString &contractId, const QString &severity) {
    if (!severity.isEmpty() && severity != "unknown") {
        return severity;
    }
    if (contractId == "PROTECTED-001") {
        return "high";
    }
    return "medium";
}

QStringList fallbackPathsForContract(
    const QString &contractId,
    const DexRepoDiffScan::RepoDiffScanState &scan) {
    if (contractId == "PROTECTED-001") {
        return scan.protectedBoundaryPaths;
    }
    if (contractId == "GENERATED-001") {
        return scan.generatedBoundaryPaths;
    }
    if (contractId == "UNKNOWN-001") {
        return scan.unknownBoundaryPaths;
    }
    if (contractId == "TEST-001") {
        return scan.testObligations;
    }
    if (contractId == "PROOF-001") {
        return scan.proofObligations;
    }
    return {};
}

bool containsContract(const QVector<CleanupQueueItem> &items, const QString &contractId) {
    return std::any_of(items.begin(), items.end(), [&contractId](const CleanupQueueItem &item) {
        return item.sourceContract == contractId;
    });
}

CleanupQueueItem makeCleanupItem(
    int sequence,
    const QString &contractId,
    const QString &issue,
    const QString &severity,
    const QString &status,
    const QString &action,
    const QStringList &paths) {
    CleanupQueueItem item;
    item.itemId = QString("clean_%1_%2")
        .arg(sequence, 3, 10, QLatin1Char('0'))
        .arg(QString(contractId).toLower().replace("-", "_"));
    item.sourceContract = contractId;
    item.issue = issue;
    item.severity = severity;
    item.status = status;
    item.recommendedAction = action;
    item.paths = sortedUniqueCleanupPaths(paths);
    item.pathCount = item.paths.size();
    return item;
}

} // namespace DexRepoCleanupQueue

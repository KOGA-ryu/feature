#include "repo_cleanup_queue_state.h"

#include "repo_cleanup_queue_rules.h"

namespace DexRepoCleanupQueue {
namespace {

void appendItemIfNeeded(
    RepoCleanupQueueState *queue,
    int *sequence,
    const QString &contractId,
    const QString &severity,
    const QString &violationStatus,
    const QStringList &paths) {
    const QStringList normalizedPaths = sortedUniqueCleanupPaths(paths);
    if (normalizedPaths.isEmpty()) {
        return;
    }
    queue->items.push_back(makeCleanupItem(
        *sequence,
        contractId,
        cleanupIssueForContract(contractId),
        fallbackSeverityForContract(contractId, severity),
        cleanupStatusForContract(contractId, violationStatus),
        cleanupActionForContract(contractId),
        normalizedPaths));
    ++(*sequence);
}

} // namespace

RepoCleanupQueueState deriveRepoCleanupQueue(
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts) {
    RepoCleanupQueueState queue;
    queue.warnings = scan.warnings + contracts.warnings;

    if (!scan.available && !contracts.available) {
        queue.queueStatus = "no_data";
        queue.available = false;
        return queue;
    }

    int sequence = 1;
    const QStringList contractOrder = {
        "PROTECTED-001",
        "GENERATED-001",
        "UNKNOWN-001",
        "TEST-001",
        "PROOF-001",
    };

    for (const QString &contractId : contractOrder) {
        for (const DexRepoContractCheck::ContractViolationRecord &violation : contracts.violations) {
            if (violation.contractId != contractId) {
                continue;
            }
            QStringList paths = violation.paths;
            if (contractId == "TEST-001" || contractId == "PROOF-001") {
                paths = fallbackPathsForContract(contractId, scan);
            }
            if (paths.isEmpty()) {
                paths = fallbackPathsForContract(contractId, scan);
            }
            appendItemIfNeeded(
                &queue,
                &sequence,
                contractId,
                violation.severity,
                violation.status,
                paths);
        }
    }

    const struct {
        const char *contractId;
        QStringList paths;
    } fallbackItems[] = {
        {"PROTECTED-001", scan.protectedBoundaryPaths},
        {"GENERATED-001", scan.generatedBoundaryPaths},
        {"UNKNOWN-001", scan.unknownBoundaryPaths},
        {"TEST-001", scan.testObligations},
        {"PROOF-001", scan.proofObligations},
    };

    for (const auto &fallback : fallbackItems) {
        const QString contractId = QString::fromUtf8(fallback.contractId);
        if (containsContract(queue.items, contractId)) {
            continue;
        }
        appendItemIfNeeded(
            &queue,
            &sequence,
            contractId,
            QString(),
            contractId == "PROTECTED-001" ? QString("fail") : QString("action_required"),
            fallback.paths);
    }

    for (const CleanupQueueItem &item : queue.items) {
        if (item.severity == "high") {
            ++queue.highCount;
        } else if (item.severity == "medium") {
            ++queue.mediumCount;
        }
        if (item.status == "blocked" || item.status == "action_required") {
            ++queue.actionRequiredCount;
        }
    }

    queue.available = scan.available || contracts.available;
    queue.queueStatus = queue.items.isEmpty() ? "clear" : "open";
    return queue;
}

} // namespace DexRepoCleanupQueue

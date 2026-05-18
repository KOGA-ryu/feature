#include "repo_context_sections.h"

#include <QVBoxLayout>

#include "app_state_helpers.h"
#include "render_helpers.h"
#include "right_context_render_helpers.h"

namespace DexRightContext {

void addRepoSummaryContext(
    QVBoxLayout *contentLayout,
    const CockpitState &state,
    const QString &selectedWorkerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &selectedTopTab,
    const QString &selectedDetailLens) {
    const WorkerSummary worker = workerSummaryForId(state, selectedWorkerId);
    addContextSection(contentLayout, "selected repo");
    addWrappedLine(contentLayout, DexProjects::displayName(project));
    addWrappedLine(contentLayout, "active subject                 " + selectedTopTab);
    addWrappedLine(contentLayout, "detail lens                    " + selectedDetailLens);
    addWrappedLine(contentLayout, "path                           " + (project.path.isEmpty() ? QString("not set") : project.path));
    addWrappedLine(contentLayout, "role                           " + (project.role.isEmpty() ? QString("not set") : project.role));
    addWrappedLine(contentLayout, "authority                      " + (project.authority.isEmpty() ? QString("not set") : project.authority));
    addWrappedLine(contentLayout, "binder template                " + projectBinderTemplate(project));
    addWrappedLine(contentLayout, "promotion                      explicit only");
    addWrappedLine(contentLayout, "editor core                    " + (project.editorCoreStatus.isEmpty() ? QString("not set") : project.editorCoreStatus));

    addContextSection(contentLayout, "selected worker");
    addWrappedLine(contentLayout, "worker                         " + worker.displayName);
    addWrappedLine(contentLayout, "worker id                      " + worker.id);
    addWrappedLine(contentLayout, "role                           " + worker.role);
    addWrappedLine(contentLayout, "status                         " + worker.status);
    addWrappedLine(contentLayout, "codex session                  " + (worker.codexSessionId.isEmpty() ? QString("not linked") : worker.codexSessionId));
    addWrappedLine(contentLayout, "codex model                    " + (worker.codexModel.isEmpty() ? QString("not linked") : worker.codexModel));
    addWrappedLine(contentLayout, "codex directory                " + (worker.codexDirectory.isEmpty() ? QString("not linked") : worker.codexDirectory));
    addWrappedLine(contentLayout, "codex permissions              " + (worker.codexPermissions.isEmpty() ? QString("not linked") : worker.codexPermissions));
    addWrappedLine(contentLayout, "linked context snapshot        " + (worker.codexContextWindow.isEmpty() ? QString("not linked") : worker.codexContextWindow));
    addWrappedLine(contentLayout, "selection scope                repo binder lens");
}

void addRepoScanAndContractContext(
    QVBoxLayout *contentLayout,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts) {
    addContextSection(contentLayout, "diff-scan summary");
    addWrappedLine(contentLayout, "scan status                    " + scan.scanStatus);
    addWrappedLine(contentLayout, "risk level                     " + scan.riskLevel);
    addWrappedLine(contentLayout, "changed files                  " + QString::number(scan.changedFiles.size()));
    addWrappedLine(contentLayout, "protected paths                " + QString::number(scan.protectedBoundaryPaths.size()));
    addWrappedLine(contentLayout, "generated paths                " + QString::number(scan.generatedBoundaryPaths.size()));
    addWrappedLine(contentLayout, "unknown paths                  " + QString::number(scan.unknownBoundaryPaths.size()));
    addWrappedLine(contentLayout, "test obligations              " + QString::number(scan.testObligations.size()));
    addWrappedLine(contentLayout, "proof obligations             " + QString::number(scan.proofObligations.size()));
    addWrappedLine(contentLayout, "recommended next step          " + (scan.recommendedNextStep.isEmpty() ? QString("unknown") : scan.recommendedNextStep));
    addWrappedLine(contentLayout, "first warning                  " + (scan.warnings.isEmpty() ? QString("none") : scan.warnings.first()));

    addContextSection(contentLayout, "contract-check summary");
    addWrappedLine(contentLayout, "contract status                " + contracts.contractStatus);
    addWrappedLine(contentLayout, "contract violations            " + QString::number(contracts.violations.size()));
    addWrappedLine(contentLayout, "contract next                  " + (contracts.recommendedNextStep.isEmpty() ? QString("unknown") : contracts.recommendedNextStep));
    addWrappedLine(contentLayout, "first contract warning         " + (contracts.warnings.isEmpty() ? QString("none") : contracts.warnings.first()));
}

void addRepoPromotionCleanupAndProofContext(
    QVBoxLayout *contentLayout,
    const DexRepoPromotionReport::RepoPromotionReportState &promotionReport,
    const DexRepoCleanupQueue::RepoCleanupQueueState &cleanupQueue,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt) {
    addContextSection(contentLayout, "promotion dry-run summary");
    addWrappedLine(contentLayout, "mode                           " + promotionReport.mode);
    addWrappedLine(contentLayout, "compared files                 " + QString::number(promotionReport.comparedCount));
    addWrappedLine(contentLayout, "blocked files                  " + QString::number(promotionReport.blockedCount));
    addWrappedLine(contentLayout, "authority writes               " + promotionReport.authorityWrites);
    addWrappedLine(contentLayout, "created at                     " + promotionReport.createdAt);

    addContextSection(contentLayout, "cleanup queue summary");
    addWrappedLine(contentLayout, "queue status                   " + cleanupQueue.queueStatus);
    addWrappedLine(contentLayout, "cleanup items                  " + QString::number(cleanupQueue.items.size()));
    addWrappedLine(contentLayout, "high severity                  " + QString::number(cleanupQueue.highCount));
    addWrappedLine(contentLayout, "blocked/action                 " + QString::number(cleanupQueue.actionRequiredCount));
    if (!cleanupQueue.items.isEmpty()) {
        const DexRepoCleanupQueue::CleanupQueueItem firstItem = cleanupQueue.items.first();
        addWrappedLine(contentLayout, "first cleanup                  " + firstItem.sourceContract + " -> " + firstItem.recommendedAction);
    }

    addContextSection(contentLayout, "proof receipt summary");
    addWrappedLine(contentLayout, "receipt status                 " + proofReceipt.receiptStatus);
    addWrappedLine(contentLayout, "run id                         " + proofReceipt.runId);
    addWrappedLine(contentLayout, "created at                     " + proofReceipt.createdAt);
    addWrappedLine(contentLayout, "proof type                     " + proofReceipt.proofType);
    addWrappedLine(contentLayout, "manifest tab                   " + proofReceipt.tab);
    addWrappedLine(contentLayout, "detail lens                    " + proofReceipt.detailLens);
    addWrappedLine(contentLayout, "run manifest                   " + (proofReceipt.runManifestPath.isEmpty() ? QString("none") : proofReceipt.runManifestPath));
    addWrappedLine(contentLayout, "screenshots                    " + QString::number(proofReceipt.screenshots.size()));
    addWrappedLine(contentLayout, "command receipts               " + QString::number(proofReceipt.commandReceipts.size()));
    addWrappedLine(contentLayout, "first receipt warning          " + (proofReceipt.warnings.isEmpty() ? QString("none") : proofReceipt.warnings.first()));
}

} // namespace DexRightContext

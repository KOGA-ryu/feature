#include "repo_context_renderer.h"

#include <QVBoxLayout>

#include "repo_cleanup_queue_state.h"
#include "repo_context_sections.h"

namespace DexRightContext {

void addRepoContext(
    QVBoxLayout *contentLayout,
    const CockpitState &state,
    const QString &selectedWorkerId,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoBinderTemplate::BinderTemplate &binderTemplate,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt,
    const DexRepoPromotionReport::RepoPromotionReportState &promotionReport,
    const QString &selectedTopTab,
    const QString &selectedDetailLens) {
    const DexRepoCleanupQueue::RepoCleanupQueueState cleanupQueue =
        DexRepoCleanupQueue::deriveRepoCleanupQueue(scan, contracts);

    addRepoSummaryContext(contentLayout, state, selectedWorkerId, project, selectedTopTab, selectedDetailLens);
    addTemplateContextLines(contentLayout, binderTemplate, project, scan, contracts, proofReceipt);
    addRepoScanAndContractContext(contentLayout, scan, contracts);
    addRepoPromotionCleanupAndProofContext(contentLayout, promotionReport, cleanupQueue, proofReceipt);
    addRepoTopTabContext(
        contentLayout,
        selectedTopTab,
        project,
        scan,
        contracts,
        promotionReport,
        cleanupQueue,
        proofReceipt);
    addRepoContextActions(contentLayout);
}

} // namespace DexRightContext

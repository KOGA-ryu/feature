#pragma once

#include <QString>

#include "app_state.h"
#include "project_registry.h"
#include "repo_binder_template.h"
#include "repo_cleanup_queue_state.h"
#include "repo_contract_check_state.h"
#include "repo_diff_scan_state.h"
#include "repo_promotion_report_state.h"
#include "repo_proof_receipt_state.h"

class QVBoxLayout;

namespace DexRightContext {

void addTemplateContextLines(
    QVBoxLayout *contentLayout,
    const DexRepoBinderTemplate::BinderTemplate &binderTemplate,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt);

void addRepoSummaryContext(
    QVBoxLayout *contentLayout,
    const CockpitState &state,
    const QString &selectedWorkerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &selectedTopTab,
    const QString &selectedDetailLens);

void addRepoScanAndContractContext(
    QVBoxLayout *contentLayout,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts);

void addRepoPromotionCleanupAndProofContext(
    QVBoxLayout *contentLayout,
    const DexRepoPromotionReport::RepoPromotionReportState &promotionReport,
    const DexRepoCleanupQueue::RepoCleanupQueueState &cleanupQueue,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt);

void addRepoTopTabContext(
    QVBoxLayout *contentLayout,
    const QString &selectedTopTab,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoPromotionReport::RepoPromotionReportState &promotionReport,
    const DexRepoCleanupQueue::RepoCleanupQueueState &cleanupQueue,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt);

void addRepoContextActions(QVBoxLayout *contentLayout);

} // namespace DexRightContext

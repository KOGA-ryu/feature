#pragma once

#include <QString>
#include <QStringList>

#include "app_state.h"
#include "project_registry.h"
#include "repo_binder_template.h"
#include "repo_cleanup_queue_state.h"
#include "repo_contract_check_state.h"
#include "repo_diff_scan_state.h"
#include "repo_proof_receipt_state.h"

class QVBoxLayout;

namespace DexBinderPages {

void addListRows(QVBoxLayout *layout, const QString &label, const QStringList &values, bool risk = false);
void addSelectedWorkerLens(
    QVBoxLayout *layout,
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &topTab,
    const QString &detailLens);

QString renderTemplateText(
    QString text,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt);

QStringList renderTemplateCells(
    const QStringList &cells,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt);

void addTemplateSections(
    QVBoxLayout *layout,
    const DexRepoBinderTemplate::BinderTemplate &binderTemplate,
    const QString &tab,
    const QString &lens,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt);

void addCleanupQueueRows(QVBoxLayout *layout, const DexRepoCleanupQueue::RepoCleanupQueueState &queue);

} // namespace DexBinderPages

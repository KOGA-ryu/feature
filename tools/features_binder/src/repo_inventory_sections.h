#pragma once

#include <QString>

#include "project_registry.h"
#include "repo_binder_template.h"
#include "repo_contract_check_state.h"
#include "repo_diff_scan_state.h"
#include "repo_proof_receipt_state.h"

class QVBoxLayout;

namespace DexBinderPages {
void addRepoInventoryContent(
    QVBoxLayout *layout,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &lens,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt,
    const DexRepoBinderTemplate::BinderTemplate &binderTemplate);

} // namespace DexBinderPages

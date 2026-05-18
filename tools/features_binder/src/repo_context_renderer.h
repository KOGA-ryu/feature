#pragma once

#include <QString>

#include "app_state.h"
#include "project_registry.h"
#include "repo_binder_template.h"

class QVBoxLayout;

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
    const QString &selectedDetailLens);

} // namespace DexRightContext

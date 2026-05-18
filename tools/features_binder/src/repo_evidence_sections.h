#pragma once

#include <QString>

#include "project_registry.h"
#include "repo_binder_template.h"
#include "repo_contract_check_state.h"
#include "repo_diff_scan_state.h"
#include "repo_proof_receipt_state.h"
#include "repo_promotion_report_state.h"

class QVBoxLayout;

namespace DexBinderPages {
void addRepoEvidenceContent(
    QVBoxLayout *layout,
    const QString &lens,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt,
    const DexRepoPromotionReport::RepoPromotionReportState &promotionReport);

} // namespace DexBinderPages

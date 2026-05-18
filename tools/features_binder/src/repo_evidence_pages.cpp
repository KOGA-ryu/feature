#include "repo_binder_pages.h"

#include <QVBoxLayout>
#include <QtGlobal>

#include "binder_page_helpers.h"
#include "repo_binder_page_helpers.h"
#include "repo_evidence_sections.h"

namespace DexBinderPages {

QWidget *buildRepoEvidencePage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens) {
    return buildCompactPage([state,
                             workerId,
                             project,
                             detailLens,
                             proofReceipt = state.repoProofReceipt,
                             promotionReport = state.repoPromotionReport](QVBoxLayout *layout) {
        addSelectedWorkerLens(layout, state, workerId, project, "Evidence", detailLens);
        addRepoEvidenceContent(layout, detailLens, proofReceipt, promotionReport);
    });
}

} // namespace DexBinderPages

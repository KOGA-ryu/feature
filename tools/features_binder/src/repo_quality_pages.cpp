#include "repo_binder_pages.h"

#include <QVBoxLayout>
#include <QtGlobal>

#include "binder_page_helpers.h"
#include "repo_binder_page_helpers.h"
#include "repo_quality_sections.h"

namespace DexBinderPages {

QWidget *buildRepoQualityPage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens,
    const DexRepoBinderTemplate::BinderTemplate &binderTemplate) {
    return buildCompactPage([project,
                             state,
                             workerId,
                             detailLens,
                             binderTemplate,
                             scan = state.repoDiffScan,
                             contracts = state.repoContractCheck,
                             proofReceipt = state.repoProofReceipt](QVBoxLayout *layout) {
        addSelectedWorkerLens(layout, state, workerId, project, "Quality", detailLens);
        addRepoQualityContent(layout, detailLens, project, scan, contracts, proofReceipt, binderTemplate);
    });
}

} // namespace DexBinderPages

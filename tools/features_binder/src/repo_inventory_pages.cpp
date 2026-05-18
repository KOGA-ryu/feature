#include "repo_binder_pages.h"

#include <QVBoxLayout>
#include <QtGlobal>

#include "binder_page_helpers.h"
#include "repo_binder_page_helpers.h"
#include "repo_inventory_sections.h"

namespace DexBinderPages {

QWidget *buildRepoInventoryPage(
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
        addSelectedWorkerLens(layout, state, workerId, project, "Inventory", detailLens);
        addRepoInventoryContent(layout, project, detailLens, scan, contracts, proofReceipt, binderTemplate);
    });
}

} // namespace DexBinderPages

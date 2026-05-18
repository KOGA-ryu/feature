#include "repo_binder_pages.h"

#include <QGridLayout>
#include <QHBoxLayout>
#include <QSizePolicy>
#include <QVBoxLayout>
#include <QtGlobal>

#include "binder_page_helpers.h"
#include "repo_binder_page_helpers.h"
#include "render_helpers.h"
#include "repo_binder_page_helpers.h"
#include "repo_cleanup_queue_state.h"

namespace DexBinderPages {
namespace {

void addRepoAuthorityPage(QVBoxLayout *layout, const QString &lens, const DexProjects::ProjectRegistryEntry &project) {
    layout->addWidget(makeStatsText("Purpose: files that control the repo. Authority files are load-bearing and need proof when changed."));
    auto *files = makeStatsSection("authority files", true);
    auto *filesLayout = static_cast<QVBoxLayout *>(files->layout());
    filesLayout->addWidget(makeStatsRow({"path", "authority source", "status", "proof"}, false, false));
    if (project.protectedZones.isEmpty()) {
        filesLayout->addWidget(makeStatsRow({"protected zones", "registry", "not set", "open Settings"}, false, false));
    }
    for (const QString &zone : compactLines(project.protectedZones)) {
        filesLayout->addWidget(makeStatsRow({zone, "registry protected zone", "inspect first", "required when changed"}, true, false));
    }
    layout->addWidget(files);

    if (lens == "Dashboard" || lens == "Drift") {
        auto *drift = makeStatsSection("drift checks");
        auto *driftLayout = static_cast<QVBoxLayout *>(drift->layout());
        driftLayout->addWidget(makeStatsText("drift checks are not authored in the blank binder. Contract-check and diff-scan can report boundary facts after a project is saved."));
        layout->addWidget(drift);
    }
}

} // namespace

QWidget *buildRepoAuthorityPage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens) {
    return buildCompactPage([state, workerId, project, detailLens](QVBoxLayout *layout) {
        addSelectedWorkerLens(layout, state, workerId, project, "Authority", detailLens);
        addRepoAuthorityPage(layout, detailLens, project);
    });
}

} // namespace DexBinderPages

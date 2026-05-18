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

void addRepoMapPage(QVBoxLayout *layout, const QString &lens, const DexProjects::ProjectRegistryEntry &project) {
    layout->addWidget(makeStatsText("Purpose: repo zones. Map says where things live, what belongs there, and what an agent may edit."));
    auto *zones = makeStatsSection("repo zones", true);
    auto *zonesLayout = static_cast<QVBoxLayout *>(zones->layout());
    zonesLayout->addWidget(makeStatsRow({"zone", "source", "status", "agent rule"}, false, false));
    if (project.safeEditZones.isEmpty() && project.protectedZones.isEmpty() && project.generatedZones.isEmpty() && project.scratchZones.isEmpty()) {
        zonesLayout->addWidget(makeStatsRow({"zones", "registry", "not set", "open Settings"}, false, false));
    }
    for (const QString &zone : compactLines(project.safeEditZones)) {
        zonesLayout->addWidget(makeStatsRow({zone, "registry", "safe edit", "allowed when task scope permits"}, false, false));
    }
    for (const QString &zone : compactLines(project.protectedZones)) {
        zonesLayout->addWidget(makeStatsRow({zone, "registry", "protected", "explicit approval first"}, true, false));
    }
    for (const QString &zone : compactLines(project.generatedZones)) {
        zonesLayout->addWidget(makeStatsRow({zone, "registry", "generated", "do not hand edit"}, true, false));
    }
    for (const QString &zone : compactLines(project.scratchZones)) {
        zonesLayout->addWidget(makeStatsRow({zone, "registry", "scratch", "local-only work"}, false, false));
    }
    layout->addWidget(zones);

    if (lens == "Dashboard" || lens == "Folders") {
        auto *folders = makeStatsSection("folder purpose check");
        auto *foldersLayout = static_cast<QVBoxLayout *>(folders->layout());
        foldersLayout->addWidget(makeStatsRow({"folder", "source", "purpose", "next"}, false, false));
        const QStringList allZones = compactLines(project.safeEditZones + project.protectedZones + project.generatedZones + project.scratchZones);
        if (allZones.isEmpty()) {
            foldersLayout->addWidget(makeStatsText("folder purposes are not set in the registry."));
        }
        for (const QString &zone : allZones) {
            foldersLayout->addWidget(makeStatsRow({zone, "registry", "not described", "fill out Settings fields"}, false, false));
        }
        layout->addWidget(folders);
    }
}

} // namespace

QWidget *buildRepoMapPage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens) {
    return buildCompactPage([state, workerId, project, detailLens](QVBoxLayout *layout) {
        addSelectedWorkerLens(layout, state, workerId, project, "Map", detailLens);
        addRepoMapPage(layout, detailLens, project);
    });
}

} // namespace DexBinderPages

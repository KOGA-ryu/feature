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

void addRepoActivityPage(
    QVBoxLayout *layout,
    const QString &lens,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts) {
    const DexRepoCleanupQueue::RepoCleanupQueueState cleanupQueue =
        DexRepoCleanupQueue::deriveRepoCleanupQueue(scan, contracts);
    layout->addWidget(makeStatsText("Purpose: scan, monitor, report, changes, and cleanup queue. Activity records facts and active work, not long-term grades."));
    auto *state = makeStatsSection("top repo hud", true);
    auto *stateLayout = static_cast<QVBoxLayout *>(state->layout());
    stateLayout->addWidget(makeStatsRow(
        {"scan status", scan.scanStatus,
         "risk level", scan.riskLevel,
         "recommended", scan.recommendedNextStep,
         "changed", QString::number(scan.changedFiles.size()),
         "cleanup", QString::number(cleanupQueue.items.size())},
        true,
        false));
    layout->addWidget(state);
    if (lens == "Dashboard" || lens == "Scan") {
        auto *section = makeStatsSection("repo scan facts");
        auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
        sectionLayout->addWidget(makeStatsRow(
            {"protected", QString::number(scan.protectedBoundaryPaths.size()),
             "generated", QString::number(scan.generatedBoundaryPaths.size()),
             "unknown", QString::number(scan.unknownBoundaryPaths.size())},
            true,
            false));
        if (scan.warnings.isEmpty()) {
            sectionLayout->addWidget(makeStatsText("no warnings"));
        } else {
            sectionLayout->addWidget(makeStatsText("warnings"));
            for (const QString &warning : scan.warnings) {
                sectionLayout->addWidget(makeStatsRow({warning}, true, false));
            }
        }
        layout->addWidget(section);
    }
    if (lens == "Dashboard" || lens == "Monitor") {
        auto *section = makeStatsSection("repo monitor warnings", true);
        auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
        if (scan.warnings.isEmpty() && contracts.warnings.isEmpty()) {
            sectionLayout->addWidget(makeStatsText("monitor warnings: none reported"));
        }
        for (const QString &warning : scan.warnings) {
            sectionLayout->addWidget(makeStatsRow({"diff-scan", warning}, true, false));
        }
        for (const QString &warning : contracts.warnings) {
            sectionLayout->addWidget(makeStatsRow({"contract-check", warning}, true, false));
        }
        layout->addWidget(section);
    }
    if (lens == "Dashboard" || lens == "Report") {
        auto *section = makeStatsSection("repo report windows");
        auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
        sectionLayout->addWidget(makeStatsText("report windows are not wired. Saved registry fields and backend scan facts are the only active data sources."));
        layout->addWidget(section);
    }
    if (lens == "Dashboard" || lens == "Changes") {
        auto *section = makeStatsSection("changed files");
        auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
        sectionLayout->addWidget(makeStatsRow({"path", "change type", "boundary", "risk"}, false, false));
        if (scan.changedFiles.isEmpty()) {
            sectionLayout->addWidget(makeStatsText("no changed files"));
        } else {
            for (const DexRepoDiffScan::ChangedFileRecord &record : scan.changedFiles) {
                sectionLayout->addWidget(makeStatsRow(
                    {
                        record.path,
                        record.changeType.isEmpty() ? QString("unknown") : record.changeType,
                        record.boundary.isEmpty() ? QString("unknown") : record.boundary,
                        record.riskLevel.isEmpty() ? QString("unknown") : record.riskLevel,
                    },
                    true,
                    false));
            }
        }
        layout->addWidget(section);
    }
    if (lens == "Dashboard" || lens == "Cleanup") {
        auto *section = makeStatsSection("cleanup queue", true);
        auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
        sectionLayout->addWidget(makeStatsRow(
            {
                "status", cleanupQueue.queueStatus,
                "items", QString::number(cleanupQueue.items.size()),
                "blocked/action", QString::number(cleanupQueue.actionRequiredCount),
            },
            cleanupQueue.queueStatus == "open",
            false));
        addCleanupQueueRows(sectionLayout, cleanupQueue);
        layout->addWidget(section);
    }
}

} // namespace

QWidget *buildRepoActivityPage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens) {
    return buildCompactPage([state, workerId, project, detailLens, scan = state.repoDiffScan, contracts = state.repoContractCheck](QVBoxLayout *layout) {
        addSelectedWorkerLens(layout, state, workerId, project, "Activity", detailLens);
        addRepoActivityPage(layout, detailLens, scan, contracts);
    });
}

} // namespace DexBinderPages

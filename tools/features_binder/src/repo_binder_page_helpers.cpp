#include "repo_binder_page_helpers.h"

#include <QVBoxLayout>

#include "app_state_helpers.h"
#include "binder_page_helpers.h"
#include "render_helpers.h"

namespace DexBinderPages {

void addListRows(QVBoxLayout *layout, const QString &label, const QStringList &values, bool risk) {
    for (const QString &value : compactLines(values)) {
        layout->addWidget(makeStatsRow({label, value}, risk, false));
    }
}

void addSelectedWorkerLens(
    QVBoxLayout *layout,
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &topTab,
    const QString &detailLens) {
    const WorkerSummary worker = workerSummaryForId(state, workerId);
    auto *section = makeStatsSection("selected worker lens", true);
    auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
    sectionLayout->addWidget(makeStatsRow({
        "worker id",
        worker.id,
        "display",
        worker.displayName,
        "role",
        worker.role,
    }, false, false));
    sectionLayout->addWidget(makeStatsRow({
        "status",
        worker.status,
        "active project",
        DexProjects::displayName(project),
        "subject",
        topTab,
        "detail lens",
        detailLens,
    }, false, false));
    sectionLayout->addWidget(makeStatsRow({
        "codex session",
        worker.codexSessionId.isEmpty() ? QString("not linked") : worker.codexSessionId,
        "model",
        worker.codexModel.isEmpty() ? QString("not linked") : worker.codexModel,
        "directory",
        worker.codexDirectory.isEmpty() ? QString("not linked") : worker.codexDirectory,
    }, false, false));
    sectionLayout->addWidget(makeStatsText("Worker selection is a repo-scoped lens. It does not execute workers or write records."));
    layout->addWidget(section);
}

void addCleanupQueueRows(QVBoxLayout *layout, const DexRepoCleanupQueue::RepoCleanupQueueState &queue) {
    layout->addWidget(makeStatsRow({"item id", "source", "paths", "severity", "action", "status"}, false, false));
    if (queue.items.isEmpty()) {
        const QString emptyText = queue.queueStatus == "clear"
            ? QString("cleanup queue clear")
            : QString("cleanup queue has no data");
        layout->addWidget(makeStatsText(emptyText));
        if (!queue.warnings.isEmpty()) {
            for (const QString &warning : queue.warnings) {
                layout->addWidget(makeStatsRow({"warning", warning}, true, false));
            }
        }
        return;
    }
    for (const DexRepoCleanupQueue::CleanupQueueItem &item : queue.items) {
        layout->addWidget(makeStatsRow(
            {
                item.itemId,
                item.sourceContract,
                QString::number(item.pathCount),
                item.severity,
                item.recommendedAction,
                item.status,
            },
            item.severity == "high" || item.status == "blocked",
            false));
    }
}

} // namespace DexBinderPages

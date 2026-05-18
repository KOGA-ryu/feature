#include "agent_empty_state.h"

#include <QVBoxLayout>

#include "app_state_helpers.h"
#include "binder_page_helpers.h"
#include "binder_state.h"
#include "render_helpers.h"
#include "right_context_render_helpers.h"

namespace {

QString purposeForSubject(const QString &subject) {
    if (subject == "Profile") {
        return "Purpose: worker operating envelope. Blank until a saved project assigns a worker and real profile evidence exists.";
    }
    if (subject == "Stats") {
        return "Purpose: aggregate routing patterns. Blank until real grade, transcript, and evidence records exist.";
    }
    if (subject == "Relationship") {
        return "Purpose: handoff reliability between roles. Not wired until real handoff records exist.";
    }
    if (subject == "Grade") {
        return "Purpose: single-run scoring for future routing. Blank until typed grade records exist.";
    }
    if (subject == "Transcript") {
        return "Purpose: raw session facts and references. Blank until transcript records exist.";
    }
    if (subject == "Evidence") {
        return "Purpose: proof, failure, correction, and acceptance chains. Blank until evidence records exist.";
    }
    return "Purpose: agent binder subject. Blank until real records exist.";
}

QString recordSourceForSubject(const QString &subject) {
    if (subject == "Profile") {
        return "project registry worker roster + future profile evidence";
    }
    if (subject == "Stats") {
        return "typed grade/transcript/evidence records";
    }
    if (subject == "Relationship") {
        return "typed handoff records";
    }
    if (subject == "Grade") {
        return "typed grade records";
    }
    if (subject == "Transcript") {
        return "typed transcript records";
    }
    if (subject == "Evidence") {
        return "typed evidence records";
    }
    return "typed records";
}

int recordCountForSubject(const CockpitState &state, const QString &workerId, const QString &subject) {
    if (subject == "Stats") {
        return state.binder.statsSnapshot.sampleSize;
    }
    if (subject == "Grade") {
        return DexBinder::gradeRecordsForWorker(state.binder, workerId).size();
    }
    if (subject == "Transcript") {
        return DexBinder::transcriptRecordsForWorker(state.binder, workerId).size();
    }
    if (subject == "Evidence") {
        return DexBinder::evidenceRecordsForWorker(state.binder, workerId).size();
    }
    return 0;
}

QString recordLabelForSubject(const QString &subject) {
    if (subject == "Stats") {
        return "sample size";
    }
    if (subject == "Relationship") {
        return "handoff records";
    }
    return "records";
}

WorkerSummary selectedWorkerSummary(const CockpitState &state, const QString &workerId) {
    if (workerId.isEmpty()) {
        return {"", "", "No worker selected", "none"};
    }
    return workerSummaryForId(state, workerId);
}

} // namespace

namespace DexBinderPages {

void addAgentEmptyStatePage(
    QVBoxLayout *layout,
    const CockpitState &state,
    const QString &workerId,
    const QString &subject,
    const QString &detailLens) {
    const WorkerSummary worker = selectedWorkerSummary(state, workerId);
    const int recordCount = recordCountForSubject(state, workerId, subject);

    layout->addWidget(makeStatsText(purposeForSubject(subject)));

    auto *selection = makeStatsSection("selected worker", true);
    auto *selectionLayout = static_cast<QVBoxLayout *>(selection->layout());
    selectionLayout->addWidget(makeStatsRow({
        "worker id",
        worker.id.isEmpty() ? QString("none") : worker.id,
        "display",
        worker.displayName,
        "role",
        worker.role.isEmpty() ? QString("none") : worker.role,
    }, false, false));
    selectionLayout->addWidget(makeStatsRow({
        "status",
        worker.status,
        "subject",
        subject,
        "detail lens",
        detailLens,
    }, false, false));
    layout->addWidget(selection);

    auto *records = makeStatsSection(subject.toLower() + " records", true);
    auto *recordsLayout = static_cast<QVBoxLayout *>(records->layout());
    recordsLayout->addWidget(makeStatsRow({
        "source",
        recordSourceForSubject(subject),
        recordLabelForSubject(subject),
        QString::number(recordCount),
        "state",
        recordCount > 0 ? QString("available") : QString("empty"),
    }, recordCount == 0, false));
    if (recordCount == 0) {
        recordsLayout->addWidget(makeStatsText("No record-backed content is available. No template/demo rows are shown."));
    }
    layout->addWidget(records);
}

} // namespace DexBinderPages

namespace DexRightContext {

void addAgentEmptyStateContext(
    QVBoxLayout *contentLayout,
    const CockpitState &state,
    const QString &selectedWorkerId,
    const QString &selectedTopTab,
    const QString &selectedDetailLens) {
    const WorkerSummary worker = selectedWorkerSummary(state, selectedWorkerId);
    const int recordCount = recordCountForSubject(state, selectedWorkerId, selectedTopTab);

    addContextSection(contentLayout, "selected worker");
    addWrappedLine(contentLayout, "worker                         " + worker.displayName);
    addWrappedLine(contentLayout, "worker id                      " + (worker.id.isEmpty() ? QString("none") : worker.id));
    addWrappedLine(contentLayout, "role                           " + (worker.role.isEmpty() ? QString("none") : worker.role));
    addWrappedLine(contentLayout, "status                         " + worker.status);

    addContextSection(contentLayout, "agent binder");
    addWrappedLine(contentLayout, "subject                        " + selectedTopTab);
    addWrappedLine(contentLayout, "detail lens                    " + selectedDetailLens);
    addWrappedLine(contentLayout, "source                         " + recordSourceForSubject(selectedTopTab));
    addWrappedLine(contentLayout, recordLabelForSubject(selectedTopTab) + "                         " + QString::number(recordCount));
    addWrappedLine(contentLayout, "content                        " + QString(recordCount > 0 ? "record-backed" : "empty"));
    addWrappedLine(contentLayout, "template rows                  disabled");
}

} // namespace DexRightContext

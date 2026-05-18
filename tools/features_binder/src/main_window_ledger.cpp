#include "main_window.h"

#include <QDateTime>
#include <QJsonObject>
#include <QStatusBar>
#include <QUuid>

#include "app_state_helpers.h"
#include "render_helpers.h"

void DexHomeV2Window::addGradeRecord() {
    appendLedgerAction("grade_recorded", "review", "B", QString(), "Manual cockpit grade record added from the Qt shell.");
}

void DexHomeV2Window::addReviewVerdict(const QString &decision) {
    const QString grade = decision == "accepted" ? "A" : (decision == "rejected" ? "D" : QString());
    appendLedgerAction(
        "review_verdict_recorded",
        decision,
        grade,
        decision,
        "Review verdict recorded from the native cockpit.");
}

void DexHomeV2Window::appendLedgerAction(
    const QString &kind,
    const QString &status,
    const QString &grade,
    const QString &decision,
    const QString &summary) {
    QJsonObject record;
    record["schema_version"] = 1;
    record["record_id"] = "qt-" + QUuid::createUuid().toString(QUuid::WithoutBraces);
    record["timestamp"] = QDateTime::currentDateTimeUtc().toString(Qt::ISODateWithMs);
    record["kind"] = kind;
    record["project_id"] = state_.selectedProjectId;
    record["project_path"] = backend_.repoRoot();
    record["worker_role"] = selectedWorkerId_;
    record["session_id"] = "native-ui-manual";
    record["title"] = workerDisplayName(state_, selectedWorkerId_) + " " + kind + ".";
    record["summary"] = summary;
    record["status"] = status;
    record["grade"] = grade;
    record["proof_refs"] = toJsonArray({"native-ui"});
    record["transcript_refs"] = toJsonArray({});
    record["changed_files"] = toJsonArray({});
    record["reviewer_notes"] = toJsonArray({summary});
    record["packet_id"] = "";
    record["decision"] = decision;
    record["proof_status"] = "manual";
    record["tags"] = toJsonArray({"manual", "qt"});
    record["source"] = "qt_native_shell";

    QString error;
    if (!backend_.appendLedgerRecord(record, &error)) {
        statusBar()->showMessage(error);
        return;
    }
    statusBar()->showMessage("Added " + kind + " for " + selectedWorkerId_, 5000);
    reloadState();
}

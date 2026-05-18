#include "rust_cockpit_backend.h"

#include <QJsonArray>
#include <QJsonObject>

#include "app_state_helpers.h"
#include "binder_state.h"

CockpitState RustCockpitBackend::parseState(const QJsonDocument &document) const {
    CockpitState state;
    const QJsonObject root = document.object();
    state.schemaVersion = root.value("schema_version").toInt(1);
    state.selectedProjectId = readString(root, "selected_project_id", "dex_home");
    state.selectedWorkerId = readString(root, "selected_worker_id", "planner");

    const QJsonArray projects = root.value("projects").toArray();
    for (const QJsonValue &value : projects) {
        const QJsonObject object = value.toObject();
        ProjectSummary project;
        project.id = readString(object, "project_id");
        project.name = readString(object, "name");
        project.path = readString(object, "path");
        project.status = readString(object, "status");
        project.pinned = readBool(object, "pinned");
        project.workerIds = readStringList(object, "worker_ids");
        state.projects.push_back(project);
    }

    const QJsonArray workers = root.value("workers").toArray();
    for (const QJsonValue &value : workers) {
        const QJsonObject object = value.toObject();
        WorkerSummary worker;
        worker.id = readString(object, "worker_id");
        worker.role = readString(object, "role");
        worker.displayName = readString(object, "display_name");
        worker.status = readString(object, "status");
        state.workers.push_back(worker);
    }

    const QJsonArray entries = root.value("ledger_entries").toArray();
    for (const QJsonValue &value : entries) {
        const QJsonObject object = value.toObject();
        LedgerEntry entry;
        entry.recordId = readString(object, "record_id");
        entry.sequence = readString(object, "sequence");
        entry.timestamp = readString(object, "timestamp");
        entry.kind = readString(object, "kind");
        entry.projectId = readString(object, "project_id");
        entry.projectPath = readString(object, "project_path");
        entry.workerRole = readString(object, "worker_role");
        entry.sessionId = readString(object, "session_id");
        entry.title = readString(object, "title");
        entry.summary = readString(object, "summary");
        entry.status = readString(object, "status");
        entry.grade = readString(object, "grade");
        entry.notes = readStringList(object, "notes");
        entry.proofRefs = readStringList(object, "proof_refs");
        entry.transcriptRefs = readStringList(object, "transcript_refs");
        entry.changedFiles = readStringList(object, "changed_files");
        entry.reviewerNotes = readStringList(object, "reviewer_notes");
        entry.packetId = readString(object, "packet_id");
        entry.decision = readString(object, "decision");
        entry.proofStatus = readString(object, "proof_status");
        entry.tags = readStringList(object, "tags");
        entry.source = readString(object, "source");
        state.ledgerEntries.push_back(entry);
    }

    state.binder = DexBinder::parseBinderState(root);
    state.sessionCount = root.value("sessions").toArray().size();
    state.packetCount = root.value("packets").toArray().size();
    state.reviewActionCount = root.value("review_actions").toArray().size();
    state.commandPlanCount = root.value("command_plans").toArray().size();

    const QJsonArray candidates = root.value("bench_candidates").toArray();
    for (const QJsonValue &value : candidates) {
        const QJsonObject object = value.toObject();
        BenchCandidate candidate;
        candidate.id = readString(object, "candidate_id");
        candidate.name = readString(object, "name");
        candidate.repoPath = readString(object, "repo_path");
        candidate.classification = readString(object, "classification");
        candidate.status = readString(object, "status");
        candidate.contribution = readString(object, "contribution");
        candidate.risks = readStringList(object, "risks");
        candidate.nextAction = readString(object, "next_action");
        state.benchCandidates.push_back(candidate);
    }

    const QJsonObject context = root.value("context_panel").toObject();
    state.contextPanel.title = readString(context, "title");
    state.contextPanel.subtitle = readString(context, "subtitle");
    state.contextPanel.lines = readStringList(context, "lines");

    const QJsonObject flags = root.value("ui_flags").toObject();
    state.railVisible = readBool(flags, "rail_visible", true);
    state.contextVisible = readBool(flags, "context_visible", true);
    state.bottomShelfVisible = readBool(flags, "bottom_shelf_visible", false);

    if (state.workers.isEmpty()) {
        state.workers.push_back({"planner", "planner", "Planner", "ready"});
    }
    if (state.projects.isEmpty()) {
        state.projects.push_back({"dex_home", "dex_home", repoRoot_, "active", true, {"planner"}});
    }
    return state;
}

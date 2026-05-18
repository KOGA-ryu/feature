#include "binder_stats_state.h"

#include <QJsonArray>
#include <QJsonValue>

#include "binder_json_helpers.h"

namespace DexBinder {

StatsSnapshotView noDataStats(const QString &workerId) {
    StatsSnapshotView stats;
    stats.workerId = workerId;
    stats.workerRole = workerId;
    return stats;
}

StatsSnapshotView parseStatsSnapshot(const QJsonObject &root, const QString &selectedWorkerId) {
    const QJsonObject statsObject = root.value("stats_snapshot").toObject();
    if (statsObject.isEmpty()) {
        return noDataStats(selectedWorkerId);
    }

    StatsSnapshotView stats = noDataStats(selectedWorkerId);
    stats.present = true;
    stats.workerId = readString(statsObject, "worker_id", selectedWorkerId);
    stats.workerRole = readString(statsObject, "worker_role", stats.workerId);

    const QJsonObject generated = statsObject.value("generated_from").toObject();
    stats.generatedFrom.gradeRecords = readInt(generated, "grade_records");
    stats.generatedFrom.transcriptRecords = readInt(generated, "transcript_records");
    stats.generatedFrom.evidenceRecords = readInt(generated, "evidence_records");

    const QJsonObject summary = statsObject.value("routing_summary").toObject();
    stats.currentTrust = readString(summary, "current_trust", "unknown");
    stats.bestScope = readString(summary, "best_scope", "unknown");
    stats.weakScope = readString(summary, "weak_scope", "unknown");
    stats.sampleSize = readInt(summary, "sample_size");
    stats.confidence = readString(summary, "confidence", "no_data");

    for (const QJsonValue &value : statsObject.value("outcome_by_task_shape").toArray()) {
        const QJsonObject object = value.toObject();
        TaskShapeOutcomeView row;
        row.taskShape = readString(object, "task_shape", "unknown");
        row.runs = readInt(object, "runs");
        row.pass = readInt(object, "pass");
        row.partial = readInt(object, "partial");
        row.fail = readInt(object, "fail");
        row.recommendedRoute = readStringList(object, "recommended_route");
        row.confidence = readString(object, "confidence", "no_data");
        stats.outcomeByTaskShape.push_back(row);
    }

    for (const QJsonValue &value : statsObject.value("model_fit").toArray()) {
        const QJsonObject object = value.toObject();
        ModelFitSummaryView row;
        row.modelId = readString(object, "model_id", "unknown");
        row.bestTaskShapes = readStringList(object, "best_task_shapes");
        row.weakTaskShapes = readStringList(object, "weak_task_shapes");
        row.successConditions = readStringList(object, "success_conditions");
        row.failureConditions = readStringList(object, "failure_conditions");
        stats.modelFit.push_back(row);
    }

    for (const QJsonValue &value : statsObject.value("input_quality_impact").toArray()) {
        const QJsonObject object = value.toObject();
        InputQualityImpactView row;
        row.inputCondition = readString(object, "input_condition", "unknown");
        row.runs = readInt(object, "runs");
        row.observedOutcome = readString(object, "observed_outcome");
        row.observedFailurePattern = readString(object, "observed_failure_pattern");
        stats.inputQualityImpact.push_back(row);
    }

    const QJsonObject health = statsObject.value("evidence_health").toObject();
    stats.evidenceHealth.gradedRuns = readInt(health, "graded_runs");
    stats.evidenceHealth.runsWithProofRefs = readInt(health, "runs_with_proof_refs");
    stats.evidenceHealth.runsMissingProof = readInt(health, "runs_missing_proof");
    stats.evidenceHealth.runsWithScreenshots = readInt(health, "runs_with_screenshots");
    stats.evidenceHealth.runsWithCommandReceipts = readInt(health, "runs_with_command_receipts");
    stats.evidenceHealth.runsWithChangedFileRefs = readInt(health, "runs_with_changed_file_refs");
    stats.evidenceHealth.transcriptRefsAttached = readInt(health, "transcript_refs_attached");
    stats.evidenceHealth.completeEvidenceChains = readInt(health, "complete_evidence_chains");
    stats.evidenceHealth.dataConfidence = readString(health, "data_confidence", stats.confidence);

    const QJsonObject routing = statsObject.value("routing_recommendation").toObject();
    stats.routingRecommendation.defaultRoute = readString(routing, "default_route", "unknown");
    stats.routingRecommendation.requirePlannerWhen = readStringList(routing, "require_planner_when");
    stats.routingRecommendation.requireReviewerWhen = readStringList(routing, "require_reviewer_when");
    stats.routingRecommendation.blockBuilderWhen = readStringList(routing, "block_builder_when");
    return stats;
}

} // namespace DexBinder

#include "binder_record_state.h"

#include <QJsonArray>
#include <QJsonValue>

#include "binder_json_helpers.h"

namespace DexBinder {

GradeRecordView parseGradeRecord(const QJsonObject &object) {
    GradeRecordView record;
    record.recordId = readString(object, "record_id");
    record.timestamp = readString(object, "timestamp");
    record.workerId = readString(object, "worker_id");
    record.workerRole = readString(object, "worker_role");
    record.finalGrade = readString(object, "final_grade", "n/a");
    record.outcome = readString(object, "outcome", "unknown");
    record.gradeConfidence = readString(object, "grade_confidence", "unknown");
    const QJsonObject model = object.value("model_profile").toObject();
    record.modelId = firstText(model, {"model_id", "current_model"}, "unknown");
    const QJsonObject task = object.value("task_scope").toObject();
    record.taskShape = firstText(task, {"task_shape", "task_type"}, "unknown");
    const QJsonObject routing = object.value("routing_decision").toObject();
    record.routingVerdict = firstText(routing, {"routing_verdict", "default_route"}, "unknown");
    record.inputQualityLines = objectLines(object.value("input_quality").toObject());
    record.adjustedVerdictLines = objectLines(object.value("adjusted_verdict").toObject());
    record.modelProfileLines = objectLines(model);
    record.taskScopeLines = objectLines(task);
    record.scoreLines = objectLines(object.value("scores").toObject());
    record.evidenceLines = objectLines(object.value("evidence").toObject());
    record.learnedRuleLines = arrayObjectLines(object.value("learned_rules").toArray(), "rule");
    return record;
}

TranscriptRecordView parseTranscriptRecord(const QJsonObject &object) {
    TranscriptRecordView record;
    record.recordId = readString(object, "record_id");
    record.sessionId = readString(object, "session_id");
    record.workerId = readString(object, "worker_id");
    record.workerRole = readString(object, "worker_role");
    record.modelId = readString(object, "model_id", "unknown");
    record.sourceType = readString(object, "source_type", "unknown");
    record.recordStatus = readString(object, "record_status", "unknown");
    record.collectionWarnings = readStringList(object, "collection_warnings");
    record.missingData = readStringList(object, "missing_data");
    const QJsonObject time = object.value("time_metadata").toObject();
    record.localTimestamp = readString(time, "local_timestamp");
    record.timezone = readString(time, "timezone");
    record.timeMetadataLines = objectLines(time);
    record.environmentMetadataLines = objectLines(object.value("environment_metadata").toObject());
    for (const QJsonValue &value : object.value("segments").toArray()) {
        const QJsonObject segment = value.toObject();
        const QString line = QString("%1 | %2 | %3 | %4")
            .arg(readString(segment, "timestamp_local", "time missing"))
            .arg(readString(segment, "speaker", "speaker missing"))
            .arg(readString(segment, "segment_type", "type missing"))
            .arg(firstText(segment, {"summary", "source_ref", "raw_excerpt"}, "raw segment"));
        record.segmentLines.push_back(line);
        const QString excerpt = readString(segment, "raw_excerpt");
        if (!excerpt.isEmpty()) {
            record.rawExcerptLines.push_back(excerpt);
        }
    }
    record.toolReceiptLines = arrayObjectLines(object.value("tool_receipts").toArray(), "receipt");
    record.artifactRefLines = objectLines(object.value("artifact_refs").toObject());
    return record;
}

EvidenceRecordView parseEvidenceRecord(const QJsonObject &object) {
    EvidenceRecordView record;
    record.recordId = readString(object, "record_id");
    record.sessionId = readString(object, "session_id");
    record.workerId = readString(object, "worker_id");
    record.evidenceId = readString(object, "evidence_id");
    const QJsonArray proofRefs = object.value("proof_refs").toArray();
    record.proofCount = proofRefs.size();
    record.proofRefLines = arrayObjectLines(proofRefs, "proof");

    const QJsonObject failure = object.value("failure").toObject();
    record.failureId = readString(failure, "failure_id");
    record.failureType = readString(failure, "type", "unknown");
    record.severity = readString(failure, "severity", "unknown");
    record.failureLines = objectLines(failure);

    const QJsonObject correction = object.value("correction").toObject();
    record.correctionId = readString(correction, "correction_id");
    record.correctionStatus = readString(correction, "status", "pending");
    record.correctionLines = objectLines(correction);

    const QJsonObject acceptance = object.value("acceptance").toObject();
    if (acceptance.contains("accepted")) {
        record.acceptedState = readBool(acceptance, "accepted") ? "true" : "false";
    } else {
        record.acceptedState = readString(acceptance, "status", "pending");
    }
    record.acceptanceLines = objectLines(acceptance);
    return record;
}

} // namespace DexBinder

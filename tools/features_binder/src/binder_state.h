#pragma once

#include <QString>
#include <QStringList>
#include <QVector>

#include <algorithm>

#include "binder_profile_state.h"
#include "binder_record_state.h"
#include "binder_stats_state.h"

namespace DexBinder {

struct BinderState {
    QString selectedWorkerId = "planner";
    QString selectedTopTab = "Profile";
    StatsSnapshotView statsSnapshot;
    QVector<GradeRecordView> gradeRecords;
    QVector<TranscriptRecordView> transcriptRecords;
    QVector<EvidenceRecordView> evidenceRecords;
    ProfileView profileState;
    RelationshipPlaceholderView relationship;
};

inline QStringList binderTopTabs() {
    return {"Profile", "Stats", "Relationship", "Grade", "Transcript", "Evidence"};
}

inline bool recordMatchesWorker(const QString &recordWorkerId, const QString &recordWorkerRole, const QString &selectedWorkerId) {
    return recordWorkerId == selectedWorkerId || recordWorkerRole == selectedWorkerId;
}

inline BinderState parseBinderState(const QJsonObject &root) {
    BinderState binder;
    binder.selectedWorkerId = readString(root, "selected_worker_id", "planner");
    binder.statsSnapshot = parseStatsSnapshot(root, binder.selectedWorkerId);
    for (const QJsonValue &value : root.value("grade_records").toArray()) {
        binder.gradeRecords.push_back(parseGradeRecord(value.toObject()));
    }
    for (const QJsonValue &value : root.value("transcript_records").toArray()) {
        binder.transcriptRecords.push_back(parseTranscriptRecord(value.toObject()));
    }
    for (const QJsonValue &value : root.value("evidence_records").toArray()) {
        binder.evidenceRecords.push_back(parseEvidenceRecord(value.toObject()));
    }
    std::sort(binder.gradeRecords.begin(), binder.gradeRecords.end(), [](const GradeRecordView &left, const GradeRecordView &right) {
        return left.timestamp > right.timestamp;
    });
    std::sort(binder.transcriptRecords.begin(), binder.transcriptRecords.end(), [](const TranscriptRecordView &left, const TranscriptRecordView &right) {
        return left.localTimestamp > right.localTimestamp;
    });
    binder.profileState = parseProfileView(root);
    return binder;
}

inline QVector<GradeRecordView> gradeRecordsForWorker(const BinderState &binder, const QString &workerId) {
    QVector<GradeRecordView> records;
    for (const GradeRecordView &record : binder.gradeRecords) {
        if (recordMatchesWorker(record.workerId, record.workerRole, workerId)) {
            records.push_back(record);
        }
    }
    return records;
}

inline QVector<TranscriptRecordView> transcriptRecordsForWorker(const BinderState &binder, const QString &workerId) {
    QVector<TranscriptRecordView> records;
    for (const TranscriptRecordView &record : binder.transcriptRecords) {
        if (recordMatchesWorker(record.workerId, record.workerRole, workerId)) {
            records.push_back(record);
        }
    }
    return records;
}

inline QVector<EvidenceRecordView> evidenceRecordsForWorker(const BinderState &binder, const QString &workerId) {
    QVector<EvidenceRecordView> records;
    for (const EvidenceRecordView &record : binder.evidenceRecords) {
        if (record.workerId == workerId) {
            records.push_back(record);
        }
    }
    return records;
}

} // namespace DexBinder

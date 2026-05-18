#pragma once

#include <QJsonObject>
#include <QString>
#include <QStringList>

namespace DexBinder {

struct GradeRecordView {
    QString recordId;
    QString timestamp;
    QString workerId;
    QString workerRole;
    QString modelId;
    QString taskShape;
    QString finalGrade;
    QString outcome;
    QString gradeConfidence;
    QString routingVerdict;
    QStringList inputQualityLines;
    QStringList adjustedVerdictLines;
    QStringList modelProfileLines;
    QStringList taskScopeLines;
    QStringList scoreLines;
    QStringList evidenceLines;
    QStringList learnedRuleLines;
};

struct TranscriptRecordView {
    QString recordId;
    QString sessionId;
    QString localTimestamp;
    QString timezone;
    QString workerId;
    QString workerRole;
    QString modelId;
    QString sourceType;
    QString recordStatus;
    QStringList collectionWarnings;
    QStringList missingData;
    QStringList timeMetadataLines;
    QStringList environmentMetadataLines;
    QStringList segmentLines;
    QStringList rawExcerptLines;
    QStringList toolReceiptLines;
    QStringList artifactRefLines;
};

struct EvidenceRecordView {
    QString recordId;
    QString sessionId;
    QString workerId;
    QString evidenceId;
    QString failureId;
    QString failureType;
    QString severity;
    int proofCount = 0;
    QString correctionId;
    QString correctionStatus;
    QString acceptedState = "pending";
    QStringList proofRefLines;
    QStringList failureLines;
    QStringList correctionLines;
    QStringList acceptanceLines;
};

GradeRecordView parseGradeRecord(const QJsonObject &object);
TranscriptRecordView parseTranscriptRecord(const QJsonObject &object);
EvidenceRecordView parseEvidenceRecord(const QJsonObject &object);

} // namespace DexBinder

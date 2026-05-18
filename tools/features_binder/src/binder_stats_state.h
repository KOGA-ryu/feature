#pragma once

#include <QJsonObject>
#include <QString>
#include <QStringList>
#include <QVector>

namespace DexBinder {

struct StatsGeneratedFromView {
    int gradeRecords = 0;
    int transcriptRecords = 0;
    int evidenceRecords = 0;
};

struct TaskShapeOutcomeView {
    QString taskShape;
    int runs = 0;
    int pass = 0;
    int partial = 0;
    int fail = 0;
    QStringList recommendedRoute;
    QString confidence;
};

struct ModelFitSummaryView {
    QString modelId;
    QStringList bestTaskShapes;
    QStringList weakTaskShapes;
    QStringList successConditions;
    QStringList failureConditions;
};

struct InputQualityImpactView {
    QString inputCondition;
    int runs = 0;
    QString observedOutcome;
    QString observedFailurePattern;
};

struct EvidenceHealthView {
    int gradedRuns = 0;
    int runsWithProofRefs = 0;
    int runsMissingProof = 0;
    int runsWithScreenshots = 0;
    int runsWithCommandReceipts = 0;
    int runsWithChangedFileRefs = 0;
    int transcriptRefsAttached = 0;
    int completeEvidenceChains = 0;
    QString dataConfidence = "no_data";
};

struct RoutingRecommendationView {
    QString defaultRoute = "unknown";
    QStringList requirePlannerWhen;
    QStringList requireReviewerWhen;
    QStringList blockBuilderWhen;
};

struct StatsSnapshotView {
    bool present = false;
    QString workerId;
    QString workerRole;
    QString currentTrust = "unknown";
    QString bestScope = "unknown";
    QString weakScope = "unknown";
    int sampleSize = 0;
    QString confidence = "no_data";
    StatsGeneratedFromView generatedFrom;
    QVector<TaskShapeOutcomeView> outcomeByTaskShape;
    QVector<ModelFitSummaryView> modelFit;
    QVector<InputQualityImpactView> inputQualityImpact;
    EvidenceHealthView evidenceHealth;
    RoutingRecommendationView routingRecommendation;
};

StatsSnapshotView noDataStats(const QString &workerId);
StatsSnapshotView parseStatsSnapshot(const QJsonObject &root, const QString &selectedWorkerId);

} // namespace DexBinder

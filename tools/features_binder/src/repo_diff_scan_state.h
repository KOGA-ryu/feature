#pragma once

#include <QByteArray>
#include <QJsonDocument>
#include <QString>
#include <QStringList>
#include <QVector>

namespace DexRepoDiffScan {

struct ChangedFileRecord {
    QString path;
    QString boundary;
    QString changeType;
    QString riskLevel;
};

struct RepoDiffScanState {
    QString scanStatus = "unavailable";
    QString riskLevel = "unknown";
    QString recommendedNextStep = "unknown";
    QStringList warnings;
    QVector<ChangedFileRecord> changedFiles;
    QStringList protectedBoundaryPaths;
    QStringList generatedBoundaryPaths;
    QStringList unknownBoundaryPaths;
    QStringList testObligations;
    QStringList proofObligations;
    bool available = false;
};

RepoDiffScanState unavailableScanState(const QString &reason = QString("malformed scan data"));
RepoDiffScanState parseRepoDiffScanPayload(const QByteArray &payload);
RepoDiffScanState parseRepoDiffScanDocument(const QJsonDocument &document);

} // namespace DexRepoDiffScan

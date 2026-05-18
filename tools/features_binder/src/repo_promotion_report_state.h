#pragma once

#include <QByteArray>
#include <QJsonDocument>
#include <QString>
#include <QStringList>
#include <QVector>

namespace DexRepoPromotionReport {

struct PromotionComparedFile {
    QString source;
    QString destination;
    QString status = "unknown";
};

struct PromotionBlockedFile {
    QString requestedFile;
    QString reason = "unknown";
};

struct RepoPromotionReportState {
    QString createdAt = "unknown";
    QString mode = "unknown";
    bool dryRunOnly = true;
    QString workcopyRoot;
    QString authorityRoot;
    int comparedCount = 0;
    int blockedCount = 0;
    QVector<PromotionComparedFile> compared;
    QVector<PromotionBlockedFile> blocked;
    QStringList allowedTargets;
    QStringList blockedTargets;
    QString authorityWrites = "unknown";
    QString promotionRequires = "unknown";
    QStringList warnings;
    bool loaded = false;
};

RepoPromotionReportState unavailablePromotionReport(const QString &reason = QString("promotion report unavailable"));
RepoPromotionReportState parseRepoPromotionReportPayload(const QByteArray &payload);
RepoPromotionReportState parseRepoPromotionReportDocument(const QJsonDocument &document);
RepoPromotionReportState loadRepoPromotionReportFile(const QString &path);

} // namespace DexRepoPromotionReport

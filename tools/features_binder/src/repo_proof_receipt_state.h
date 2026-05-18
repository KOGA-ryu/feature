#pragma once

#include <QByteArray>
#include <QJsonDocument>
#include <QString>
#include <QStringList>
#include <QVector>

namespace DexRepoProofReceipt {

struct ProofScreenshotRecord {
    QString path;
    QString viewport;
    QString subject;
    QString status = "unknown";
};

struct CommandReceiptRecord {
    QString receiptId;
    QString command;
    QString status = "unknown";
    int exitCode = -1;
    QString logPath;
};

struct RepoProofReceiptState {
    QString proofType = "unknown";
    QString runId = "unknown";
    QString createdAt = "unknown";
    QString workcopyRoot;
    QString repoRoot;
    QString projectId = "unknown";
    QString tab = "unknown";
    QString detailLens = "unknown";
    QString receiptStatus = "unavailable";
    QString latestManifestPath;
    QString runManifestPath;
    QVector<ProofScreenshotRecord> screenshots;
    QVector<CommandReceiptRecord> commandReceipts;
    QStringList warnings;
    bool loaded = false;
};

RepoProofReceiptState unavailableProofReceipt(const QString &reason = QString("proof receipt unavailable"));
RepoProofReceiptState parseRepoProofReceiptPayload(const QByteArray &payload);
RepoProofReceiptState parseRepoProofReceiptDocument(const QJsonDocument &document);
RepoProofReceiptState loadRepoProofReceiptFile(const QString &path);

} // namespace DexRepoProofReceipt

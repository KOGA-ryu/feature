#pragma once

#include <QByteArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QString>
#include <QStringList>

#include "app_state.h"

class RustCockpitBackend final {
public:
    RustCockpitBackend(QString repoRoot, QString binaryPath);

    static QString defaultRepoRoot();
    static QString defaultBinaryPath(const QString &repoRoot);

    CockpitState deriveState() const;
    DexBinder::StatsSnapshotView deriveStatsForWorker(const QString &workerId, QString *error) const;
    DexRepoDiffScan::RepoDiffScanState runRepoDiffScan(
        const QString &projectId,
        const QString &projectRegistryPath,
        QString *error) const;
    DexRepoContractCheck::RepoContractCheckState runRepoContractCheck(
        const QString &projectId,
        const QString &projectRegistryPath,
        QString *error) const;
    bool appendLedgerRecord(const QJsonObject &record, QString *error) const;
    const QString &repoRoot() const;

private:
    QJsonDocument runJson(const QStringList &arguments, const QByteArray &input, QString *error) const;
    CockpitState parseState(const QJsonDocument &document) const;
    CockpitState fallbackState(const QString &error) const;

    QString repoRoot_;
    QString binaryPath_;
};

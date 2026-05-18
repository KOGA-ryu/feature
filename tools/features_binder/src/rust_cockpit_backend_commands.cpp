#include "rust_cockpit_backend.h"

#include <QJsonDocument>

#include "binder_state.h"
#include "repo_contract_check_state.h"
#include "repo_diff_scan_state.h"

CockpitState RustCockpitBackend::deriveState() const {
    QString error;
    const QJsonDocument document = runJson({"state", "derive", "--repo-root", repoRoot_}, QByteArray(), &error);
    if (!error.isEmpty()) {
        return fallbackState(error);
    }
    return parseState(document);
}

DexBinder::StatsSnapshotView RustCockpitBackend::deriveStatsForWorker(const QString &workerId, QString *error) const {
    const QJsonDocument document = runJson(
        {"stats", "derive", "--repo-root", repoRoot_, "--worker-id", workerId},
        QByteArray(),
        error);
    if (error && !error->isEmpty()) {
        return DexBinder::noDataStats(workerId);
    }
    QJsonObject wrapper;
    wrapper["stats_snapshot"] = document.object();
    return DexBinder::parseStatsSnapshot(wrapper, workerId);
}

DexRepoDiffScan::RepoDiffScanState RustCockpitBackend::runRepoDiffScan(
    const QString &projectId,
    const QString &projectRegistryPath,
    QString *error) const {
    if (projectId.isEmpty()) {
        if (error) {
            *error = "Unable to run repo diff-scan: missing project id.";
        }
        return DexRepoDiffScan::unavailableScanState("missing project id");
    }
    const QJsonDocument document = runJson(
        {"repo", "diff-scan", "--repo-root", repoRoot_, "--project-registry", projectRegistryPath, "--project-id", projectId},
        QByteArray(),
        error);
    if (error && !error->isEmpty()) {
        return DexRepoDiffScan::unavailableScanState(*error);
    }
    return DexRepoDiffScan::parseRepoDiffScanDocument(document);
}

DexRepoContractCheck::RepoContractCheckState RustCockpitBackend::runRepoContractCheck(
    const QString &projectId,
    const QString &projectRegistryPath,
    QString *error) const {
    if (projectId.isEmpty()) {
        if (error) {
            *error = "Unable to run repo contract-check: missing project id.";
        }
        return DexRepoContractCheck::unavailableContractState("missing project id");
    }
    const QJsonDocument document = runJson(
        {"repo", "contract-check", "--repo-root", repoRoot_, "--project-registry", projectRegistryPath, "--project-id", projectId},
        QByteArray(),
        error);
    if (error && !error->isEmpty()) {
        return DexRepoContractCheck::unavailableContractState(*error);
    }
    return DexRepoContractCheck::parseRepoContractCheckDocument(document);
}

bool RustCockpitBackend::appendLedgerRecord(const QJsonObject &record, QString *error) const {
    runJson({"ledger", "append", "--repo-root", repoRoot_}, QJsonDocument(record).toJson(QJsonDocument::Compact), error);
    return error->isEmpty();
}


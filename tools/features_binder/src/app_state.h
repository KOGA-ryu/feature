#pragma once

#include <QString>
#include <QVector>

#include "app_state_records.h"
#include "binder_state.h"
#include "project_registry.h"
#include "repo_binder_template.h"
#include "repo_contract_check_state.h"
#include "repo_diff_scan_state.h"
#include "repo_proof_receipt_state.h"
#include "repo_promotion_report_state.h"

struct CockpitState {
    int schemaVersion = 1;
    QString selectedProjectId;
    QString selectedWorkerId;
    QVector<ProjectSummary> projects;
    QVector<DexProjects::ProjectRegistryEntry> registryProjects;
    QVector<DexProjects::WorkerRegistryEntry> registryWorkers;
    QString projectRegistrySource;
    bool projectRegistryLoaded = false;
    QString projectRegistryError;
    DexRepoBinderTemplate::BinderTemplateStore binderTemplateStore;
    QVector<WorkerSummary> workers;
    QVector<LedgerEntry> ledgerEntries;
    DexBinder::BinderState binder;
    QVector<BenchCandidate> benchCandidates;
    int sessionCount = 0;
    int packetCount = 0;
    int reviewActionCount = 0;
    int commandPlanCount = 0;
    ContextPanelState contextPanel;
    bool railVisible = true;
    bool contextVisible = true;
    bool bottomShelfVisible = false;
    bool backendAvailable = true;
    QString backendError;
    DexRepoDiffScan::RepoDiffScanState repoDiffScan;
    DexRepoContractCheck::RepoContractCheckState repoContractCheck;
    DexRepoProofReceipt::RepoProofReceiptState repoProofReceipt;
    DexRepoPromotionReport::RepoPromotionReportState repoPromotionReport;
};

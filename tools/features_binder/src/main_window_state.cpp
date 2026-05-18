#include "main_window.h"

#include <QStatusBar>

#include <algorithm>
#include <utility>

#include "app_state_helpers.h"
#include "binder_state.h"
#include "detail_lens_rail.h"
#include "ledger_view.h"
#include "project_rail.h"
#include "repo_contract_check_state.h"
#include "repo_diff_scan_state.h"
#include "project_registry.h"
#include "right_context_panel.h"
#include "sheet_stack_body.h"

void DexHomeV2Window::reloadState() {
    state_ = backend_.deriveState();
    loadProjectRegistryIntoState();
    loadBinderTemplatesIntoState();
    loadProofReceiptIntoState();
    loadPromotionReportIntoState();
    if (selectedProjectId_.isEmpty()) {
        selectedProjectId_ = state_.selectedProjectId;
    }
    const QVector<DexProjects::ProjectRegistryEntry> projects = registryProjectsForState(state_);
    if (projects.isEmpty()) {
        selectedProjectId_.clear();
    } else if (!DexProjects::findProjectById(projects, selectedProjectId_)) {
        auto pinned = std::find_if(
            projects.begin(),
            projects.end(),
            [](const DexProjects::ProjectRegistryEntry &project) {
                return project.pinned;
            });
        if (pinned != projects.end()) {
            selectedProjectId_ = pinned->projectId;
        } else if (!projects.isEmpty()) {
            selectedProjectId_ = projects.first().projectId;
        }
    }
    state_.selectedProjectId = selectedProjectId_;
    if (!repoMode_ && (selectedWorkerId_.isEmpty() || !hasWorker(state_, selectedWorkerId_))) {
        selectedWorkerId_ = state_.selectedWorkerId;
    }
    if (!repoMode_ && !hasWorker(state_, selectedWorkerId_) && !state_.workers.isEmpty()) {
        selectedWorkerId_ = state_.workers.first().id;
    }
    if (repoMode_) {
        syncSelectedWorkerToSelectedProject();
    }
    refreshSelectedProjectScan();
    refreshSelectedWorkerStats();
    refreshViews();
    if (!state_.backendAvailable) {
        statusBar()->showMessage(state_.backendError);
    }
}

void DexHomeV2Window::refreshSelectedWorkerStats() {
    if (settingsMode_ || !state_.backendAvailable || selectedWorkerId_.isEmpty()) {
        return;
    }
    QString error;
    const DexBinder::StatsSnapshotView stats = backend_.deriveStatsForWorker(selectedWorkerId_, &error);
    if (!error.isEmpty()) {
        statusBar()->showMessage(error, 5000);
        return;
    }
    state_.binder.statsSnapshot = stats;
}

void DexHomeV2Window::refreshSelectedProjectScan() {
    if (settingsMode_) {
        state_.repoDiffScan = DexRepoDiffScan::unavailableScanState("settings mode");
        state_.repoContractCheck = DexRepoContractCheck::unavailableContractState("settings mode");
        return;
    }
    if (selectedProjectId_.isEmpty()) {
        state_.repoDiffScan = DexRepoDiffScan::unavailableScanState("no project selected");
        state_.repoContractCheck = DexRepoContractCheck::unavailableContractState("no project selected");
        return;
    }
    const QVector<DexProjects::ProjectRegistryEntry> projects = registryProjectsForState(state_);
    if (!DexProjects::findProjectById(projects, selectedProjectId_)) {
        state_.repoDiffScan = DexRepoDiffScan::unavailableScanState("selected project is not registered");
        state_.repoContractCheck = DexRepoContractCheck::unavailableContractState("selected project is not registered");
        return;
    }
    if (!state_.backendAvailable) {
        state_.repoDiffScan = DexRepoDiffScan::unavailableScanState("backend unavailable");
        state_.repoContractCheck = DexRepoContractCheck::unavailableContractState("backend unavailable");
        return;
    }
    QString error;
    state_.repoDiffScan = backend_.runRepoDiffScan(selectedProjectId_, projectRegistryPath_, &error);
    if (!error.isEmpty()) {
        statusBar()->showMessage(error, 5000);
    }
    QString contractError;
    state_.repoContractCheck = backend_.runRepoContractCheck(selectedProjectId_, projectRegistryPath_, &contractError);
    if (!contractError.isEmpty()) {
        statusBar()->showMessage(contractError, 5000);
    }
}

void DexHomeV2Window::syncSelectedWorkerToSelectedProject() {
    const QVector<WorkerSummary> workers = workersForProject(state_, selectedProjectId_);
    if (workers.isEmpty()) {
        selectedWorkerId_.clear();
        return;
    }
    const bool selectedWorkerIsAssigned = std::any_of(
        workers.begin(),
        workers.end(),
        [this](const WorkerSummary &worker) {
            return worker.id == selectedWorkerId_;
        });
    if (!selectedWorkerIsAssigned) {
        selectedWorkerId_ = workers.first().id;
    }
}

void DexHomeV2Window::refreshViews() {
    projectRail_->setState(state_, selectedWorkerId_, selectedProjectId_, repoMode_, settingsMode_);
    detailLensRail_->setVisible(!settingsMode_);
    if (settingsMode_) {
        ledger_->setSettingsState(
            state_,
            selectedProjectId_,
            selectedSettingsFeature_,
            [this](DexProjects::ProjectRegistry registry, QString projectId, DexRepoBinderTemplate::BinderTemplate binderTemplate) {
                saveProjectRegistryFromSettings(std::move(registry), projectId, std::move(binderTemplate), false);
            },
            [this]() {
                revertProjectRegistrySettings();
            },
            [this]() {
                exitSettingsMode();
            },
            [this](DexProjects::ProjectRegistry registry, QString projectId, DexRepoBinderTemplate::BinderTemplate binderTemplate) {
                saveProjectRegistryFromSettings(std::move(registry), projectId, std::move(binderTemplate), true);
            });
        rightContext_->setSettingsState(state_, selectedProjectId_, selectedSettingsFeature_);
    } else {
        detailLensRail_->setTopTab(selectedTopTab_, selectedDetailLens_, repoMode_);
        selectedDetailLens_ = detailLensRail_->currentLens();
        ledger_->setState(state_, selectedWorkerId_, selectedProjectId_, selectedTopTab_, selectedDetailLens_, repoMode_);
        rightContext_->setState(state_, selectedWorkerId_, selectedProjectId_, selectedTopTab_, selectedDetailLens_, repoMode_);
    }
    if (chromeLocationLabel_) {
        QString projectName = selectedProjectId_.isEmpty() ? QString("No project") : selectedProjectId_;
        if (const auto *project = DexProjects::findProjectById(registryProjectsForState(state_), selectedProjectId_)) {
            projectName = DexProjects::displayName(*project);
        }
        const QString modeLabel = settingsMode_
            ? QString("Settings")
            : repoMode_ && selectedProjectId_.isEmpty()
            ? QString("Repo Binder")
            : workerDisplayName(state_, selectedWorkerId_);
        if (settingsMode_) {
            chromeLocationLabel_->setText(QString("%1 / Settings / %2").arg(projectName, selectedSettingsFeature_));
        } else {
            chromeLocationLabel_->setText(QString("%1 / %2 / %3 / %4")
                .arg(projectName, modeLabel, selectedTopTab_, selectedDetailLens_));
        }
    }
    body_->relayoutSheets();
}

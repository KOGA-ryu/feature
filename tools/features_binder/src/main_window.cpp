#include "main_window.h"

#include <QSize>
#include <QStatusBar>

#include <utility>

#include "app_state_helpers.h"
#include "binder_navigation.h"
#include "repo_contract_check_state.h"
#include "repo_diff_scan_state.h"
#include "project_rail.h"
#include "right_context_panel.h"
#include "sheet_stack_body.h"
#include <QToolButton>
#include "ui_rules.h"

namespace {

constexpr int kDesignWidth = dex_ui::metrics::design_width;
constexpr int kDesignHeight = dex_ui::metrics::design_height;
constexpr int kMinWidth = dex_ui::metrics::min_width;
constexpr int kMinHeight = dex_ui::metrics::min_height;

} // namespace

DexHomeV2Window::DexHomeV2Window(QString repoRoot, QString binaryPath, QString projectRegistryPath, QString proofReceiptPath)
    : backend_(std::move(repoRoot), std::move(binaryPath)),
      projectRegistryPath_(resolveProjectRegistryPath(std::move(projectRegistryPath))),
      binderTemplateDirPath_(resolveBinderTemplateDirPath(projectRegistryPath_)),
      proofReceiptPath_(resolveProofReceiptPath(std::move(proofReceiptPath))),
      promotionReportPath_(resolvePromotionReportPath()) {
    textEditorWorkspace_ = new DexTextEditorWorkspace::TextEditorWorkspaceController(this);
    setWindowTitle("Dex Home v2");
    setMinimumSize(kMinWidth, kMinHeight);
    resize(kDesignWidth, kDesignHeight);
    setUnifiedTitleAndToolBarOnMac(true);

    buildToolbar();
    buildBody();
    reloadState();
}

void DexHomeV2Window::setProjectRailVisible(bool visible) {
    projectRail_->setVisible(visible);
    if (railToggle_) {
        railToggle_->setChecked(visible);
    }
    if (body_) {
        body_->relayoutSheets();
    }
}

void DexHomeV2Window::setRightContextVisible(bool visible) {
    rightContext_->setVisible(visible);
    if (contextToggle_) {
        contextToggle_->setChecked(visible);
    }
    if (body_) {
        body_->relayoutSheets();
    }
}

void DexHomeV2Window::setTopTab(const QString &tabName) {
    if (topTabsFor(repoMode_).contains(tabName)) {
        settingsMode_ = false;
        selectedTopTab_ = tabName;
        selectedDetailLens_ = detailLensTabsFor(selectedTopTab_, repoMode_).value(0, "Summary");
        setWorkspaceKind(repoMode_
            ? (selectedTopTab_ == "Text Editor" ? WorkspaceKind::TextEditor : WorkspaceKind::Repo)
            : WorkspaceKind::Agent);
        refreshViews();
    }
}

void DexHomeV2Window::setDetailLens(const QString &lensName) {
    const QStringList lenses = detailLensTabsFor(selectedTopTab_, repoMode_);
    if (lenses.contains(lensName)) {
        settingsMode_ = false;
        selectedDetailLens_ = lensName;
        refreshViews();
    }
}

void DexHomeV2Window::setRepoBinderMode(bool enabled) {
    if (repoMode_ == enabled) {
        return;
    }
    repoMode_ = enabled;
    settingsMode_ = false;
    workspaceKind_ = repoMode_ ? WorkspaceKind::Repo : WorkspaceKind::Agent;
    selectedTopTab_ = "Profile";
    selectedDetailLens_ = "Dashboard";
    if (repoModeToggle_) {
        repoModeToggle_->setChecked(repoMode_);
    }
    if (repoMode_) {
        syncSelectedWorkerToSelectedProject();
    }
    refreshViews();
}

void DexHomeV2Window::setSelectedWorker(const QString &workerId) {
    if (workerId.isEmpty()) {
        return;
    }
    settingsMode_ = false;
    workspaceKind_ = repoMode_ ? WorkspaceKind::Repo : WorkspaceKind::Agent;
    selectedWorkerId_ = workerId;
    if (!repoMode_ && !state_.workers.isEmpty() && !hasWorker(state_, selectedWorkerId_)) {
        selectedWorkerId_ = state_.workers.first().id;
    }
    if (repoMode_) {
        syncSelectedWorkerToSelectedProject();
    }
    refreshSelectedWorkerStats();
    refreshViews();
}

void DexHomeV2Window::setSelectedProject(const QString &projectId) {
    if (projectId.isEmpty()) {
        return;
    }
    settingsMode_ = false;
    workspaceKind_ = WorkspaceKind::Repo;
    selectedProjectId_ = projectId;
    state_.selectedProjectId = selectedProjectId_;
    state_.repoDiffScan = DexRepoDiffScan::unavailableScanState("scan pending");
    state_.repoContractCheck = DexRepoContractCheck::unavailableContractState("contract-check pending");
    if (repoMode_) {
        syncSelectedWorkerToSelectedProject();
    }
    refreshSelectedProjectScan();
    refreshViews();
}

void DexHomeV2Window::setSettingsMode(bool enabled) {
    if (!enabled) {
        exitSettingsMode();
        return;
    }
    settingsMode_ = enabled;
    if (settingsMode_) {
        workspaceKind_ = WorkspaceKind::Settings;
        repoMode_ = true;
        if (repoModeToggle_) {
            repoModeToggle_->setChecked(true);
        }
    }
    refreshViews();
}

void DexHomeV2Window::setSettingsFeature(const QString &featureName) {
    if (featureName == "Project Spec") {
        selectedSettingsFeature_ = featureName;
        if (settingsMode_) {
            refreshViews();
        }
    }
}

void DexHomeV2Window::openTextEditorWorkspace() {
    settingsMode_ = false;
    repoMode_ = true;
    workspaceKind_ = WorkspaceKind::TextEditor;
    selectedTopTab_ = "Text Editor";
    selectedDetailLens_ = detailLensTabsFor(selectedTopTab_, repoMode_).value(0, "Dashboard");
    if (repoModeToggle_) {
        repoModeToggle_->setChecked(true);
    }
    refreshViews();
    statusBar()->showMessage("Text Editor workspace opened from Shelf", 3000);
}

void DexHomeV2Window::openTextEditorCommandPalette() {
    openTextEditorWorkspace();
    textEditorWorkspace_->requestCommandPalette();
}

void DexHomeV2Window::runTextEditorCleanupPreviewProof() {
    openTextEditorWorkspace();
    textEditorWorkspace_->requestCleanupPreviewProof();
}

void DexHomeV2Window::setWorkspaceKind(WorkspaceKind workspaceKind) {
    workspaceKind_ = workspaceKind;
    settingsMode_ = workspaceKind_ == WorkspaceKind::Settings;
    repoMode_ = workspaceKind_ != WorkspaceKind::Agent;
    if (repoModeToggle_) {
        repoModeToggle_->setChecked(repoMode_);
    }
}

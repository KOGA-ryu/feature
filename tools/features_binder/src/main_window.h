#pragma once

#include <QMainWindow>
#include <QString>

#include "app_state.h"
#include "repo_binder_template.h"
#include "rust_cockpit_backend.h"
#include "text_editor_workspace_state.h"

class DetailLensRail;
class QLabel;
class LedgerView;
class ProjectRail;
class QToolBar;
class QToolButton;
class RightContextPanel;
class SheetStackBody;

class DexHomeV2Window final : public QMainWindow {
public:
    DexHomeV2Window(QString repoRoot, QString binaryPath, QString projectRegistryPath, QString proofReceiptPath);

    void setProjectRailVisible(bool visible);
    void setRightContextVisible(bool visible);
    void setTopTab(const QString &tabName);
    void setDetailLens(const QString &lensName);
    void setRepoBinderMode(bool enabled);
    void setSelectedWorker(const QString &workerId);
    void setSelectedProject(const QString &projectId);
    void setSettingsMode(bool enabled);
    void setSettingsFeature(const QString &featureName);
    void openTextEditorCommandPalette();
    void runTextEditorCleanupPreviewProof();

private:
    enum class WorkspaceKind {
        TextEditor,
        Agent,
        Repo,
        Settings,
    };

    static QString resolveProjectRegistryPath(const QString &requestedPath);
    static QString resolveProofReceiptPath(const QString &requestedPath);
    static QString resolveBinderTemplateDirPath(const QString &projectRegistryPath);
    static QString resolvePromotionReportPath();

    void buildToolbar();
    void buildBody();
    void reloadState();
    void loadProjectRegistryIntoState();
    void loadBinderTemplatesIntoState();
    void loadProofReceiptIntoState();
    void loadPromotionReportIntoState();
    void refreshSelectedWorkerStats();
    void refreshSelectedProjectScan();
    void refreshViews();
    void syncSelectedWorkerToSelectedProject();
    void openTextEditorWorkspace();
    void setWorkspaceKind(WorkspaceKind workspaceKind);
    void exitSettingsMode();
    void saveProjectRegistryFromSettings(
        DexProjects::ProjectRegistry registry,
        const QString &selectedProjectId,
        DexRepoBinderTemplate::BinderTemplate binderTemplate,
        bool exitAfterSave = false);
    void revertProjectRegistrySettings();
    void addGradeRecord();
    void addReviewVerdict(const QString &decision);
    void appendLedgerAction(
        const QString &kind,
        const QString &status,
        const QString &grade,
        const QString &decision,
        const QString &summary);

    RustCockpitBackend backend_;
    CockpitState state_;
    QString selectedWorkerId_;
    QString selectedProjectId_;
    QString projectRegistryPath_;
    QString binderTemplateDirPath_;
    QString proofReceiptPath_;
    QString promotionReportPath_;
    bool repoMode_ = true;
    bool settingsMode_ = false;
    WorkspaceKind workspaceKind_ = WorkspaceKind::Settings;
    QString selectedSettingsFeature_ = "Project Spec";
    QString selectedTopTab_ = "Profile";
    QString selectedDetailLens_ = "Dashboard";
    DexTextEditorWorkspace::TextEditorWorkspaceController *textEditorWorkspace_ = nullptr;
    SheetStackBody *body_ = nullptr;
    ProjectRail *projectRail_ = nullptr;
    LedgerView *ledger_ = nullptr;
    DetailLensRail *detailLensRail_ = nullptr;
    RightContextPanel *rightContext_ = nullptr;
    QLabel *chromeTitleLabel_ = nullptr;
    QLabel *chromeLocationLabel_ = nullptr;
    QToolButton *railToggle_ = nullptr;
    QToolButton *bottomToggle_ = nullptr;
    QToolButton *repoModeToggle_ = nullptr;
    QToolButton *contextToggle_ = nullptr;
};

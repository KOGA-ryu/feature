#include "project_rail.h"

#include <QHBoxLayout>
#include <QLabel>
#include <QMouseEvent>
#include <QScrollArea>
#include <QStyle>
#include <QVBoxLayout>
#include <QWidget>

#include "app_state_helpers.h"
#include "project_rail_rows.h"
#include "render_helpers.h"
#include "ui_rules.h"

namespace {

class SettingsRow final : public QFrame {
public:
    explicit SettingsRow(std::function<void()> onSelected, QWidget *parent = nullptr)
        : QFrame(parent), onSelected_(std::move(onSelected)) {
        setObjectName("settingsRow");
        setFixedHeight(32);
        setCursor(Qt::PointingHandCursor);
        auto *settingsLayout = new QHBoxLayout(this);
        settingsLayout->setContentsMargins(12, 4, 10, 4);
        settingsLayout->setSpacing(8);
        auto *icon = new QLabel;
        icon->setFixedSize(14, 14);
        icon->setStyleSheet("background:#fbfcfd; border:1px solid #8d969f; border-radius:3px;");
        settingsLayout->addWidget(icon);
        settingsLayout->addWidget(makeLabel("Settings"));
    }

protected:
    void mousePressEvent(QMouseEvent *event) override {
        if (event->button() == Qt::LeftButton && onSelected_) {
            onSelected_();
            event->accept();
            return;
        }
        QFrame::mousePressEvent(event);
    }

private:
    std::function<void()> onSelected_;
};

void polishWorkspaceWidget(QWidget *widget, const char *workspace) {
    if (!widget) {
        return;
    }
    widget->setProperty("workspace", workspace);
    widget->style()->unpolish(widget);
    widget->style()->polish(widget);
}

void setRailWorkspace(QFrame *rail, QWidget *railBody, QFrame *settingsRow, const char *workspace) {
    rail->setFixedWidth(QString::fromLatin1(workspace) == "text_editor"
            ? dex_ui::text_editor_metrics::rail_width
            : dex_ui::metrics::rail_width);
    rail->setProperty("workspace", workspace);
    rail->style()->unpolish(rail);
    rail->style()->polish(rail);
    polishWorkspaceWidget(railBody, workspace);
    polishWorkspaceWidget(settingsRow, workspace);
}

QFrame *makeTextEditorBucket(const QString &title, const QString &note, const QString &uiPath) {
    auto *bucket = new QFrame;
    bucket->setObjectName("textEditorRailBucket");
    bucket->setProperty("uiPath", uiPath);
    bucket->setMinimumHeight(dex_ui::text_editor_metrics::rail_bucket_height);

    auto *layout = new QVBoxLayout(bucket);
    layout->setContentsMargins(
        dex_ui::text_editor_metrics::panel_padding,
        dex_ui::text_editor_metrics::panel_padding_dense,
        dex_ui::text_editor_metrics::panel_padding,
        dex_ui::text_editor_metrics::panel_padding_dense);
    layout->setSpacing(dex_ui::text_editor_metrics::dense_gap);

    auto *titleLabel = makeLabel(title, "textEditorBucketTitle");
    titleLabel->setProperty("uiPath", uiPath + ".title");
    layout->addWidget(titleLabel);

    auto *noteLabel = makeLabel(note, "textEditorBucketNote");
    noteLabel->setProperty("uiPath", uiPath + ".empty_state");
    layout->addWidget(noteLabel);

    return bucket;
}

} // namespace

ProjectRail::ProjectRail(
    std::function<void(QString)> onWorkerSelected,
    std::function<void(QString)> onProjectSelected,
    std::function<void()> onSettingsSelected,
    QWidget *parent)
    : QFrame(parent),
      onWorkerSelected_(std::move(onWorkerSelected)),
      onProjectSelected_(std::move(onProjectSelected)),
      onSettingsSelected_(std::move(onSettingsSelected)) {
    setObjectName("projectRail");
    setFixedWidth(dex_ui::metrics::rail_width);

    auto *outer = new QVBoxLayout(this);
    outer->setContentsMargins(dex_ui::metrics::rail_inset, 12, dex_ui::metrics::rail_inset, 10);
    outer->setSpacing(8);

    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);

    railBody_ = new QWidget;
    railBody_->setObjectName("projectRailBody");
    listLayout_ = new QVBoxLayout(railBody_);
    listLayout_->setContentsMargins(0, 0, 0, 0);
    listLayout_->setSpacing(8);
    scroll->setWidget(railBody_);
    outer->addWidget(scroll, 1);

    settingsRow_ = new SettingsRow(onSettingsSelected_, this);
    outer->addWidget(settingsRow_);
}

void ProjectRail::setState(
    const CockpitState &state,
    const QString &selectedWorkerId,
    const QString &selectedProjectId,
    bool repoMode,
    bool settingsMode) {
    if (settingsMode) {
        setSettingsState(state, selectedProjectId);
        return;
    }
    if (repoMode) {
        setRepoState(state, selectedWorkerId, selectedProjectId);
        return;
    }
    setAgentState(state, selectedWorkerId);
}

void ProjectRail::setRepoState(
    const CockpitState &state,
    const QString &selectedWorkerId,
    const QString &selectedProjectId) {
    setRailWorkspace(this, railBody_, settingsRow_, "repo");
    clearLayout(listLayout_);
    settingsRow_->setProperty("active", false);
    settingsRow_->style()->unpolish(settingsRow_);
    settingsRow_->style()->polish(settingsRow_);

    const QVector<DexProjects::ProjectRegistryEntry> projects = registryProjectsForState(state);

    if (projects.isEmpty()) {
        listLayout_->addWidget(makeLabel("Projects", "sectionLabel"));
        auto *empty = makeLabel("No projects registered", "mutedLabel");
        empty->setFixedHeight(24);
        listLayout_->addWidget(empty);
        listLayout_->addStretch(1);
        return;
    }

    listLayout_->addWidget(makeLabel("Pinned", "sectionLabel"));
    addProjectRows(state, selectedProjectId, true);
    const auto *selectedProject = DexProjects::findProjectById(projects, selectedProjectId);
    if (selectedProject) {
        listLayout_->addWidget(makeLabel("Workers", "sectionLabel"));
        addWorkerRows(state, selectedWorkerId, selectedProjectId, true);
    }

    const bool hasNonPinnedProject = std::any_of(
        projects.begin(),
        projects.end(),
        [](const DexProjects::ProjectRegistryEntry &project) {
            return !project.pinned;
        });
    if (hasNonPinnedProject) {
        listLayout_->addWidget(makeLabel("All Projects", "sectionLabel"));
        addProjectRows(state, selectedProjectId, false);
    }
    listLayout_->addStretch(1);
}

void ProjectRail::setAgentState(const CockpitState &state, const QString &selectedWorkerId) {
    setRailWorkspace(this, railBody_, settingsRow_, "agent");
    clearLayout(listLayout_);
    settingsRow_->setProperty("active", false);
    settingsRow_->style()->unpolish(settingsRow_);
    settingsRow_->style()->polish(settingsRow_);

    listLayout_->addWidget(makeLabel("Agents", "sectionLabel"));
    if (state.workers.isEmpty()) {
        auto *empty = makeLabel("No agents available", "mutedLabel");
        empty->setFixedHeight(22);
        listLayout_->addWidget(empty);
    }
    for (const WorkerSummary &worker : state.workers) {
        listLayout_->addWidget(DexProjectRailRows::makeWorkerRow(
            worker.id,
            worker.role,
            worker.displayName,
            worker.status,
            worker.id == selectedWorkerId,
            onWorkerSelected_));
    }
    listLayout_->addStretch(1);
}

void ProjectRail::setTextEditorState() {
    setRailWorkspace(this, railBody_, settingsRow_, "text_editor");
    clearLayout(listLayout_);
    settingsRow_->setProperty("active", false);
    settingsRow_->style()->unpolish(settingsRow_);
    settingsRow_->style()->polish(settingsRow_);

    auto *title = makeLabel("Text Editor", "sectionLabel");
    title->setProperty("uiPath", "workbench.rail.text_editor.title");
    listLayout_->addWidget(title);
    auto *empty = makeLabel("Blank workspace", "mutedLabel");
    empty->setProperty("uiPath", "workbench.rail.text_editor.empty_state");
    listLayout_->addWidget(empty);
    listLayout_->addWidget(makeTextEditorBucket("Documents", "No document open", "workbench.rail.text_editor.documents"));
    listLayout_->addWidget(makeTextEditorBucket("Clipboard", "No clipboard capture", "workbench.rail.text_editor.clipboard"));
    listLayout_->addWidget(makeTextEditorBucket("Drafts", "No drafts saved", "workbench.rail.text_editor.drafts"));
    listLayout_->addWidget(makeTextEditorBucket("Fixtures", "No fixture selected", "workbench.rail.text_editor.fixtures"));
    listLayout_->addStretch(1);
}

void ProjectRail::setSettingsState(const CockpitState &state, const QString &selectedProjectId) {
    setRailWorkspace(this, railBody_, settingsRow_, "settings");
    clearLayout(listLayout_);
    settingsRow_->setProperty("active", true);
    settingsRow_->style()->unpolish(settingsRow_);
    settingsRow_->style()->polish(settingsRow_);

    listLayout_->addWidget(makeLabel("Settings", "sectionLabel"));
    listLayout_->addWidget(makeLabel("Project Spec", "mutedLabel"));
    listLayout_->addWidget(makeLabel("Registry", "sectionLabel"));
    listLayout_->addWidget(makeLabel(state.projectRegistryLoaded ? "projects.json loaded" : "registry unavailable", "mutedLabel"));
    listLayout_->addWidget(makeLabel("Selected", "sectionLabel"));
    const auto *project = DexProjects::findProjectById(registryProjectsForState(state), selectedProjectId);
    listLayout_->addWidget(makeLabel(project ? DexProjects::displayName(*project) : QString("new project / none"), "mutedLabel"));
    listLayout_->addStretch(1);
}

void ProjectRail::addProjectRows(const CockpitState &state, const QString &selectedProjectId, bool pinned) {
    bool added = false;
    const QVector<DexProjects::ProjectRegistryEntry> projects = registryProjectsForState(state);
    for (const DexProjects::ProjectRegistryEntry &project : projects) {
        if (project.pinned == pinned) {
            added = true;
            listLayout_->addWidget(DexProjectRailRows::makeProjectRow(
                project.projectId,
                DexProjects::displayName(project),
                project.path,
                project.status,
                project.projectId == selectedProjectId,
                onProjectSelected_));
        }
    }
    if (!added) {
        auto *empty = makeLabel(pinned ? "No pinned projects" : "No projects", "mutedLabel");
        empty->setFixedHeight(22);
        listLayout_->addWidget(empty);
    }
}

void ProjectRail::addWorkerRows(const CockpitState &state, const QString &selectedWorkerId, const QString &selectedProjectId, bool repoMode) {
    auto *workerContainer = new QWidget;
    auto *workerLayout = new QVBoxLayout(workerContainer);
    workerLayout->setContentsMargins(12, 2, 0, 12);
    workerLayout->setSpacing(6);
    const QVector<WorkerSummary> workers = state.projectRegistryLoaded
        ? workersForProject(state, selectedProjectId)
        : (repoMode ? workersForProject(state, selectedProjectId) : state.workers);
    if (workers.isEmpty()) {
        auto *empty = makeLabel("No workers assigned", "mutedLabel");
        empty->setFixedHeight(22);
        workerLayout->addWidget(empty);
    }
    for (const WorkerSummary &worker : workers) {
        workerLayout->addWidget(DexProjectRailRows::makeWorkerRow(
            worker.id,
            worker.role,
            worker.displayName,
            worker.status,
            worker.id == selectedWorkerId,
            onWorkerSelected_));
    }
    listLayout_->addWidget(workerContainer);
}

#include "right_context_panel.h"

#include <QLabel>
#include <QPushButton>
#include <QScrollArea>
#include <QStyle>
#include <QVBoxLayout>
#include <QWidget>

#include <utility>

#include "agent_context_renderer.h"
#include "app_state_helpers.h"
#include "project_registry.h"
#include "render_helpers.h"
#include "repo_binder_template.h"
#include "repo_context_renderer.h"
#include "right_context_render_helpers.h"
#include "text_editor_workspace_blank.h"
#include "ui_rules.h"

namespace {

void setContextWorkspace(QFrame *context, const char *workspace) {
    context->setProperty("workspace", workspace);
    context->style()->unpolish(context);
    context->style()->polish(context);
}

} // namespace

RightContextPanel::RightContextPanel(
    std::function<void()> onAddGrade,
    std::function<void(QString)> onReviewVerdict,
    QWidget *parent)
    : QFrame(parent),
      onAddGrade_(std::move(onAddGrade)),
      onReviewVerdict_(std::move(onReviewVerdict)) {
    setObjectName("rightContext");
    setFixedWidth(dex_ui::metrics::right_context_width);

    auto *outer = new QVBoxLayout(this);
    outer->setContentsMargins(10, 12, 10, 10);
    outer->setSpacing(10);
    content_ = new QWidget;
    contentLayout_ = new QVBoxLayout(content_);
    contentLayout_->setContentsMargins(0, 0, 0, 0);
    contentLayout_->setSpacing(9);

    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    scroll->setWidget(content_);
    outer->addWidget(scroll, 1);
}

void RightContextPanel::setState(
    const CockpitState &state,
    const QString &selectedWorkerId,
    const QString &selectedProjectId,
    const QString &selectedTopTab,
    const QString &selectedDetailLens,
    bool repoMode) {
    if (repoMode) {
        setRepoState(state, selectedWorkerId, selectedProjectId, selectedTopTab, selectedDetailLens);
        return;
    }
    setAgentState(state, selectedWorkerId, selectedTopTab, selectedDetailLens);
}

void RightContextPanel::setRepoState(
    const CockpitState &state,
    const QString &selectedWorkerId,
    const QString &selectedProjectId,
    const QString &selectedTopTab,
    const QString &selectedDetailLens) {
    setContextWorkspace(this, "repo");
    clearLayout(contentLayout_);
    const QVector<DexProjects::ProjectRegistryEntry> projects = registryProjectsForState(state);
    const DexProjects::ProjectRegistryEntry *selected = DexProjects::findProjectById(projects, selectedProjectId);
    if (!selected) {
        DexRightContext::addContextSection(contentLayout_, "selected repo");
        DexRightContext::addWrappedLine(contentLayout_, "No project selected");
        DexRightContext::addWrappedLine(contentLayout_, "active subject                 " + selectedTopTab);
        DexRightContext::addWrappedLine(contentLayout_, "detail lens                    " + selectedDetailLens);
        DexRightContext::addWrappedLine(contentLayout_, "registered projects            0");
        DexRightContext::addWrappedLine(contentLayout_, "diff-scan                      not run");
        DexRightContext::addWrappedLine(contentLayout_, "contract-check                 not run");
        DexRightContext::addContextSection(contentLayout_, "next registration step");
        DexRightContext::addWrappedLine(contentLayout_, "Open Settings to create the first project spec.");
        contentLayout_->addStretch(1);
        return;
    }
    const DexProjects::ProjectRegistryEntry project = *selected;
    const DexRepoBinderTemplate::BinderTemplate binderTemplate =
        DexRepoBinderTemplate::resolveTemplateForProject(state.binderTemplateStore, project.binderTemplate);
    DexRightContext::addRepoContext(
        contentLayout_,
        state,
        selectedWorkerId,
        project,
        binderTemplate,
        state.repoDiffScan,
        state.repoContractCheck,
        state.repoProofReceipt,
        state.repoPromotionReport,
        selectedTopTab,
        selectedDetailLens);
    contentLayout_->addStretch(1);
}

void RightContextPanel::setAgentState(
    const CockpitState &state,
    const QString &selectedWorkerId,
    const QString &selectedTopTab,
    const QString &selectedDetailLens) {
    setContextWorkspace(this, "agent");
    clearLayout(contentLayout_);
    DexRightContext::addAgentContext(contentLayout_, state, selectedWorkerId, selectedTopTab, selectedDetailLens);
    contentLayout_->addStretch(1);
}

void RightContextPanel::setTextEditorState(const QString &selectedDetailLens) {
    setContextWorkspace(this, "text_editor");
    clearLayout(contentLayout_);
    DexTextEditorWorkspace::addBlankWorkspaceContext(contentLayout_, selectedDetailLens);
    contentLayout_->addStretch(1);
}

void RightContextPanel::setSettingsState(
    const CockpitState &state,
    const QString &selectedProjectId,
    const QString &selectedSettingsFeature) {
    setContextWorkspace(this, "settings");
    clearLayout(contentLayout_);

    DexRightContext::addContextSection(contentLayout_, "settings");
    DexRightContext::addWrappedLine(contentLayout_, "screen                         " + selectedSettingsFeature);
    DexRightContext::addWrappedLine(contentLayout_, "save behavior                  explicit binder spec save only");
    DexRightContext::addWrappedLine(contentLayout_, "registry                       " + (state.projectRegistrySource.isEmpty() ? QString("unknown") : state.projectRegistrySource));
    DexRightContext::addWrappedLine(contentLayout_, "loaded                         " + QString(state.projectRegistryLoaded ? "yes" : "no"));
    if (!state.projectRegistryError.isEmpty()) {
        DexRightContext::addWrappedLine(contentLayout_, "registry warning               " + state.projectRegistryError);
    }

    DexRightContext::addContextSection(contentLayout_, "selected project");
    const QVector<DexProjects::ProjectRegistryEntry> projects = registryProjectsForState(state);
    if (const auto *project = DexProjects::findProjectById(projects, selectedProjectId)) {
        DexRightContext::addWrappedLine(contentLayout_, "project                        " + DexProjects::displayName(*project));
        DexRightContext::addWrappedLine(contentLayout_, "project id                     " + project->projectId);
        DexRightContext::addWrappedLine(contentLayout_, "path                           " + project->path);
        DexRightContext::addWrappedLine(contentLayout_, "assigned workers               " + QString::number(project->workerIds.size()));
    } else {
        DexRightContext::addWrappedLine(contentLayout_, "project                        new project / none selected");
    }

    DexRightContext::addContextSection(contentLayout_, "boundaries");
    DexRightContext::addWrappedLine(contentLayout_, "writes                         projects.json and selected template JSON");
    DexRightContext::addWrappedLine(contentLayout_, "jsonl                          not touched");
    DexRightContext::addWrappedLine(contentLayout_, "backend scanner                not run by settings");
    DexRightContext::addWrappedLine(contentLayout_, "worker execution               not allowed");
    DexRightContext::addContextSection(contentLayout_, "navigation");
    DexRightContext::addWrappedLine(contentLayout_, "Back                           returns to repo binder without saving");
    DexRightContext::addWrappedLine(contentLayout_, "Save and Back                  writes binder spec, then returns");
    contentLayout_->addStretch(1);
}

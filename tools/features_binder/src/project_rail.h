#pragma once

#include <QFrame>

#include <functional>

#include "app_state.h"

class QVBoxLayout;
class QWidget;

class ProjectRail final : public QFrame {
public:
    ProjectRail(
        std::function<void(QString)> onWorkerSelected,
        std::function<void(QString)> onProjectSelected,
        std::function<void()> onSettingsSelected,
        QWidget *parent = nullptr);

    void setRepoState(
        const CockpitState &state,
        const QString &selectedWorkerId,
        const QString &selectedProjectId);
    void setAgentState(const CockpitState &state, const QString &selectedWorkerId);
    void setTextEditorState();
    void setSettingsState(const CockpitState &state, const QString &selectedProjectId);
    void setState(
        const CockpitState &state,
        const QString &selectedWorkerId,
        const QString &selectedProjectId,
        bool repoMode,
        bool settingsMode);

private:
    void addProjectRows(const CockpitState &state, const QString &selectedProjectId, bool pinned);
    void addWorkerRows(const CockpitState &state, const QString &selectedWorkerId, const QString &selectedProjectId, bool repoMode);

    QVBoxLayout *listLayout_ = nullptr;
    QWidget *railBody_ = nullptr;
    std::function<void(QString)> onWorkerSelected_;
    std::function<void(QString)> onProjectSelected_;
    std::function<void()> onSettingsSelected_;
    QFrame *settingsRow_ = nullptr;
};

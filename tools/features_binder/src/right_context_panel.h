#pragma once

#include <QFrame>
#include <QString>

#include <functional>

#include "app_state.h"

class QVBoxLayout;
class QWidget;

class RightContextPanel final : public QFrame {
public:
    RightContextPanel(
        std::function<void()> onAddGrade,
        std::function<void(QString)> onReviewVerdict,
        QWidget *parent = nullptr);

    void setState(
        const CockpitState &state,
        const QString &selectedWorkerId,
        const QString &selectedProjectId,
        const QString &selectedTopTab,
        const QString &selectedDetailLens,
        bool repoMode);
    void setRepoState(
        const CockpitState &state,
        const QString &selectedWorkerId,
        const QString &selectedProjectId,
        const QString &selectedTopTab,
        const QString &selectedDetailLens);
    void setAgentState(
        const CockpitState &state,
        const QString &selectedWorkerId,
        const QString &selectedTopTab,
        const QString &selectedDetailLens);
    void setTextEditorState(const QString &selectedDetailLens);
    void setSettingsState(
        const CockpitState &state,
        const QString &selectedProjectId,
        const QString &selectedSettingsFeature);

private:
    QWidget *content_ = nullptr;
    QVBoxLayout *contentLayout_ = nullptr;
    std::function<void()> onAddGrade_;
    std::function<void(QString)> onReviewVerdict_;
};

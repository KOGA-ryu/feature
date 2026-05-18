#pragma once

#include <QButtonGroup>
#include <QHBoxLayout>
#include <QPushButton>
#include <QStackedWidget>
#include <QString>
#include <QStringList>
#include <QVBoxLayout>
#include <QWidget>

#include <algorithm>
#include <functional>
#include <utility>

#include "agent_binder_pages.h"
#include "app_state.h"
#include "app_state_helpers.h"
#include "binder_navigation.h"
#include "render_helpers.h"
#include "repo_binder_template.h"
#include "repo_binder_pages.h"
#include "project_registry_spec_page.h"
#include "text_editor_workbench_page.h"
#include "ui_rules.h"

class LedgerView final : public QWidget {
public:
    explicit LedgerView(std::function<void(QString)> onTabSelected, QWidget *parent = nullptr)
        : QWidget(parent), onTabSelected_(std::move(onTabSelected)) {
        setObjectName("centerLedger");
        auto *outer = new QVBoxLayout(this);
        outer->setContentsMargins(dex_ui::metrics::sheet_overlap, 0, dex_ui::metrics::sheet_overlap, 0);
        outer->setSpacing(0);

        auto *tabRow = new QWidget;
        tabLayout_ = new QHBoxLayout(tabRow);
        tabLayout_->setContentsMargins(0, 0, 0, 0);
        tabLayout_->setSpacing(0);

        pages_ = new QStackedWidget;
        tabGroup_ = new QButtonGroup(this);
        tabGroup_->setExclusive(true);

        connect(tabGroup_, &QButtonGroup::idClicked, this, [this](int index) {
            pages_->setCurrentIndex(index);
            currentTab_ = currentTabs_.value(index, "Profile");
            onTabSelected_(currentTab_);
        });

        outer->addWidget(tabRow);
        outer->addWidget(pages_, 1);
    }

    void setState(
        const CockpitState &state,
        const QString &workerId,
        const QString &selectedProjectId,
        const QString &topTab,
        const QString &detailLens,
        bool repoMode) {
        const QStringList tabs = topTabsFor(repoMode);
        if (currentTabs_ != tabs) {
            rebuildTabs(tabs);
        }
        const int requestedIndex = std::max(0, static_cast<int>(tabs.indexOf(topTab)));
        const int previousIndex = pages_->count() == 0 ? requestedIndex : std::max(0, pages_->currentIndex());
        while (pages_->count() > 0) {
            QWidget *page = pages_->widget(0);
            pages_->removeWidget(page);
            page->deleteLater();
        }

        if (repoMode) {
            const QVector<DexProjects::ProjectRegistryEntry> projects = registryProjectsForState(state);
            const DexProjects::ProjectRegistryEntry *selected = DexProjects::findProjectById(projects, selectedProjectId);
            if (!selected) {
                for (const QString &tab : tabs) {
                    pages_->addWidget(DexBinderPages::buildRepoBlankPage(state, tab, detailLens));
                }
            } else {
                const DexProjects::ProjectRegistryEntry project = *selected;
                const DexRepoBinderTemplate::BinderTemplate binderTemplate =
                    DexRepoBinderTemplate::resolveTemplateForProject(state.binderTemplateStore, project.binderTemplate);
                pages_->addWidget(DexBinderPages::buildRepoProfilePage(state, workerId, project, detailLens, binderTemplate));
                pages_->addWidget(DexBinderPages::buildRepoInventoryPage(state, workerId, project, detailLens, binderTemplate));
                pages_->addWidget(DexBinderPages::buildRepoMapPage(state, workerId, project, detailLens));
                pages_->addWidget(DexBinderPages::buildRepoAuthorityPage(state, workerId, project, detailLens));
                pages_->addWidget(DexBinderPages::buildRepoContractsPage(state, workerId, project, detailLens));
                pages_->addWidget(DexTextEditorPages::buildTextEditorWorkbenchPage());
                pages_->addWidget(DexBinderPages::buildRepoActivityPage(state, workerId, project, detailLens));
                pages_->addWidget(DexBinderPages::buildRepoQualityPage(state, workerId, project, detailLens, binderTemplate));
                pages_->addWidget(DexBinderPages::buildRepoEvidencePage(state, workerId, project, detailLens));
            }
        } else {
            pages_->addWidget(DexBinderPages::buildProfilePage(state, workerId, detailLens));
            pages_->addWidget(DexBinderPages::buildStatsPage(state, workerId, detailLens));
            pages_->addWidget(DexBinderPages::buildRelationshipPage(state, workerId, detailLens));
            pages_->addWidget(DexBinderPages::buildGradePage(state, workerId, detailLens));
            pages_->addWidget(DexBinderPages::buildTranscriptPage(state, workerId, detailLens));
            pages_->addWidget(DexBinderPages::buildEvidencePage(state, workerId, detailLens));
        }
        const int nextIndex = requestedIndex >= 0
            ? requestedIndex
            : std::min(previousIndex, pages_->count() - 1);
        pages_->setCurrentIndex(std::clamp(nextIndex, 0, pages_->count() - 1));
        currentTab_ = tabs.value(pages_->currentIndex(), "Profile");
        if (auto *button = tabGroup_->button(pages_->currentIndex())) {
            button->setChecked(true);
        }
    }

    void setTextEditorWorkspaceState() {
        clearLayout(tabLayout_);
        currentTabs_.clear();
        delete tabGroup_;
        tabGroup_ = new QButtonGroup(this);
        tabGroup_->setExclusive(true);
        while (pages_->count() > 0) {
            QWidget *page = pages_->widget(0);
            pages_->removeWidget(page);
            page->deleteLater();
        }
        pages_->addWidget(DexTextEditorPages::buildTextEditorWorkbenchPage());
        pages_->setCurrentIndex(0);
        currentTab_ = "Text Editor";
    }

    void setSettingsState(
        const CockpitState &state,
        const QString &selectedProjectId,
        const QString &selectedSettingsFeature,
        std::function<void(DexProjects::ProjectRegistry, QString, DexRepoBinderTemplate::BinderTemplate)> onSave,
        std::function<void()> onRevert,
        std::function<void()> onBack,
        std::function<void(DexProjects::ProjectRegistry, QString, DexRepoBinderTemplate::BinderTemplate)> onSaveAndBack) {
        clearLayout(tabLayout_);
        currentTabs_.clear();
        delete tabGroup_;
        tabGroup_ = new QButtonGroup(this);
        tabGroup_->setExclusive(true);
        while (pages_->count() > 0) {
            QWidget *page = pages_->widget(0);
            pages_->removeWidget(page);
            page->deleteLater();
        }

        DexProjects::ProjectRegistry registry;
        registry.loaded = state.projectRegistryLoaded;
        registry.sourcePath = state.projectRegistrySource;
        registry.error = state.projectRegistryError;
        registry.projects = state.registryProjects;
        registry.workers = state.registryWorkers;
        currentTabs_ = {"Project Spec"};
        const int requestedIndex = std::max(0, static_cast<int>(currentTabs_.indexOf(selectedSettingsFeature)));
        for (int i = 0; i < currentTabs_.size(); ++i) {
            auto *button = new QPushButton(currentTabs_.at(i));
            button->setObjectName("ledgerTab");
            button->setProperty("tabName", currentTabs_.at(i));
            button->setProperty("tabIndex", i);
            button->setProperty("firstTab", i == 0);
            button->setProperty("lastTab", i == currentTabs_.size() - 1);
            button->setCheckable(true);
            if (i == requestedIndex) {
                button->setChecked(true);
            }
            tabGroup_->addButton(button, i);
            tabLayout_->addWidget(button);
        }
        tabLayout_->addStretch(1);
        connect(tabGroup_, &QButtonGroup::idClicked, this, [this](int index) {
            pages_->setCurrentIndex(index);
            currentTab_ = currentTabs_.value(index, "Project Spec");
        });
        pages_->addWidget(new ProjectRegistrySpecPage(
            registry,
            selectedProjectId,
            std::move(onSave),
            std::move(onRevert),
            std::move(onBack),
            std::move(onSaveAndBack)));
        pages_->setCurrentIndex(requestedIndex);
        currentTab_ = currentTabs_.value(requestedIndex, "Project Spec");
    }

private:
    void rebuildTabs(const QStringList &tabs) {
        clearLayout(tabLayout_);
        delete tabGroup_;
        tabGroup_ = new QButtonGroup(this);
        tabGroup_->setExclusive(true);
        connect(tabGroup_, &QButtonGroup::idClicked, this, [this](int index) {
            pages_->setCurrentIndex(index);
            currentTab_ = currentTabs_.value(index, "Profile");
            onTabSelected_(currentTab_);
        });
        currentTabs_ = tabs;
        for (int i = 0; i < currentTabs_.size(); ++i) {
            auto *button = new QPushButton(currentTabs_.at(i));
            button->setObjectName("ledgerTab");
            button->setProperty("tabName", currentTabs_.at(i));
            button->setProperty("tabIndex", i);
            button->setProperty("firstTab", i == 0);
            button->setProperty("lastTab", i == currentTabs_.size() - 1);
            button->setCheckable(true);
            if (i == 0) {
                button->setChecked(true);
            }
            tabGroup_->addButton(button, i);
            tabLayout_->addWidget(button);
        }
        tabLayout_->addStretch(1);
    }

    QString currentTab_ = "Profile";
    QStringList currentTabs_;
    std::function<void(QString)> onTabSelected_;
    QStackedWidget *pages_ = nullptr;
    QButtonGroup *tabGroup_ = nullptr;
    QHBoxLayout *tabLayout_ = nullptr;
};

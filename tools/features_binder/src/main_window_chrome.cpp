#include "main_window.h"

#include <QLabel>
#include <QSize>
#include <QSizePolicy>
#include <QToolBar>
#include <QToolButton>
#include <QWidget>

#include "binder_navigation.h"
#include "detail_lens_rail.h"
#include "ledger_view.h"
#include "project_rail.h"
#include "right_context_panel.h"
#include "sheet_stack_body.h"

void DexHomeV2Window::buildToolbar() {
    auto *toolbar = new QToolBar("Dex Home v2 chrome");
    toolbar->setMovable(false);
    toolbar->setFloatable(false);
    toolbar->setIconSize(QSize(16, 16));
    addToolBar(Qt::TopToolBarArea, toolbar);

    auto *appLabel = new QLabel("Dex Home");
    appLabel->setObjectName("chromeTitle");
    toolbar->addWidget(appLabel);

    chromeLocationLabel_ = new QLabel;
    chromeLocationLabel_->setObjectName("chromeLocation");
    toolbar->addWidget(chromeLocationLabel_);

    auto *spacer = new QWidget;
    spacer->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Preferred);
    toolbar->addWidget(spacer);

    railToggle_ = new QToolButton;
    railToggle_->setText("Rail");
    railToggle_->setCheckable(true);
    railToggle_->setChecked(true);
    railToggle_->setToolTip("Toggle project rail");
    toolbar->addWidget(railToggle_);

    bottomToggle_ = new QToolButton;
    bottomToggle_->setText("Shelf");
    bottomToggle_->setCheckable(false);
    bottomToggle_->setToolTip("Open the Text Editor workspace");
    toolbar->addWidget(bottomToggle_);

    repoModeToggle_ = new QToolButton;
    repoModeToggle_->setText("Repo");
    repoModeToggle_->setCheckable(true);
    repoModeToggle_->setChecked(repoMode_);
    repoModeToggle_->setToolTip("Toggle repo binder mode");
    toolbar->addWidget(repoModeToggle_);

    contextToggle_ = new QToolButton;
    contextToggle_->setText("Context");
    contextToggle_->setCheckable(true);
    contextToggle_->setChecked(true);
    contextToggle_->setToolTip("Toggle right context panel");
    toolbar->addWidget(contextToggle_);
}

void DexHomeV2Window::buildBody() {
    body_ = new SheetStackBody;

    projectRail_ = new ProjectRail(
        [this](const QString &workerId) {
            settingsMode_ = false;
            setSelectedWorker(workerId);
        },
        [this](const QString &projectId) {
            settingsMode_ = false;
            setSelectedProject(projectId);
        },
        [this]() {
            setSettingsMode(true);
        },
        body_);
    ledger_ = new LedgerView([this](const QString &tabName) {
        setTopTab(tabName);
    }, body_);
    detailLensRail_ = new DetailLensRail([this](const QString &lensName) {
        setDetailLens(lensName);
    }, body_);
    rightContext_ = new RightContextPanel(
        [this]() {
            addGradeRecord();
        },
        [this](const QString &decision) {
            addReviewVerdict(decision);
        },
        body_);

    body_->setSurfaces(projectRail_, ledger_, detailLensRail_, rightContext_);
    setCentralWidget(body_);

    connect(railToggle_, &QToolButton::toggled, projectRail_, &QWidget::setVisible);
    connect(railToggle_, &QToolButton::toggled, body_, &SheetStackBody::relayoutSheets);
    connect(contextToggle_, &QToolButton::toggled, rightContext_, &QWidget::setVisible);
    connect(contextToggle_, &QToolButton::toggled, body_, &SheetStackBody::relayoutSheets);
    connect(repoModeToggle_, &QToolButton::toggled, this, [this](bool enabled) {
        setRepoBinderMode(enabled);
    });
    connect(bottomToggle_, &QToolButton::clicked, this, [this]() {
        openTextEditorWorkspace();
    });

}

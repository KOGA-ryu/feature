#include "project_registry_spec_page.h"

#include <QCheckBox>
#include <QDir>
#include <QFormLayout>
#include <QGridLayout>
#include <QHeaderView>
#include <QAbstractItemView>
#include <QLabel>
#include <QLineEdit>
#include <QMessageBox>
#include <QFileInfo>
#include <QPlainTextEdit>
#include <QPushButton>
#include <QScrollArea>
#include <QTableWidget>
#include <QVBoxLayout>

#include <utility>

#include "binder_page_helpers.h"
#include "render_helpers.h"
#include "repo_binder_template.h"
#include "settings_shortcuts_dialog.h"

namespace {

QLineEdit *lineEdit(const QString &value = QString()) {
    auto *edit = new QLineEdit;
    edit->setText(value);
    edit->setMinimumHeight(22);
    return edit;
}

QPlainTextEdit *linesEdit(const QStringList &values = {}) {
    auto *edit = new QPlainTextEdit;
    edit->setPlainText(values.join('\n'));
    edit->setMinimumHeight(56);
    return edit;
}

QTableWidgetItem *tableItem(const QString &value) {
    auto *item = new QTableWidgetItem(value);
    item->setFlags(item->flags() | Qt::ItemIsEditable);
    return item;
}

QString cellText(const QTableWidget *table, int row, int column) {
    const QTableWidgetItem *item = table->item(row, column);
    return item ? item->text().trimmed() : QString();
}

void addFormField(QFormLayout *form, const QString &label, QWidget *field) {
    form->addRow(new QLabel(label), field);
}

QStringList commaList(const QString &value) {
    QStringList result;
    for (QString item : value.split(',')) {
        item = item.trimmed();
        if (!item.isEmpty()) {
            result.push_back(item);
        }
    }
    return result;
}

QString joinCommaList(const QStringList &values) {
    return values.join(", ");
}

QString rowsText(const QVector<DexRepoBinderTemplate::BinderTemplateRow> &rows) {
    QStringList lines;
    for (const DexRepoBinderTemplate::BinderTemplateRow &row : rows) {
        QString line = row.cells.join(" | ");
        if (row.risk) {
            line = "! " + line;
        }
        lines.push_back(line);
    }
    return lines.join('\n');
}

QVector<DexRepoBinderTemplate::BinderTemplateRow> parseRowsText(const QString &text) {
    QVector<DexRepoBinderTemplate::BinderTemplateRow> rows;
    for (QString line : text.split('\n')) {
        line = line.trimmed();
        if (line.isEmpty()) {
            continue;
        }
        DexRepoBinderTemplate::BinderTemplateRow row;
        if (line.startsWith("!")) {
            row.risk = true;
            line = line.mid(1).trimmed();
        }
        for (QString cell : line.split('|')) {
            cell = cell.trimmed();
            if (!cell.isEmpty()) {
                row.cells.push_back(cell);
            }
        }
        if (!row.cells.isEmpty()) {
            rows.push_back(row);
        }
    }
    return rows;
}

DexRepoBinderTemplate::BinderTemplate loadEditableTemplate(
    const DexProjects::ProjectRegistry &registry,
    const DexProjects::ProjectRegistryEntry &project) {
    DexRepoBinderTemplate::BinderTemplate fallback = DexRepoBinderTemplate::defaultBinderTemplate();
    fallback.templateId = project.binderTemplate;
    fallback.displayName = project.binderTemplate;
    fallback.sections.clear();
    fallback.contextLines.clear();
    if (project.binderTemplate.isEmpty() || registry.sourcePath.isEmpty()) {
        fallback.templateId.clear();
        fallback.displayName.clear();
        return fallback;
    }

    const QString templatePath = QFileInfo(registry.sourcePath)
        .absoluteDir()
        .filePath("binder_templates/" + project.binderTemplate + ".json");
    const DexRepoBinderTemplate::BinderTemplate loaded =
        DexRepoBinderTemplate::loadBinderTemplateFile(templatePath);
    if (loaded.loaded) {
        return loaded;
    }
    return fallback;
}

QString preservedCodexAccount(
    const QVector<DexProjects::ProjectRegistrySpecWorkerRow> &workers,
    const QString &workerId) {
    for (const DexProjects::ProjectRegistrySpecWorkerRow &worker : workers) {
        if (worker.workerId == workerId) {
            return worker.codexAccount;
        }
    }
    return {};
}

} // namespace

ProjectRegistrySpecPage::ProjectRegistrySpecPage(
    DexProjects::ProjectRegistry registry,
    QString selectedProjectId,
    std::function<void(DexProjects::ProjectRegistry, QString, DexRepoBinderTemplate::BinderTemplate)> onSave,
    std::function<void()> onRevert,
    std::function<void()> onBack,
    std::function<void(DexProjects::ProjectRegistry, QString, DexRepoBinderTemplate::BinderTemplate)> onSaveAndBack,
    QWidget *parent)
    : QWidget(parent),
      registry_(std::move(registry)),
      draft_(DexProjects::projectRegistrySpecDraft(registry_, selectedProjectId)),
      onSave_(std::move(onSave)),
      onRevert_(std::move(onRevert)),
      onBack_(std::move(onBack)),
      onSaveAndBack_(std::move(onSaveAndBack)) {
    draft_.binderTemplate = loadEditableTemplate(registry_, draft_.project);
    auto *outer = new QVBoxLayout(this);
    outer->setContentsMargins(0, 0, 0, 0);
    outer->setSpacing(0);

    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);

    auto *body = new QWidget;
    auto *layout = new QVBoxLayout(body);
    layout->setContentsMargins(12, 12, 12, 12);
    layout->setSpacing(10);

    auto *title = DexBinderPages::makeStatsSection("project registry spec sheet", true);
    auto *titleLayout = static_cast<QVBoxLayout *>(title->layout());
    titleLayout->addWidget(DexBinderPages::makeStatsText(
        "Settings edits the selected repo binder spec. Nothing is written until Save Binder Spec is pressed."));
    titleLayout->addWidget(DexBinderPages::makeStatsRow({
        "registry",
        registry_.sourcePath.isEmpty() ? QString("unknown") : registry_.sourcePath,
        "selected project",
        draft_.originalProjectId.isEmpty() ? QString("new project") : draft_.originalProjectId,
        "workers",
        QString::number(draft_.workers.size()),
    }, false, false));
    layout->addWidget(title);

    auto *actions = new QWidget;
    auto *actionsLayout = new QHBoxLayout(actions);
    actionsLayout->setContentsMargins(0, 0, 0, 0);
    auto *back = new QPushButton("Back to Binder");
    back->setObjectName("statsContextAction");
    auto *save = new QPushButton("Save Binder Spec");
    save->setObjectName("primaryAction");
    auto *saveAndBack = new QPushButton("Save and Back");
    saveAndBack->setObjectName("primaryAction");
    auto *revert = new QPushButton("Cancel / Revert");
    revert->setObjectName("statsContextAction");
    auto *shortcuts = new QPushButton("Commands / Hotkeys");
    shortcuts->setObjectName("statsContextAction");
    connect(back, &QPushButton::clicked, this, [this]() {
        if (onBack_) {
            onBack_();
        }
    });
    connect(save, &QPushButton::clicked, this, [this]() {
        saveDraft(false);
    });
    connect(saveAndBack, &QPushButton::clicked, this, [this]() {
        saveDraft(true);
    });
    connect(revert, &QPushButton::clicked, this, [this]() {
        if (onRevert_) {
            onRevert_();
        }
    });
    connect(shortcuts, &QPushButton::clicked, this, [this]() {
        DexSettingsShortcuts::showShortcutsDialog(this);
    });
    actionsLayout->addWidget(back);
    actionsLayout->addWidget(save);
    actionsLayout->addWidget(saveAndBack);
    actionsLayout->addWidget(revert);
    actionsLayout->addWidget(shortcuts);
    actionsLayout->addStretch(1);
    layout->addWidget(actions);

    auto *identity = DexBinderPages::makeStatsSection("project identity");
    auto *identityForm = new QFormLayout;
    identityForm->setContentsMargins(0, 0, 0, 0);
    identityForm->setSpacing(6);
    projectId_ = lineEdit(draft_.project.projectId);
    name_ = lineEdit(draft_.project.name);
    path_ = lineEdit(draft_.project.path);
    role_ = lineEdit(draft_.project.role);
    status_ = lineEdit(draft_.project.status);
    authority_ = lineEdit(draft_.project.authority);
    projectType_ = lineEdit(draft_.project.projectType);
    binderTemplate_ = lineEdit(draft_.project.binderTemplate);
    pinned_ = new QCheckBox("Pinned in left rail");
    pinned_->setChecked(draft_.project.pinned);
    addFormField(identityForm, "project id", projectId_);
    addFormField(identityForm, "name", name_);
    addFormField(identityForm, "path", path_);
    addFormField(identityForm, "role", role_);
    addFormField(identityForm, "status", status_);
    addFormField(identityForm, "authority", authority_);
    addFormField(identityForm, "project type", projectType_);
    addFormField(identityForm, "binder template", binderTemplate_);
    addFormField(identityForm, "left rail", pinned_);
    static_cast<QVBoxLayout *>(identity->layout())->addLayout(identityForm);
    layout->addWidget(identity);

    auto *workerSection = DexBinderPages::makeStatsSection("worker roster", true);
    auto *workerLayout = static_cast<QVBoxLayout *>(workerSection->layout());
    workerLayout->addWidget(DexBinderPages::makeStatsText(
        "Workers listed here become the stateful left-rail worker lenses for this project."));
    workers_ = new QTableWidget(0, 9);
    workers_->setHorizontalHeaderLabels({
        "worker id",
        "role",
        "display name",
        "status",
        "codex session id",
        "model",
        "directory",
        "permissions",
        "linked context snapshot",
    });
    workers_->horizontalHeader()->setStretchLastSection(true);
    workers_->horizontalHeader()->setSectionResizeMode(QHeaderView::Stretch);
    workers_->verticalHeader()->hide();
    workers_->setSelectionBehavior(QAbstractItemView::SelectRows);
    workers_->setSelectionMode(QAbstractItemView::SingleSelection);
    workers_->setMinimumHeight(118);
    for (const DexProjects::ProjectRegistrySpecWorkerRow &worker : draft_.workers) {
        addWorkerRow(worker);
    }
    workerLayout->addWidget(workers_);
    auto *workerActions = new QWidget;
    auto *workerActionsLayout = new QHBoxLayout(workerActions);
    workerActionsLayout->setContentsMargins(0, 0, 0, 0);
    auto *addWorker = new QPushButton("Add worker row");
    addWorker->setObjectName("statsContextAction");
    auto *removeWorker = new QPushButton("Remove selected rows");
    removeWorker->setObjectName("statsContextAction");
    connect(addWorker, &QPushButton::clicked, this, [this]() {
        addWorkerRow();
    });
    connect(removeWorker, &QPushButton::clicked, this, [this]() {
        removeSelectedWorkerRows();
    });
    workerActionsLayout->addWidget(addWorker);
    workerActionsLayout->addWidget(removeWorker);
    workerActionsLayout->addStretch(1);
    workerLayout->addWidget(workerActions);

    auto *codexImport = DexBinderPages::makeStatsSection("import codex /status");
    auto *codexImportLayout = static_cast<QVBoxLayout *>(codexImport->layout());
    codexImportLayout->addWidget(DexBinderPages::makeStatsText(
        "Paste Codex /status output, select one worker row, then apply. This only fills the selected row; Save Binder Spec writes it."));
    codexStatusPaste_ = new QPlainTextEdit;
    codexStatusPaste_->setMinimumHeight(92);
    codexStatusPaste_->setPlaceholderText("Paste /status output here.");
    codexImportLayout->addWidget(codexStatusPaste_);
    auto *applyStatus = new QPushButton("Apply to selected worker");
    applyStatus->setObjectName("statsContextAction");
    connect(applyStatus, &QPushButton::clicked, this, [this]() {
        applyCodexStatusToSelectedWorker();
    });
    codexImportLayout->addWidget(applyStatus);
    workerLayout->addWidget(codexImport);
    layout->addWidget(workerSection);

    auto *boundaries = DexBinderPages::makeStatsSection("boundaries and commands", true);
    auto *boundaryForm = new QFormLayout;
    boundaryForm->setContentsMargins(0, 0, 0, 0);
    boundaryForm->setSpacing(6);
    safeEditZones_ = linesEdit(draft_.project.safeEditZones);
    protectedZones_ = linesEdit(draft_.project.protectedZones);
    generatedZones_ = linesEdit(draft_.project.generatedZones);
    scratchZones_ = linesEdit(draft_.project.scratchZones);
    sourceDocs_ = linesEdit(draft_.project.sourceDocs);
    buildCommands_ = linesEdit(draft_.project.buildCommands);
    testCommands_ = linesEdit(draft_.project.testCommands);
    proofCommands_ = linesEdit(draft_.project.proofCommands);
    const QVector<QPair<QString, QPlainTextEdit *>> textFields = {
        {"safe edit zones", safeEditZones_},
        {"protected zones", protectedZones_},
        {"generated zones", generatedZones_},
        {"scratch zones", scratchZones_},
        {"source docs", sourceDocs_},
        {"build commands", buildCommands_},
        {"test commands", testCommands_},
        {"proof commands", proofCommands_},
    };
    for (const auto &field : textFields) {
        addFormField(boundaryForm, field.first, field.second);
    }
    static_cast<QVBoxLayout *>(boundaries->layout())->addLayout(boundaryForm);
    layout->addWidget(boundaries);

    auto *ide = DexBinderPages::makeStatsSection("ide seam");
    auto *ideForm = new QFormLayout;
    ideForm->setContentsMargins(0, 0, 0, 0);
    ideForm->setSpacing(6);
    editorCoreStatus_ = lineEdit(draft_.project.editorCoreStatus);
    editorCorePath_ = lineEdit(draft_.project.editorCorePath);
    fileInspectSupported_ = new QCheckBox("file inspect supported");
    fileInspectSupported_->setChecked(draft_.project.fileInspectSupported);
    fileEditSupported_ = new QCheckBox("file edit supported");
    fileEditSupported_->setChecked(draft_.project.fileEditSupported);
    addFormField(ideForm, "editor core status", editorCoreStatus_);
    addFormField(ideForm, "editor core path", editorCorePath_);
    addFormField(ideForm, "inspect", fileInspectSupported_);
    addFormField(ideForm, "edit", fileEditSupported_);
    static_cast<QVBoxLayout *>(ide->layout())->addLayout(ideForm);
    layout->addWidget(ide);

    auto *features = DexBinderPages::makeStatsSection("feature summary", true);
    auto *featuresForm = new QFormLayout;
    featuresForm->setContentsMargins(0, 0, 0, 0);
    featuresForm->setSpacing(6);
    featurePackTotal_ = lineEdit(QString::number(draft_.project.featurePackTotal));
    featurePackLogic_ = lineEdit(QString::number(draft_.project.featurePackLogic));
    featurePackSim_ = lineEdit(QString::number(draft_.project.featurePackSim));
    featurePackUi_ = lineEdit(QString::number(draft_.project.featurePackUi));
    featurePackWorkflows_ = lineEdit(QString::number(draft_.project.featurePackWorkflows));
    addFormField(featuresForm, "total", featurePackTotal_);
    addFormField(featuresForm, "logic", featurePackLogic_);
    addFormField(featuresForm, "sim", featurePackSim_);
    addFormField(featuresForm, "ui", featurePackUi_);
    addFormField(featuresForm, "workflows", featurePackWorkflows_);
    static_cast<QVBoxLayout *>(features->layout())->addLayout(featuresForm);
    layout->addWidget(features);

    auto *templateSection = DexBinderPages::makeStatsSection("custom binder template", true);
    auto *templateLayout = static_cast<QVBoxLayout *>(templateSection->layout());
    templateLayout->addWidget(DexBinderPages::makeStatsText(
        "Template rows use cells separated by ` | `. Prefix a row with `! ` to render it as a risk row."));
    auto *templateForm = new QFormLayout;
    templateForm->setContentsMargins(0, 0, 0, 0);
    templateForm->setSpacing(6);
    templateId_ = lineEdit(draft_.binderTemplate.templateId);
    templateDisplayName_ = lineEdit(draft_.binderTemplate.displayName);
    templateContextLines_ = linesEdit(draft_.binderTemplate.contextLines);
    addFormField(templateForm, "template id", templateId_);
    addFormField(templateForm, "display name", templateDisplayName_);
    addFormField(templateForm, "context lines", templateContextLines_);
    templateLayout->addLayout(templateForm);

    templateSections_ = new QTableWidget(0, 5);
    templateSections_->setHorizontalHeaderLabels({"tab", "lenses", "title", "subtle", "rows"});
    templateSections_->horizontalHeader()->setSectionResizeMode(QHeaderView::Stretch);
    templateSections_->verticalHeader()->hide();
    templateSections_->setMinimumHeight(170);
    for (const DexRepoBinderTemplate::BinderTemplateSection &section : draft_.binderTemplate.sections) {
        addTemplateSectionRow(section);
    }
    templateLayout->addWidget(templateSections_);
    auto *templateActions = new QWidget;
    auto *templateActionsLayout = new QHBoxLayout(templateActions);
    templateActionsLayout->setContentsMargins(0, 0, 0, 0);
    auto *addTemplateSection = new QPushButton("Add template section");
    addTemplateSection->setObjectName("statsContextAction");
    auto *removeTemplateSection = new QPushButton("Remove selected sections");
    removeTemplateSection->setObjectName("statsContextAction");
    connect(addTemplateSection, &QPushButton::clicked, this, [this]() {
        addTemplateSectionRow();
    });
    connect(removeTemplateSection, &QPushButton::clicked, this, [this]() {
        removeSelectedTemplateSectionRows();
    });
    templateActionsLayout->addWidget(addTemplateSection);
    templateActionsLayout->addWidget(removeTemplateSection);
    templateActionsLayout->addStretch(1);
    templateLayout->addWidget(templateActions);
    layout->addWidget(templateSection);

    layout->addStretch(1);

    scroll->setWidget(body);
    outer->addWidget(scroll, 1);
}

QStringList ProjectRegistrySpecPage::textLines(QPlainTextEdit *edit) const {
    QStringList values;
    for (QString line : edit->toPlainText().split('\n')) {
        line = line.trimmed();
        if (!line.isEmpty()) {
            values.push_back(line);
        }
    }
    return values;
}

void ProjectRegistrySpecPage::setTextLines(QPlainTextEdit *edit, const QStringList &values) {
    edit->setPlainText(values.join('\n'));
}

void ProjectRegistrySpecPage::addWorkerRow(const DexProjects::ProjectRegistrySpecWorkerRow &worker) {
    const int row = workers_->rowCount();
    workers_->insertRow(row);
    workers_->setItem(row, 0, tableItem(worker.workerId));
    workers_->setItem(row, 1, tableItem(worker.role));
    workers_->setItem(row, 2, tableItem(worker.displayName));
    workers_->setItem(row, 3, tableItem(worker.status));
    workers_->setItem(row, 4, tableItem(worker.codexSessionId));
    workers_->setItem(row, 5, tableItem(worker.codexModel));
    workers_->setItem(row, 6, tableItem(worker.codexDirectory));
    workers_->setItem(row, 7, tableItem(worker.codexPermissions));
    workers_->setItem(row, 8, tableItem(worker.codexContextWindow));
}

void ProjectRegistrySpecPage::removeSelectedWorkerRows() {
    QList<int> rows;
    for (const QModelIndex &index : workers_->selectionModel()->selectedRows()) {
        rows.push_back(index.row());
    }
    std::sort(rows.begin(), rows.end(), std::greater<int>());
    for (const int row : rows) {
        workers_->removeRow(row);
    }
}

void ProjectRegistrySpecPage::applyCodexStatusToSelectedWorker() {
    const int row = workers_->currentRow();
    if (row < 0 || row >= workers_->rowCount()) {
        QMessageBox::warning(this, "Codex Status Import", "Select one worker row before applying /status.");
        return;
    }
    const DexProjects::CodexStatusImport status =
        DexProjects::parseCodexStatusText(codexStatusPaste_->toPlainText());
    if (!status.hasAnyValue()) {
        QMessageBox::warning(this, "Codex Status Import", "No Codex /status fields were found.");
        return;
    }
    DexProjects::ProjectRegistrySpecWorkerRow worker;
    worker.workerId = cellText(workers_, row, 0);
    worker.role = cellText(workers_, row, 1);
    worker.displayName = cellText(workers_, row, 2);
    worker.status = cellText(workers_, row, 3);
    worker.codexSessionId = cellText(workers_, row, 4);
    worker.codexModel = cellText(workers_, row, 5);
    worker.codexDirectory = cellText(workers_, row, 6);
    worker.codexPermissions = cellText(workers_, row, 7);
    worker.codexContextWindow = cellText(workers_, row, 8);
    worker.codexAccount = preservedCodexAccount(draft_.workers, worker.workerId);
    DexProjects::applyCodexStatusImport(&worker, status);
    workers_->setItem(row, 4, tableItem(worker.codexSessionId));
    workers_->setItem(row, 5, tableItem(worker.codexModel));
    workers_->setItem(row, 6, tableItem(worker.codexDirectory));
    workers_->setItem(row, 7, tableItem(worker.codexPermissions));
    workers_->setItem(row, 8, tableItem(worker.codexContextWindow));
}

void ProjectRegistrySpecPage::addTemplateSectionRow(const DexRepoBinderTemplate::BinderTemplateSection &section) {
    const int row = templateSections_->rowCount();
    templateSections_->insertRow(row);
    templateSections_->setItem(row, 0, tableItem(section.tab));
    templateSections_->setItem(row, 1, tableItem(joinCommaList(section.lenses)));
    templateSections_->setItem(row, 2, tableItem(section.title));
    templateSections_->setItem(row, 3, tableItem(section.subtle ? "true" : "false"));
    templateSections_->setItem(row, 4, tableItem(rowsText(section.rows)));
}

void ProjectRegistrySpecPage::removeSelectedTemplateSectionRows() {
    QList<int> rows;
    for (const QModelIndex &index : templateSections_->selectionModel()->selectedRows()) {
        rows.push_back(index.row());
    }
    std::sort(rows.begin(), rows.end(), std::greater<int>());
    for (const int row : rows) {
        templateSections_->removeRow(row);
    }
}

DexProjects::ProjectRegistrySpecDraft ProjectRegistrySpecPage::collectDraft() const {
    DexProjects::ProjectRegistrySpecDraft draft = draft_;
    draft.project.projectId = projectId_->text().trimmed();
    draft.project.name = name_->text().trimmed();
    draft.project.path = path_->text().trimmed();
    draft.project.role = role_->text().trimmed();
    draft.project.status = status_->text().trimmed();
    draft.project.authority = authority_->text().trimmed();
    draft.project.projectType = projectType_->text().trimmed();
    draft.project.binderTemplate = binderTemplate_->text().trimmed();
    draft.project.pinned = pinned_->isChecked();
    draft.project.safeEditZones = textLines(safeEditZones_);
    draft.project.protectedZones = textLines(protectedZones_);
    draft.project.generatedZones = textLines(generatedZones_);
    draft.project.scratchZones = textLines(scratchZones_);
    draft.project.sourceDocs = textLines(sourceDocs_);
    draft.project.buildCommands = textLines(buildCommands_);
    draft.project.testCommands = textLines(testCommands_);
    draft.project.proofCommands = textLines(proofCommands_);
    draft.project.editorCoreStatus = editorCoreStatus_->text().trimmed();
    draft.project.editorCorePath = editorCorePath_->text().trimmed();
    draft.project.fileInspectSupported = fileInspectSupported_->isChecked();
    draft.project.fileEditSupported = fileEditSupported_->isChecked();
    draft.project.featurePackTotal = featurePackTotal_->text().trimmed().toInt();
    draft.project.featurePackLogic = featurePackLogic_->text().trimmed().toInt();
    draft.project.featurePackSim = featurePackSim_->text().trimmed().toInt();
    draft.project.featurePackUi = featurePackUi_->text().trimmed().toInt();
    draft.project.featurePackWorkflows = featurePackWorkflows_->text().trimmed().toInt();
    draft.workers.clear();
    for (int row = 0; row < workers_->rowCount(); ++row) {
        DexProjects::ProjectRegistrySpecWorkerRow worker;
        worker.workerId = cellText(workers_, row, 0);
        worker.role = cellText(workers_, row, 1);
        worker.displayName = cellText(workers_, row, 2);
        worker.status = cellText(workers_, row, 3);
        worker.codexSessionId = cellText(workers_, row, 4);
        worker.codexModel = cellText(workers_, row, 5);
        worker.codexDirectory = cellText(workers_, row, 6);
        worker.codexPermissions = cellText(workers_, row, 7);
        worker.codexContextWindow = cellText(workers_, row, 8);
        worker.codexAccount = preservedCodexAccount(draft_.workers, worker.workerId);
        if (!worker.workerId.isEmpty()) {
            draft.workers.push_back(worker);
        }
    }

    const QString templateId = templateId_->text().trimmed().isEmpty()
        ? draft.project.binderTemplate
        : templateId_->text().trimmed();
    draft.project.binderTemplate = templateId;
    draft.binderTemplate.templateId = templateId;
    draft.binderTemplate.displayName = templateDisplayName_->text().trimmed();
    draft.binderTemplate.contextLines = textLines(templateContextLines_);
    draft.binderTemplate.sections.clear();
    DexRepoBinderTemplate::BinderTemplate defaults = DexRepoBinderTemplate::defaultBinderTemplate();
    draft.binderTemplate.topTabs = draft.binderTemplate.topTabs.isEmpty()
        ? defaults.topTabs
        : draft.binderTemplate.topTabs;
    draft.binderTemplate.detailLenses = draft.binderTemplate.detailLenses.isEmpty()
        ? defaults.detailLenses
        : draft.binderTemplate.detailLenses;
    for (int row = 0; row < templateSections_->rowCount(); ++row) {
        DexRepoBinderTemplate::BinderTemplateSection section;
        section.tab = cellText(templateSections_, row, 0);
        section.lenses = commaList(cellText(templateSections_, row, 1));
        section.title = cellText(templateSections_, row, 2);
        const QString subtle = cellText(templateSections_, row, 3).toLower();
        section.subtle = subtle == "true" || subtle == "yes" || subtle == "1";
        section.rows = parseRowsText(cellText(templateSections_, row, 4));
        if (!section.tab.isEmpty() && !section.title.isEmpty()) {
            draft.binderTemplate.sections.push_back(section);
        }
    }
    return draft;
}

bool ProjectRegistrySpecPage::validateDraft(const DexProjects::ProjectRegistrySpecDraft &draft) const {
    if (draft.project.projectId.isEmpty()) {
        QMessageBox::warning(const_cast<ProjectRegistrySpecPage *>(this), "Project Registry", "Project id is required.");
        return false;
    }
    for (const DexProjects::ProjectRegistryEntry &project : registry_.projects) {
        if (project.projectId == draft.project.projectId && project.projectId != draft.originalProjectId) {
            QMessageBox::warning(const_cast<ProjectRegistrySpecPage *>(this), "Project Registry", "Project id already exists.");
            return false;
        }
    }
    QStringList workerIds;
    for (const DexProjects::ProjectRegistrySpecWorkerRow &worker : draft.workers) {
        if (workerIds.contains(worker.workerId)) {
            QMessageBox::warning(const_cast<ProjectRegistrySpecPage *>(this), "Project Registry", "Duplicate worker id: " + worker.workerId);
            return false;
        }
        workerIds.push_back(worker.workerId);
    }
    if (!draft.project.binderTemplate.isEmpty() && draft.project.binderTemplate.contains('/')) {
        QMessageBox::warning(const_cast<ProjectRegistrySpecPage *>(this), "Binder Template", "Template id must not contain '/'.");
        return false;
    }
    return true;
}

void ProjectRegistrySpecPage::saveDraft(bool exitAfterSave) {
    const DexProjects::ProjectRegistrySpecDraft draft = collectDraft();
    if (!validateDraft(draft)) {
        return;
    }
    DexProjects::ProjectRegistry next = DexProjects::applyProjectRegistrySpecDraft(registry_, draft);
    if (exitAfterSave && onSaveAndBack_) {
        onSaveAndBack_(next, draft.project.projectId, draft.binderTemplate);
        return;
    }
    if (onSave_) {
        onSave_(next, draft.project.projectId, draft.binderTemplate);
    }
}

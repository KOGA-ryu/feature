#include "main_window.h"

#include <QMessageBox>
#include <QDir>
#include <QFileInfo>
#include <QToolButton>

#include "project_registry.h"
#include "repo_binder_template.h"

void DexHomeV2Window::exitSettingsMode() {
    settingsMode_ = false;
    repoMode_ = true;
    if (repoModeToggle_) {
        repoModeToggle_->setChecked(true);
    }
    if (selectedTopTab_.isEmpty()) {
        selectedTopTab_ = "Profile";
    }
    if (selectedDetailLens_.isEmpty()) {
        selectedDetailLens_ = "Dashboard";
    }
    refreshSelectedProjectScan();
    refreshViews();
}

void DexHomeV2Window::saveProjectRegistryFromSettings(
    DexProjects::ProjectRegistry registry,
    const QString &selectedProjectId,
    DexRepoBinderTemplate::BinderTemplate binderTemplate,
    bool exitAfterSave) {
    if (registry.sourcePath.isEmpty()) {
        registry.sourcePath = projectRegistryPath_;
    }
    QString error;
    if (!DexProjects::saveProjectRegistryFile(registry, &error)) {
        QMessageBox::warning(this, "Project Registry", error);
        return;
    }
    if (!binderTemplate.templateId.isEmpty()) {
        const QString templateDir = QFileInfo(registry.sourcePath).absoluteDir().filePath("binder_templates");
        QDir().mkpath(templateDir);
        const QString templatePath = QDir(templateDir).filePath(binderTemplate.templateId + ".json");
        QString templateError;
        if (!DexRepoBinderTemplate::saveBinderTemplateFile(binderTemplate, templatePath, &templateError)) {
            QMessageBox::warning(this, "Binder Template", templateError);
            return;
        }
    }
    selectedProjectId_ = selectedProjectId;
    settingsMode_ = !exitAfterSave;
    repoMode_ = true;
    reloadState();
}

void DexHomeV2Window::revertProjectRegistrySettings() {
    settingsMode_ = true;
    reloadState();
}

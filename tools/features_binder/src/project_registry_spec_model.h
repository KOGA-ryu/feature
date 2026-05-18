#pragma once

#include <QString>
#include <QStringList>
#include <QVector>

#include "project_registry.h"
#include "repo_binder_template.h"

namespace DexProjects {

struct ProjectRegistrySpecWorkerRow {
    QString workerId;
    QString role;
    QString displayName;
    QString status;
    QString codexSessionId;
    QString codexModel;
    QString codexDirectory;
    QString codexPermissions;
    QString codexContextWindow;
    QString codexAccount;
};

struct CodexStatusImport {
    QString sessionId;
    QString model;
    QString directory;
    QString permissions;
    QString contextWindow;

    bool hasAnyValue() const;
};

struct ProjectRegistrySpecDraft {
    QString originalProjectId;
    ProjectRegistryEntry project;
    QVector<ProjectRegistrySpecWorkerRow> workers;
    DexRepoBinderTemplate::BinderTemplate binderTemplate;
};

ProjectRegistrySpecDraft projectRegistrySpecDraft(
    const ProjectRegistry &registry,
    const QString &selectedProjectId);

ProjectRegistry applyProjectRegistrySpecDraft(
    const ProjectRegistry &registry,
    const ProjectRegistrySpecDraft &draft);

CodexStatusImport parseCodexStatusText(const QString &text);
void applyCodexStatusImport(ProjectRegistrySpecWorkerRow *worker, const CodexStatusImport &status);

} // namespace DexProjects

#include "project_registry.h"

#include <QFile>
#include <QFileInfo>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QJsonParseError>
#include <QSaveFile>

namespace DexProjects {
namespace {

QJsonArray stringListToJson(const QStringList &values) {
    QJsonArray array;
    for (const QString &value : values) {
        array.push_back(value);
    }
    return array;
}

QJsonObject workerToJson(const WorkerRegistryEntry &worker) {
    QJsonObject object;
    object["worker_id"] = worker.workerId;
    object["role"] = worker.role;
    object["display_name"] = worker.displayName;
    object["status"] = worker.status;
    if (!worker.codexSessionId.isEmpty()) {
        object["codex_session_id"] = worker.codexSessionId;
    }
    if (!worker.codexModel.isEmpty()) {
        object["codex_model"] = worker.codexModel;
    }
    if (!worker.codexDirectory.isEmpty()) {
        object["codex_directory"] = worker.codexDirectory;
    }
    if (!worker.codexPermissions.isEmpty()) {
        object["codex_permissions"] = worker.codexPermissions;
    }
    if (!worker.codexContextWindow.isEmpty()) {
        object["codex_context_window"] = worker.codexContextWindow;
    }
    if (!worker.codexAccount.isEmpty()) {
        object["codex_account"] = worker.codexAccount;
    }
    return object;
}

QJsonObject projectToJson(const ProjectRegistryEntry &project) {
    QJsonObject object;
    object["project_id"] = project.projectId;
    object["name"] = project.name;
    object["path"] = project.path;
    object["role"] = project.role;
    object["status"] = project.status;
    object["authority"] = project.authority;
    object["project_type"] = project.projectType;
    object["pinned"] = project.pinned;
    object["worker_ids"] = stringListToJson(project.workerIds);
    object["safe_edit_zones"] = stringListToJson(project.safeEditZones);
    object["protected_zones"] = stringListToJson(project.protectedZones);
    object["generated_zones"] = stringListToJson(project.generatedZones);
    object["scratch_zones"] = stringListToJson(project.scratchZones);
    object["source_docs"] = stringListToJson(project.sourceDocs);
    object["build_commands"] = stringListToJson(project.buildCommands);
    object["test_commands"] = stringListToJson(project.testCommands);
    object["proof_commands"] = stringListToJson(project.proofCommands);
    object["editor_core_status"] = project.editorCoreStatus;
    object["editor_core_path"] = project.editorCorePath;
    object["file_inspect_supported"] = project.fileInspectSupported;
    object["file_edit_supported"] = project.fileEditSupported;
    if (!project.binderTemplate.isEmpty()) {
        object["binder_template"] = project.binderTemplate;
    }
    if (project.featurePackTotal > 0 || project.featurePackLogic > 0 || project.featurePackSim > 0 || project.featurePackUi > 0 || project.featurePackWorkflows > 0) {
        object["feature_pack_summary"] = QJsonObject{
            {"total", project.featurePackTotal},
            {"groups", QJsonObject{
                {"logic", project.featurePackLogic},
                {"sim", project.featurePackSim},
                {"ui", project.featurePackUi},
                {"workflows", project.featurePackWorkflows},
            }},
        };
    }
    return object;
}

} // namespace

ProjectRegistry loadProjectRegistryFile(const QString &path) {
    ProjectRegistry registry;
    registry.sourcePath = path;
    if (path.isEmpty()) {
        registry.error = "project registry path is empty";
        return registry;
    }

    QFile file(path);
    if (!file.exists()) {
        registry.error = "project registry not found: " + path;
        return registry;
    }
    if (!file.open(QIODevice::ReadOnly)) {
        registry.error = "project registry could not be opened: " + path;
        return registry;
    }

    QJsonParseError parseError;
    const QJsonDocument document = QJsonDocument::fromJson(file.readAll(), &parseError);
    if (parseError.error != QJsonParseError::NoError) {
        registry.error = "project registry JSON error: " + parseError.errorString();
        return registry;
    }
    registry = parseProjectRegistryDocument(document, QFileInfo(path).absoluteFilePath());
    return registry;
}

bool saveProjectRegistryFile(const ProjectRegistry &registry, QString *error) {
    if (registry.sourcePath.isEmpty()) {
        if (error) {
            *error = "project registry path is empty";
        }
        return false;
    }

    QJsonArray workers;
    for (const WorkerRegistryEntry &worker : registry.workers) {
        if (!worker.workerId.isEmpty()) {
            workers.push_back(workerToJson(worker));
        }
    }

    QJsonArray projects;
    for (const ProjectRegistryEntry &project : registry.projects) {
        if (!project.projectId.isEmpty()) {
            projects.push_back(projectToJson(project));
        }
    }

    QSaveFile file(registry.sourcePath);
    if (!file.open(QIODevice::WriteOnly)) {
        if (error) {
            *error = "project registry could not be opened for writing: " + registry.sourcePath;
        }
        return false;
    }
    const QJsonDocument document(QJsonObject{{"workers", workers}, {"projects", projects}});
    file.write(document.toJson(QJsonDocument::Indented));
    if (!file.commit()) {
        if (error) {
            *error = "project registry write failed: " + registry.sourcePath;
        }
        return false;
    }
    return true;
}

} // namespace DexProjects

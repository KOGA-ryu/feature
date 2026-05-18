#include "project_registry.h"

#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QJsonValue>

namespace DexProjects {
namespace {

QString readString(const QJsonObject &object, const QString &key, const QString &fallback = QString()) {
    const QJsonValue value = object.value(key);
    return value.isString() ? value.toString() : fallback;
}

bool readBool(const QJsonObject &object, const QString &key, bool fallback = false) {
    const QJsonValue value = object.value(key);
    return value.isBool() ? value.toBool() : fallback;
}

QStringList readStringList(const QJsonObject &object, const QString &key) {
    QStringList result;
    const QJsonValue value = object.value(key);
    if (!value.isArray()) {
        return result;
    }
    for (const QJsonValue &item : value.toArray()) {
        if (item.isString()) {
            result.push_back(item.toString());
        }
    }
    return result;
}

} // namespace

WorkerRegistryEntry parseWorkerRegistryEntry(const QJsonObject &object) {
    WorkerRegistryEntry entry;
    entry.workerId = readString(object, "worker_id");
    entry.role = readString(object, "role", entry.workerId);
    entry.displayName = readString(object, "display_name", entry.workerId);
    entry.status = readString(object, "status", "ready");
    entry.codexSessionId = readString(object, "codex_session_id");
    entry.codexModel = readString(object, "codex_model");
    entry.codexDirectory = readString(object, "codex_directory");
    entry.codexPermissions = readString(object, "codex_permissions");
    entry.codexContextWindow = readString(object, "codex_context_window");
    entry.codexAccount = readString(object, "codex_account");
    return entry;
}

ProjectRegistryEntry parseProjectRegistryEntry(const QJsonObject &object) {
    ProjectRegistryEntry entry;
    entry.projectId = readString(object, "project_id");
    entry.name = readString(object, "name", entry.projectId);
    entry.path = readString(object, "path");
    entry.role = readString(object, "role");
    entry.status = readString(object, "status");
    entry.authority = readString(object, "authority");
    entry.projectType = readString(object, "project_type");
    entry.pinned = readBool(object, "pinned");

    entry.safeEditZones = readStringList(object, "safe_edit_zones");
    entry.protectedZones = readStringList(object, "protected_zones");
    entry.generatedZones = readStringList(object, "generated_zones");
    entry.scratchZones = readStringList(object, "scratch_zones");
    entry.sourceDocs = readStringList(object, "source_docs");
    entry.buildCommands = readStringList(object, "build_commands");
    entry.testCommands = readStringList(object, "test_commands");
    entry.proofCommands = readStringList(object, "proof_commands");
    entry.workerIds = readStringList(object, "worker_ids");

    entry.editorCoreStatus = readString(object, "editor_core_status");
    entry.editorCorePath = readString(object, "editor_core_path");
    entry.fileInspectSupported = readBool(object, "file_inspect_supported");
    entry.fileEditSupported = readBool(object, "file_edit_supported");

    entry.binderTemplate = readString(object, "binder_template");
    const QJsonObject featureSummary = object.value("feature_pack_summary").toObject();
    if (!featureSummary.isEmpty()) {
        entry.featurePackTotal = featureSummary.value("total").toInt(0);
        const QJsonObject groups = featureSummary.value("groups").toObject();
        entry.featurePackLogic = groups.value("logic").toInt(0);
        entry.featurePackSim = groups.value("sim").toInt(0);
        entry.featurePackUi = groups.value("ui").toInt(0);
        entry.featurePackWorkflows = groups.value("workflows").toInt(0);
    }
    return entry;
}

ProjectRegistry parseProjectRegistryDocument(const QJsonDocument &document, const QString &sourcePath) {
    ProjectRegistry registry;
    registry.sourcePath = sourcePath;
    registry.loaded = true;

    QJsonArray projects;
    if (document.isArray()) {
        projects = document.array();
    } else if (document.isObject()) {
        const QJsonObject root = document.object();
        const QJsonArray workers = root.value("workers").toArray();
        for (const QJsonValue &value : workers) {
            if (!value.isObject()) {
                continue;
            }
            WorkerRegistryEntry entry = parseWorkerRegistryEntry(value.toObject());
            if (!entry.workerId.isEmpty()) {
                registry.workers.push_back(entry);
            }
        }
        projects = root.value("projects").toArray();
    }

    for (const QJsonValue &value : projects) {
        if (!value.isObject()) {
            continue;
        }
        ProjectRegistryEntry entry = parseProjectRegistryEntry(value.toObject());
        if (!entry.projectId.isEmpty()) {
            registry.projects.push_back(entry);
        }
    }
    return registry;
}

} // namespace DexProjects

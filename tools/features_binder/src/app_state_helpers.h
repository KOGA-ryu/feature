#pragma once

#include <QJsonArray>
#include <QJsonObject>
#include <QJsonValue>
#include <QString>
#include <QStringList>
#include <QVector>

#include <algorithm>

#include "app_state.h"

inline QString readString(const QJsonObject &object, const QString &key, const QString &fallback = QString()) {
    const QJsonValue value = object.value(key);
    return value.isString() ? value.toString() : fallback;
}

inline bool readBool(const QJsonObject &object, const QString &key, bool fallback = false) {
    const QJsonValue value = object.value(key);
    return value.isBool() ? value.toBool() : fallback;
}

inline QStringList readStringList(const QJsonObject &object, const QString &key) {
    QStringList result;
    const QJsonArray values = object.value(key).toArray();
    for (const QJsonValue &value : values) {
        if (value.isString()) {
            result.push_back(value.toString());
        }
    }
    return result;
}

inline DexProjects::ProjectRegistryEntry registryEntryFromProjectSummary(const ProjectSummary &project) {
    DexProjects::ProjectRegistryEntry entry;
    entry.projectId = project.id;
    entry.name = project.name;
    entry.path = project.path;
    entry.status = project.status;
    entry.role = "rust state project";
    entry.authority = "unknown";
    entry.projectType = "unknown";
    entry.pinned = project.pinned;
    entry.workerIds = project.workerIds;
    entry.editorCoreStatus = "registry_missing";
    entry.fileInspectSupported = false;
    entry.fileEditSupported = false;
    return entry;
}

inline QVector<DexProjects::ProjectRegistryEntry> registryProjectsForState(const CockpitState &state) {
    if (state.projectRegistryLoaded) {
        return state.registryProjects;
    }

    QVector<DexProjects::ProjectRegistryEntry> projects;
    for (const ProjectSummary &project : state.projects) {
        projects.push_back(registryEntryFromProjectSummary(project));
    }
    return projects;
}

inline const DexProjects::ProjectRegistryEntry *selectedRegistryProject(
    const QVector<DexProjects::ProjectRegistryEntry> &projects,
    const QString &selectedProjectId) {
    if (const auto *project = DexProjects::findProjectById(projects, selectedProjectId)) {
        return project;
    }
    if (!projects.isEmpty()) {
        return &projects.first();
    }
    return nullptr;
}

inline bool hasWorker(const CockpitState &state, const QString &workerId) {
    return std::any_of(state.workers.begin(), state.workers.end(), [&workerId](const WorkerSummary &worker) {
        return worker.id == workerId;
    });
}

inline QString workerDisplayName(const CockpitState &state, const QString &workerId) {
    if (workerId.isEmpty()) {
        return "No worker";
    }
    if (state.projectRegistryLoaded) {
        for (const DexProjects::WorkerRegistryEntry &worker : state.registryWorkers) {
            if (worker.workerId == workerId) {
                return worker.displayName.isEmpty() ? worker.workerId : worker.displayName;
            }
        }
    }
    for (const WorkerSummary &worker : state.workers) {
        if (worker.id == workerId) {
            return worker.displayName;
        }
    }
    return workerId;
}

inline WorkerSummary workerSummaryForId(const CockpitState &state, const QString &workerId) {
    if (workerId.isEmpty()) {
        return {"", "", "No worker selected", "none"};
    }
    if (state.projectRegistryLoaded) {
        for (const DexProjects::WorkerRegistryEntry &worker : state.registryWorkers) {
            if (worker.workerId == workerId) {
                return {
                    worker.workerId,
                    worker.role.isEmpty() ? worker.workerId : worker.role,
                    worker.displayName.isEmpty() ? worker.workerId : worker.displayName,
                    worker.status.isEmpty() ? QString("unknown") : worker.status,
                    worker.codexSessionId,
                    worker.codexModel,
                    worker.codexDirectory,
                    worker.codexPermissions,
                    worker.codexContextWindow,
                    worker.codexAccount,
                };
            }
        }
    }
    for (const WorkerSummary &worker : state.workers) {
        if (worker.id == workerId) {
            return worker;
        }
    }
    return {workerId, workerId, workerId, "unknown"};
}

inline WorkerSummary workerSummaryFromRegistry(
    const QVector<DexProjects::WorkerRegistryEntry> &workers,
    const QString &workerId) {
    for (const DexProjects::WorkerRegistryEntry &worker : workers) {
        if (worker.workerId == workerId) {
            return {
                worker.workerId,
                worker.role.isEmpty() ? worker.workerId : worker.role,
                worker.displayName.isEmpty() ? worker.workerId : worker.displayName,
                worker.status.isEmpty() ? QString("unknown") : worker.status,
                worker.codexSessionId,
                worker.codexModel,
                worker.codexDirectory,
                worker.codexPermissions,
                worker.codexContextWindow,
                worker.codexAccount,
            };
        }
    }
    return {workerId, workerId, workerId, "unknown"};
}

inline QVector<WorkerSummary> workersForProject(
    const CockpitState &state,
    const QString &selectedProjectId) {
    const QVector<DexProjects::ProjectRegistryEntry> projects = registryProjectsForState(state);
    const auto *project = DexProjects::findProjectById(projects, selectedProjectId);
    if (!project) {
        return {};
    }
    if (state.projectRegistryLoaded) {
        QVector<WorkerSummary> workers;
        for (const QString &workerId : project->workerIds) {
            if (!workerId.isEmpty()) {
                workers.push_back(workerSummaryFromRegistry(state.registryWorkers, workerId));
            }
        }
        return workers;
    }
    if (project->workerIds.isEmpty()) {
        return state.workers;
    }

    QVector<WorkerSummary> workers;
    for (const WorkerSummary &worker : state.workers) {
        if (project->workerIds.contains(worker.id)) {
            workers.push_back(worker);
        }
    }
    return workers;
}

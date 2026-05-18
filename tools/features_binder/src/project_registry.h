#pragma once

#include <QString>
#include <QStringList>
#include <QVector>

class QJsonDocument;
class QJsonObject;

namespace DexProjects {

struct WorkerRegistryEntry {
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

struct ProjectRegistryEntry {
    QString projectId;
    QString name;
    QString path;
    QString role;
    QString status;
    QString authority;
    QString projectType;
    bool pinned = false;

    QStringList safeEditZones;
    QStringList protectedZones;
    QStringList generatedZones;
    QStringList scratchZones;
    QStringList sourceDocs;
    QStringList buildCommands;
    QStringList testCommands;
    QStringList proofCommands;
    QStringList workerIds;

    QString editorCoreStatus;
    QString editorCorePath;
    bool fileInspectSupported = false;
    bool fileEditSupported = false;

    QString binderTemplate;
    int featurePackTotal = 0;
    int featurePackLogic = 0;
    int featurePackSim = 0;
    int featurePackUi = 0;
    int featurePackWorkflows = 0;
};

struct ProjectRegistry {
    QVector<ProjectRegistryEntry> projects;
    QVector<WorkerRegistryEntry> workers;
    QString sourcePath;
    bool loaded = false;
    QString error;
};

WorkerRegistryEntry parseWorkerRegistryEntry(const QJsonObject &object);
ProjectRegistryEntry parseProjectRegistryEntry(const QJsonObject &object);
ProjectRegistry parseProjectRegistryDocument(const QJsonDocument &document, const QString &sourcePath = QString());
ProjectRegistry loadProjectRegistryFile(const QString &path);
bool saveProjectRegistryFile(const ProjectRegistry &registry, QString *error = nullptr);
const ProjectRegistryEntry *findProjectById(const QVector<ProjectRegistryEntry> &projects, const QString &projectId);
QString displayName(const ProjectRegistryEntry &project);

} // namespace DexProjects

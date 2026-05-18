#include "project_registry.h"

namespace DexProjects {

const ProjectRegistryEntry *findProjectById(const QVector<ProjectRegistryEntry> &projects, const QString &projectId) {
    for (const ProjectRegistryEntry &project : projects) {
        if (project.projectId == projectId) {
            return &project;
        }
    }
    return nullptr;
}

QString displayName(const ProjectRegistryEntry &project) {
    return project.name.isEmpty() ? project.projectId : project.name;
}

} // namespace DexProjects

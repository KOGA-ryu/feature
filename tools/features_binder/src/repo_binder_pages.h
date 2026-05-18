#pragma once

#include <QString>

#include "app_state.h"
#include "project_registry.h"
#include "repo_binder_template.h"

class QWidget;

namespace DexBinderPages {

QWidget *buildRepoProfilePage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens,
    const DexRepoBinderTemplate::BinderTemplate &binderTemplate);
QWidget *buildRepoInventoryPage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens,
    const DexRepoBinderTemplate::BinderTemplate &binderTemplate);
QWidget *buildRepoMapPage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens);
QWidget *buildRepoAuthorityPage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens);
QWidget *buildRepoContractsPage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens);
QWidget *buildRepoActivityPage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens);
QWidget *buildRepoQualityPage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens,
    const DexRepoBinderTemplate::BinderTemplate &binderTemplate);
QWidget *buildRepoEvidencePage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens);
QWidget *buildRepoBlankPage(
    const CockpitState &state,
    const QString &topTab,
    const QString &detailLens);

} // namespace DexBinderPages

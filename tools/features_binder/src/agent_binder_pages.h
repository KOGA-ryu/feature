#pragma once

#include <QString>

#include "app_state.h"

class QWidget;

namespace DexBinderPages {

QWidget *buildProfilePage(const CockpitState &state, const QString &workerId, const QString &detailLens);
QWidget *buildStatsPage(const CockpitState &state, const QString &workerId, const QString &detailLens);
QWidget *buildRelationshipPage(const CockpitState &state, const QString &workerId, const QString &detailLens);
QWidget *buildGradePage(const CockpitState &state, const QString &workerId, const QString &detailLens);
QWidget *buildTranscriptPage(const CockpitState &state, const QString &workerId, const QString &detailLens);
QWidget *buildEvidencePage(const CockpitState &state, const QString &workerId, const QString &detailLens);

} // namespace DexBinderPages

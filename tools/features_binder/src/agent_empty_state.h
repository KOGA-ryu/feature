#pragma once

#include <QString>

#include "app_state.h"

class QVBoxLayout;

namespace DexBinderPages {

void addAgentEmptyStatePage(
    QVBoxLayout *layout,
    const CockpitState &state,
    const QString &workerId,
    const QString &subject,
    const QString &detailLens);

} // namespace DexBinderPages

namespace DexRightContext {

void addAgentEmptyStateContext(
    QVBoxLayout *contentLayout,
    const CockpitState &state,
    const QString &selectedWorkerId,
    const QString &selectedTopTab,
    const QString &selectedDetailLens);

} // namespace DexRightContext

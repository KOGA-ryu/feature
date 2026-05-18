#pragma once

#include <QString>

#include "app_state.h"

class QVBoxLayout;

namespace DexRightContext {

void addAgentContext(
    QVBoxLayout *contentLayout,
    const CockpitState &state,
    const QString &selectedWorkerId,
    const QString &selectedTopTab,
    const QString &selectedDetailLens);

} // namespace DexRightContext

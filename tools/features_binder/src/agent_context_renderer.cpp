#include "agent_context_renderer.h"

#include "agent_empty_state.h"

namespace DexRightContext {

void addAgentContext(
    QVBoxLayout *contentLayout,
    const CockpitState &state,
    const QString &selectedWorkerId,
    const QString &selectedTopTab,
    const QString &selectedDetailLens) {
    addAgentEmptyStateContext(contentLayout, state, selectedWorkerId, selectedTopTab, selectedDetailLens);
}

} // namespace DexRightContext

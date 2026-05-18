#include "agent_binder_pages.h"

#include <QVBoxLayout>

#include "agent_empty_state.h"
#include "binder_page_helpers.h"

namespace DexBinderPages {

QWidget *buildTranscriptPage(const CockpitState &state, const QString &workerId, const QString &detailLens) {
    return buildCompactPage([state, workerId, detailLens](QVBoxLayout *layout) {
        addAgentEmptyStatePage(layout, state, workerId, "Transcript", detailLens);
    });
}

} // namespace DexBinderPages

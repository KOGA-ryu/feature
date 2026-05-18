#include "repo_context_sections.h"

#include <QPushButton>
#include <QVBoxLayout>

#include "right_context_render_helpers.h"

namespace DexRightContext {

void addRepoContextActions(QVBoxLayout *contentLayout) {
    addContextSection(contentLayout, "quick actions");
    const QStringList actions = {
        "Open in editor",
        "Copy repo path",
        "Open inventory",
        "Run repo scan",
        "Mark boundary",
        "Attach proof"
    };
    for (const QString &action : actions) {
        const bool isEditorAction = action == "Open in editor";
        contentLayout->addWidget(makeContextActionButton(
            action,
            150,
            isEditorAction ? QString("Reserved for Project File Inspect V1") : action,
            !isEditorAction));
    }
    addWrappedLine(contentLayout, "Repo Binder records boundaries and receipts. It does not mutate files.");
}

} // namespace DexRightContext

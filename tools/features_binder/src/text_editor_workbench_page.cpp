#include "text_editor_workbench_page.h"

#include <QFrame>
#include <QScrollArea>
#include <QVBoxLayout>
#include <QWidget>

#include "text_editor_workspace_blank.h"

namespace DexTextEditorPages {

QWidget *buildTextEditorWorkbenchPage() {
    auto *page = new QWidget;
    page->setObjectName("textEditorWorkbenchPage");
    page->setProperty("uiPath", "workbench.text_editor");
    auto *layout = new QVBoxLayout(page);
    layout->setContentsMargins(0, 0, 0, 0);
    layout->setSpacing(0);

    auto *scroll = new QScrollArea;
    scroll->setObjectName("textEditorWorkspaceScroll");
    scroll->setProperty("uiPath", "workbench.editor.scroll");
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);

    scroll->setWidget(DexTextEditorWorkspace::buildBlankWorkspaceBody());
    layout->addWidget(scroll, 1);
    return page;
}

} // namespace DexTextEditorPages

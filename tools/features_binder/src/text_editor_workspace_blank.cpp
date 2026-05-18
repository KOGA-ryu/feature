#include "text_editor_workspace_blank.h"

#include <QFrame>
#include <QGridLayout>
#include <QHBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QSizePolicy>
#include <QStringList>
#include <QVBoxLayout>
#include <QWidget>

#include "render_helpers.h"
#include "ui_rules.h"

namespace {

void setUiPath(QWidget *widget, const QString &uiPath) {
    widget->setProperty("uiPath", uiPath);
}

QLabel *makeTextEditorLabel(const QString &text, const char *objectName, const QString &uiPath = QString()) {
    auto *label = makeLabel(text, objectName);
    label->setMinimumWidth(0);
    label->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Preferred);
    if (!uiPath.isEmpty()) {
        setUiPath(label, uiPath);
    }
    return label;
}

QPushButton *makeDisabledAction(const QString &label, const QString &uiPath) {
    auto *button = new QPushButton(label);
    button->setObjectName("textEditorActionButton");
    setUiPath(button, uiPath);
    button->setProperty("componentState", "disabled");
    button->setEnabled(false);
    button->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Fixed);
    button->setToolTip("Reserved Text Editor action slot. No document is open.");
    return button;
}

QFrame *makeActionStrip() {
    auto *strip = new QFrame;
    strip->setObjectName("textEditorActionStrip");
    setUiPath(strip, "workbench.toolbar.primary");
    strip->setMinimumHeight(dex_ui::text_editor_metrics::action_strip_height);

    auto *layout = new QGridLayout(strip);
    layout->setContentsMargins(10, 8, 10, 8);
    layout->setSpacing(7);
    layout->addWidget(makeDisabledAction("Commands", "workbench.toolbar.primary.commands"), 0, 0);
    layout->addWidget(makeDisabledAction("Run All", "workbench.toolbar.primary.run_all"), 0, 1);
    layout->addWidget(makeDisabledAction("Copy", "workbench.toolbar.primary.copy_plain"), 1, 0);
    layout->addWidget(makeDisabledAction("Prompt", "workbench.toolbar.primary.copy_prompt_block"), 1, 1);
    layout->addWidget(makeDisabledAction("Markdown", "workbench.toolbar.primary.copy_markdown_block"), 2, 0);
    layout->addWidget(makeDisabledAction("Fence", "workbench.toolbar.primary.copy_code_fence"), 2, 1);
    layout->addWidget(makeDisabledAction("Clean", "workbench.toolbar.primary.clean_basic"), 3, 0);
    layout->addWidget(makeDisabledAction("Settings", "workbench.toolbar.primary.settings"), 3, 1);
    for (int column = 0; column < 2; ++column) {
        layout->setColumnStretch(column, 1);
    }
    return strip;
}

QFrame *makeDocumentSurface() {
    auto *surface = new QFrame;
    surface->setObjectName("textEditorBlankSurface");
    setUiPath(surface, "workbench.editor.surface.document");
    surface->setMinimumHeight(360);

    auto *layout = new QVBoxLayout(surface);
    layout->setContentsMargins(0, 0, 0, 10);
    layout->setSpacing(0);

    auto *header = new QFrame;
    header->setObjectName("textEditorDocumentHeader");
    setUiPath(header, "workbench.editor.tabs");
    header->setFixedHeight(42);
    auto *headerLayout = new QHBoxLayout(header);
    headerLayout->setContentsMargins(0, 0, 0, 0);
    headerLayout->setSpacing(0);

    auto *tab = new QFrame;
    tab->setObjectName("textEditorDocumentTab");
    setUiPath(tab, "workbench.editor.tabs.empty_document");
    tab->setFixedWidth(150);
    auto *tabLayout = new QHBoxLayout(tab);
    tabLayout->setContentsMargins(18, 0, 12, 0);
    tabLayout->addWidget(makeTextEditorLabel("No document", "textEditorSurfaceTitle", "workbench.editor.tabs.empty_document.label"));
    headerLayout->addWidget(tab);
    headerLayout->addWidget(makeTextEditorLabel("+", "textEditorSurfaceTitle", "workbench.editor.tabs.add_document"));
    headerLayout->addStretch(1);
    layout->addWidget(header);

    auto *body = new QWidget;
    auto *bodyLayout = new QHBoxLayout(body);
    bodyLayout->setContentsMargins(14, 14, 14, 8);
    bodyLayout->setSpacing(12);
    auto *gutter = makeTextEditorLabel("1\n2\n3\n4\n5\n6", "textEditorGutterLabel", "workbench.editor.gutter.line_numbers");
    gutter->setAlignment(Qt::AlignRight | Qt::AlignTop);
    bodyLayout->addWidget(gutter);

    auto *textColumn = new QVBoxLayout;
    textColumn->setContentsMargins(0, 0, 0, 0);
    textColumn->setSpacing(10);
    auto *emptyState = makeTextEditorLabel(
        "No document open\n\nReserved for exact editor text.\nLoad a document, draft, clipboard capture, or fixture source to fill this surface.",
        "textEditorMonoLabel",
        "workbench.editor.surface.empty_state");
    emptyState->setWordWrap(true);
    textColumn->addWidget(emptyState);
    auto *selectionPreview = new QFrame;
    selectionPreview->setObjectName("textEditorSelectionPreview");
    setUiPath(selectionPreview, "workbench.editor.selection.preview_empty");
    selectionPreview->setFixedHeight(28);
    auto *emptyHint = makeTextEditorLabel("selection/output preview: empty", "textEditorSurfaceEmpty", "workbench.editor.selection.empty_hint");
    emptyHint->setWordWrap(true);
    textColumn->addWidget(selectionPreview);
    textColumn->addWidget(emptyHint);
    textColumn->addStretch(1);
    bodyLayout->addLayout(textColumn, 1);
    layout->addWidget(body, 1);

    layout->addWidget(makeTextEditorLabel("Ln 1, Col 1        UTF-8        LF", "textEditorMonoLabel", "workbench.editor.status.cursor_position"));
    return surface;
}

QFrame *makeFixturePanel(const QString &title, const QString &emptyText, const QString &uiPath) {
    auto *panel = new QFrame;
    panel->setObjectName("textEditorFixturePanel");
    setUiPath(panel, uiPath);
    auto *layout = new QVBoxLayout(panel);
    layout->setContentsMargins(12, 10, 12, 10);
    layout->setSpacing(8);
    layout->addWidget(makeTextEditorLabel(title, "textEditorSurfaceTitle", uiPath + ".title"));
    auto *empty = makeTextEditorLabel(emptyText, "textEditorSurfaceEmpty", uiPath + ".empty_state");
    empty->setWordWrap(true);
    layout->addWidget(empty);
    layout->addStretch(1);
    return panel;
}

QFrame *makeFixtureShelf() {
    auto *shelf = new QFrame;
    shelf->setObjectName("textEditorFixtureShelf");
    setUiPath(shelf, "workbench.fixture_bench");
    shelf->setMinimumHeight(dex_ui::text_editor_metrics::fixture_shelf_height);

    auto *layout = new QGridLayout(shelf);
    layout->setContentsMargins(10, 10, 10, 10);
    layout->setSpacing(8);
    layout->addWidget(makeFixturePanel("FIXTURE RUNNER", "No fixture selected\nNo fixture has run in this blank workspace.", "workbench.fixture_bench.list"), 0, 0, 1, 2);
    layout->addWidget(makeFixturePanel("EXPECTED", "No expected output yet.", "workbench.fixture_bench.results.expected"), 1, 0);
    layout->addWidget(makeFixturePanel("ACTUAL", "No actual output yet.", "workbench.fixture_bench.results.actual"), 1, 1);
    layout->setColumnStretch(0, 1);
    layout->setColumnStretch(1, 1);
    return shelf;
}

QFrame *makeContextPanel(const QString &title, const QStringList &lines, const QString &uiPath) {
    auto *panel = new QFrame;
    panel->setObjectName("textEditorContextPanel");
    setUiPath(panel, uiPath);
    auto *layout = new QVBoxLayout(panel);
    layout->setContentsMargins(12, 10, 12, 10);
    layout->setSpacing(7);
    layout->addWidget(makeTextEditorLabel(title, "sectionLabel", uiPath + ".title"));
    for (const QString &line : lines) {
        auto *label = makeTextEditorLabel(line, "textEditorSurfaceEmpty", uiPath + ".line");
        label->setWordWrap(true);
        layout->addWidget(label);
    }
    layout->addStretch(1);
    return panel;
}

} // namespace

namespace DexTextEditorWorkspace {

QWidget *buildBlankWorkspaceBody() {
    auto *body = new QWidget;
    body->setObjectName("textEditorBlankBody");
    setUiPath(body, "workbench.editor.workspace");
    auto *bodyLayout = new QVBoxLayout(body);
    bodyLayout->setContentsMargins(12, 12, 12, 12);
    bodyLayout->setSpacing(10);

    bodyLayout->addWidget(makeActionStrip());
    bodyLayout->addWidget(makeDocumentSurface(), 1);
    bodyLayout->addWidget(makeFixtureShelf());
    return body;
}

void addBlankWorkspaceContext(QVBoxLayout *layout, const QString &detailLens) {
    layout->addWidget(makeContextPanel(
        "INSPECTOR",
        {
            "No action selected",
            "Choose a toolbar, palette, menu, or fixture action to show parameters here.",
        },
        "workbench.inspector.text_editor.options"));
    layout->addWidget(makeContextPanel(
        "TEXT EDITOR CONTEXT",
        {
            "state: blank workbench shell",
            "detail lens: " + detailLens,
            "active document: none",
            "system clipboard: not touched",
            "fixtures: not running",
        },
        "workbench.inspector.text_editor.context"));
    layout->addWidget(makeContextPanel(
        "RECEIPT LOG",
        {
            "No action receipts yet.",
            "Receipts appear only after a real action runs.",
        },
        "workbench.inspector.text_editor.receipts"));
}

} // namespace DexTextEditorWorkspace

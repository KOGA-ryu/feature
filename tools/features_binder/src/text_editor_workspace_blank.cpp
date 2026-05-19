#include "text_editor_workspace_blank.h"

#include <QCheckBox>
#include <QClipboard>
#include <QComboBox>
#include <QFrame>
#include <QGridLayout>
#include <QGuiApplication>
#include <QHBoxLayout>
#include <QLabel>
#include <QLineEdit>
#include <QPlainTextEdit>
#include <QPushButton>
#include <QResizeEvent>
#include <QScrollArea>
#include <QSignalBlocker>
#include <QSizePolicy>
#include <QSpinBox>
#include <QStringList>
#include <QTextCursor>
#include <QTextDocument>
#include <QTimer>
#include <QVBoxLayout>
#include <QWidget>

#include <algorithm>

#include "render_helpers.h"
#include "text_action_proof_model.h"
#include "text_editor_rust_action_client.h"
#include "text_editor_workspace_state.h"
#include "ui_rules.h"

namespace {

void setUiPath(QWidget *widget, const QString &uiPath) {
    widget->setProperty("uiPath", uiPath);
}

void setComponentState(QWidget *widget, const QString &state) {
    widget->setProperty("componentState", state);
    widget->style()->unpolish(widget);
    widget->style()->polish(widget);
}

QString actionPathSuffix(const QString &actionId) {
    return actionId.startsWith("text.") ? actionId.mid(5) : actionId;
}

bool isPrimaryToolbarAction(const QString &actionId) {
    static const QStringList primaryActions = {
        "text.copy_plain",
        "text.copy_prompt_block",
        "text.copy_markdown_block",
        "text.copy_code_fence",
        "text.clean_basic",
        "text.select_all",
    };
    return primaryActions.contains(actionId);
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

QLineEdit *makeTextEditorLineEdit(const QString &value, const QString &uiPath) {
    auto *edit = new QLineEdit;
    edit->setText(value);
    edit->setMinimumHeight(28);
    edit->setMinimumWidth(0);
    edit->setProperty("uiPath", uiPath);
    edit->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Fixed);
    return edit;
}

QFrame *makePanel(const QString &objectName, const QString &uiPath) {
    auto *panel = new QFrame;
    panel->setObjectName(objectName);
    panel->setMinimumWidth(0);
    setUiPath(panel, uiPath);
    return panel;
}

void clearTextEditorLayout(QLayout *layout) {
    while (QLayoutItem *item = layout->takeAt(0)) {
        if (QWidget *widget = item->widget()) {
            widget->deleteLater();
        }
        if (QLayout *child = item->layout()) {
            clearTextEditorLayout(child);
        }
        delete item;
    }
}

QString receiptText(const DexTextActions::ClipboardReceiptSummary &receipt) {
    QStringList lines;
    lines << QString("changes: %1").arg(receipt.changeCount);
    for (const QString &change : receipt.changes) {
        lines << "  " + change;
    }
    lines << QString("warnings: %1").arg(receipt.warningCount);
    for (const QString &warning : receipt.warnings) {
        lines << "  " + warning;
    }
    return lines.join('\n');
}

QString receiptText(const DexTextEditorRust::ReceiptSummary &receipt) {
    QStringList lines;
    lines << QString("changes: %1").arg(receipt.changeCount);
    for (const QString &change : receipt.changes) {
        lines << "  " + change;
    }
    lines << QString("warnings: %1").arg(receipt.warningCount);
    for (const QString &warning : receipt.warnings) {
        lines << "  " + warning;
    }
    return lines.join('\n');
}

QString resultReceiptText(const DexTextEditorRust::ActionResult &result) {
    QStringList sections{receiptText(result.receipt)};
    const QString metadata = DexTextEditorRust::payloadMetadataSummary(result.payloadMetadata);
    if (!metadata.isEmpty()) {
        sections << metadata;
    }
    return sections.join("\n\n");
}

QString fixtureSummaryText(const DexTextActions::TextActionFixtureSuiteResult &suite) {
    QStringList lines;
    lines << suite.summary;
    for (const DexTextActions::TextActionFixtureResult &result : suite.results) {
        lines << result.summary;
    }
    return lines.join('\n');
}

QString resultStateForKind(const QString &kind, bool ok = true) {
    if (!ok) {
        return "danger";
    }
    if (kind == "disabled") {
        return "disabled";
    }
    if (kind == "none") {
        return "empty";
    }
    return "success";
}

QString lineSummary(int lines, int characters) {
    return QString("%1 lines | %2 chars").arg(std::max(1, lines)).arg(std::max(0, characters));
}

QString clippedPreviewText(const QString &text, int limit) {
    if (text.size() <= limit) {
        return text;
    }
    return text.left(limit)
        + QString("\n\n[preview clipped: %1 of %2 chars shown]").arg(limit).arg(text.size());
}

void setPreviewText(QPlainTextEdit *edit, const QString &text, int limit) {
    if (edit) {
        edit->setPlainText(clippedPreviewText(text, limit));
    }
}

QSpinBox *makeTextEditorSpinBox(int value, int maximum, const QString &uiPath) {
    auto *spin = new QSpinBox;
    spin->setMinimum(0);
    spin->setMaximum(std::max(0, maximum));
    spin->setValue(std::max(0, std::min(value, spin->maximum())));
    spin->setMinimumHeight(28);
    spin->setProperty("uiPath", uiPath);
    spin->setProperty("componentState", "default");
    return spin;
}

QCheckBox *makeTextEditorCheckBox(const QString &label, bool checked, const QString &uiPath) {
    auto *check = new QCheckBox(label);
    check->setChecked(checked);
    check->setProperty("uiPath", uiPath);
    check->setProperty("componentState", checked ? "active" : "default");
    return check;
}

using DexTextEditorWorkspace::TextEditorWorkspaceController;
using DexTextEditorWorkspace::TextEditorWorkspaceState;

class TextEditorWorkspaceBody final : public QWidget {
public:
    explicit TextEditorWorkspaceBody(TextEditorWorkspaceController *controller, QWidget *parent = nullptr)
        : QWidget(parent),
          controller_(controller ? controller : new TextEditorWorkspaceController(this)) {
        setObjectName("textEditorBlankBody");
        setUiPath(this, "workbench.editor.workspace");
        setMinimumWidth(0);
        setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Expanding);

        auto *bodyLayout = new QVBoxLayout(this);
        bodyLayout->setContentsMargins(
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding);
        bodyLayout->setSpacing(dex_ui::text_editor_metrics::region_gap);

        bodyLayout->addWidget(buildActionStrip());
        bodyLayout->addWidget(buildDocumentSurface(), 1);
        bodyLayout->addWidget(buildFixtureShelf());

        connect(editor_, &QPlainTextEdit::textChanged, this, [this]() {
            refreshLineControls();
            rebuildActionButtons();
            updateEditorStatus();
        });
        connect(editor_, &QPlainTextEdit::cursorPositionChanged, this, [this]() {
            updateEditorStatus();
            rebuildActionButtons();
        });
        connect(controller_, &TextEditorWorkspaceController::actionInputChanged, this, [this]() {
            refreshLineControls();
            rebuildActionButtons();
        });

        controller_->setActionInventory(
            DexTextActions::textActionRecords().size(),
            DexTextActions::textActionFixtures().size());
        loadSelectedFixture();
        runAllFixtures();
        updateEditorStatus();
    }

    QSize minimumSizeHint() const override {
        const QSize hint = QWidget::minimumSizeHint();
        return QSize(0, hint.height());
    }

    QSize sizeHint() const override {
        const QSize hint = QWidget::sizeHint();
        return QSize(0, hint.height());
    }

private:
    void resizeEvent(QResizeEvent *event) override {
        QWidget::resizeEvent(event);
        const int columns = desiredActionColumns();
        if (columns != actionColumnCount_) {
            rebuildActionButtons();
        }
    }

    QFrame *buildActionStrip() {
        auto *strip = makePanel("textEditorActionStrip", "workbench.toolbar.primary");
        actionStrip_ = strip;
        strip->setFixedHeight(dex_ui::text_editor_metrics::action_strip_height);
        setComponentState(strip, "default");

        actionsLayout_ = new QGridLayout(strip);
        actionsLayout_->setContentsMargins(
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding_dense,
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding_dense);
        actionsLayout_->setHorizontalSpacing(dex_ui::text_editor_metrics::region_gap);
        actionsLayout_->setVerticalSpacing(dex_ui::text_editor_metrics::dense_gap);
        return strip;
    }

    QFrame *buildDocumentSurface() {
        auto *surface = makePanel("textEditorBlankSurface", "workbench.editor.surface.document");
        surface->setMinimumHeight(dex_ui::text_editor_metrics::document_min_height);
        setComponentState(surface, "default");

        auto *layout = new QVBoxLayout(surface);
        layout->setContentsMargins(0, 0, 0, dex_ui::text_editor_metrics::panel_padding_dense);
        layout->setSpacing(0);

        auto *header = makePanel("textEditorDocumentHeader", "workbench.editor.tabs");
        header->setFixedHeight(dex_ui::text_editor_metrics::document_tab_height);
        auto *headerLayout = new QHBoxLayout(header);
        headerLayout->setContentsMargins(0, 0, 0, 0);
        headerLayout->setSpacing(0);

        auto *tab = makePanel("textEditorDocumentTab", "workbench.editor.tabs.active_document");
        tab->setFixedWidth(176);
        setComponentState(tab, "active");
        auto *tabLayout = new QHBoxLayout(tab);
        tabLayout->setContentsMargins(
            dex_ui::text_editor_metrics::section_gap,
            0,
            dex_ui::text_editor_metrics::panel_padding,
            0);
        tabLayout->addWidget(makeTextEditorLabel("scratch.txt", "textEditorSurfaceTitle", "workbench.editor.tabs.active_document.label"));
        headerLayout->addWidget(tab);
        editorStatus_ = makeTextEditorLabel("ready", "textEditorSurfaceEmpty", "workbench.editor.status.summary");
        headerLayout->addWidget(editorStatus_, 1);
        layout->addWidget(header);

        editor_ = new QPlainTextEdit;
        editor_->setObjectName("textEditorDocumentEditor");
        editor_->setProperty("uiPath", "workbench.editor.surface.text");
        editor_->setProperty("componentState", "focused");
        editor_->setMinimumHeight(280);
        editor_->setPlaceholderText("Write or paste text here, select a range, then run a text action.");
        layout->addWidget(editor_, 1);

        cursorStatus_ = makeTextEditorLabel("Ln 1, Col 1        UTF-8        LF        selection: none",
            "textEditorMonoLabel",
            "workbench.editor.status.cursor_position");
        cursorStatus_->setProperty("componentState", "empty");
        cursorStatus_->setContentsMargins(
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding_dense,
            dex_ui::text_editor_metrics::panel_padding,
            0);
        layout->addWidget(cursorStatus_);
        return surface;
    }

    QFrame *buildFixtureShelf() {
        auto *shelf = makePanel("textEditorFixtureShelf", "workbench.fixture_bench");
        shelf->setMinimumHeight(dex_ui::text_editor_metrics::fixture_shelf_height);
        shelf->setMinimumWidth(0);
        shelf->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Preferred);
        setComponentState(shelf, "empty");

        auto *layout = new QVBoxLayout(shelf);
        layout->setContentsMargins(
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding);
        layout->setSpacing(dex_ui::text_editor_metrics::region_gap);

        auto *runner = makePanel("textEditorFixturePanel", "workbench.fixture_bench.runner");
        runner->setMinimumWidth(0);
        setComponentState(runner, "empty");
        auto *runnerLayout = new QVBoxLayout(runner);
        runnerLayout->setContentsMargins(
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding_dense,
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding_dense);
        runnerLayout->setSpacing(dex_ui::text_editor_metrics::region_gap);
        runnerLayout->addWidget(makeTextEditorLabel("FIXTURE RUNNER", "textEditorSurfaceTitle", "workbench.fixture_bench.runner.title"));

        auto *fixtureRow = new QWidget;
        auto *fixtureRowLayout = new QHBoxLayout(fixtureRow);
        fixtureRowLayout->setContentsMargins(0, 0, 0, 0);
        fixtureRowLayout->setSpacing(dex_ui::text_editor_metrics::region_gap);
        fixturePicker_ = new QComboBox;
        fixturePicker_->setMinimumWidth(0);
        fixturePicker_->setSizePolicy(QSizePolicy::Ignored, QSizePolicy::Fixed);
        for (const DexTextActions::TextActionFixture &fixture : DexTextActions::textActionFixtures()) {
            fixturePicker_->addItem(fixture.label, fixture.fixtureId);
        }
        auto *loadFixture = makeActionButton("Load Fixture", "workbench.fixture_bench.runner.load");
        auto *runFixture = makeActionButton("Run Fixture", "workbench.fixture_bench.runner.run");
        auto *runAll = makeActionButton("Run All", "workbench.fixture_bench.runner.run_all");
        fixtureRowLayout->addWidget(fixturePicker_, 1);
        runnerLayout->addWidget(fixtureRow);

        auto *fixtureButtonRow = new QWidget;
        auto *fixtureButtonLayout = new QVBoxLayout(fixtureButtonRow);
        fixtureButtonLayout->setContentsMargins(0, 0, 0, 0);
        fixtureButtonLayout->setSpacing(dex_ui::text_editor_metrics::region_gap);
        fixtureButtonLayout->addWidget(loadFixture);
        fixtureButtonLayout->addWidget(runFixture);
        fixtureButtonLayout->addWidget(runAll);
        runnerLayout->addWidget(fixtureButtonRow);

        fixtureStatus_ = makeTextEditorLabel("No fixture run yet.", "textEditorSurfaceEmpty", "workbench.fixture_bench.runner.status");
        fixtureStatus_->setWordWrap(true);
        runnerLayout->addWidget(fixtureStatus_);
        layout->addWidget(runner);

        expected_ = makeOutputBox("expected output", "workbench.fixture_bench.results.expected");
        actual_ = makeOutputBox("actual output", "workbench.fixture_bench.results.actual");
        auto *resultRow = new QWidget;
        resultRow->setMinimumWidth(0);
        resultRow->setProperty("uiPath", "workbench.fixture_bench.results");
        auto *resultLayout = new QHBoxLayout(resultRow);
        resultLayout->setContentsMargins(0, 0, 0, 0);
        resultLayout->setSpacing(dex_ui::text_editor_metrics::region_gap);
        resultLayout->addWidget(makeOutputPanel("EXPECTED", expected_, "workbench.fixture_bench.results.expected_panel"), 1);
        resultLayout->addWidget(makeOutputPanel("ACTUAL / ACTION OUTPUT", actual_, "workbench.fixture_bench.results.actual_panel"), 1);
        layout->addWidget(resultRow, 1);

        connect(loadFixture, &QPushButton::clicked, this, [this]() {
            loadSelectedFixture();
        });
        connect(runFixture, &QPushButton::clicked, this, [this]() {
            runSelectedFixture();
        });
        connect(runAll, &QPushButton::clicked, this, [this]() {
            runAllFixtures();
        });
        return shelf;
    }

    QPushButton *makeActionButton(const QString &label, const QString &uiPath) const {
        auto *button = new QPushButton(label);
        button->setObjectName("textEditorActionButton");
        button->setProperty("uiPath", uiPath);
        button->setProperty("componentState", "default");
        button->setMinimumWidth(0);
        button->setFixedHeight(dex_ui::text_editor_metrics::toolbar_button_height);
        button->setSizePolicy(QSizePolicy::Ignored, QSizePolicy::Fixed);
        return button;
    }

    QPlainTextEdit *makeOutputBox(const QString &placeholder, const QString &uiPath) const {
        auto *box = new QPlainTextEdit;
        box->setObjectName("textEditorOutputPreview");
        box->setProperty("uiPath", uiPath);
        box->setProperty("componentState", "empty");
        box->setReadOnly(true);
        box->setMinimumWidth(0);
        box->setSizePolicy(QSizePolicy::Ignored, QSizePolicy::Preferred);
        box->setMinimumHeight(dex_ui::text_editor_metrics::fixture_result_min_height);
        box->setPlaceholderText(placeholder);
        return box;
    }

    QFrame *makeOutputPanel(const QString &title, QPlainTextEdit *output, const QString &uiPath) const {
        auto *panel = makePanel("textEditorFixturePanel", uiPath);
        panel->setMinimumWidth(0);
        panel->setSizePolicy(QSizePolicy::Ignored, QSizePolicy::Preferred);
        setComponentState(panel, "empty");
        auto *layout = new QVBoxLayout(panel);
        layout->setContentsMargins(
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding_dense,
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding_dense);
        layout->setSpacing(dex_ui::text_editor_metrics::region_gap);
        layout->addWidget(makeTextEditorLabel(title, "textEditorSurfaceTitle", uiPath + ".title"));
        layout->addWidget(output, 1);
        return panel;
    }

    void rebuildActionButtons() {
        if (!actionsLayout_) {
            return;
        }
        clearTextEditorLayout(actionsLayout_);
        QString actionError;
        QVector<DexTextActions::HostActionItem> actions =
            DexTextEditorRust::renderActionsWithRunner(editor_->toPlainText(), currentInput(), &actionError);
        actionInventorySource_ = actions.isEmpty() ? "C++ fixture fallback" : "Rust action runner";
        if (actions.isEmpty()) {
            actions = DexTextActions::renderHostActionItems(editor_->toPlainText(), currentInput());
        }
        controller_->setActionInventory(actions.size(), DexTextActions::textActionFixtures().size());
        QVector<DexTextActions::HostActionItem> toolbarActions;
        for (const DexTextActions::HostActionItem &action : actions) {
            if (isPrimaryToolbarAction(action.actionId)) {
                toolbarActions.append(action);
            }
        }
        int row = 0;
        int column = 0;
        const int kColumns = desiredActionColumns();
        actionColumnCount_ = kColumns;
        if (actionStrip_) {
            actionStrip_->setFixedHeight(actionStripHeight(kColumns, toolbarActions.size()));
        }
        for (const DexTextActions::HostActionItem &action : toolbarActions) {
            auto *button = makeActionButton(action.shortLabel, "workbench.toolbar.primary." + actionPathSuffix(action.actionId));
            const bool isRunning = action.actionId == activeActionId_ && activeActionState_ == "running";
            const QString buttonState = action.enabled
                ? (action.actionId == activeActionId_ ? activeActionState_ : QString("default"))
                : QString("disabled");
            button->setEnabled(action.enabled && !isRunning);
            button->setProperty("componentState", buttonState);
            button->setToolTip(action.hotkeyLabel.isEmpty()
                ? action.tooltip + (action.disabledReason.isEmpty() ? QString() : "\n" + action.disabledReason)
                : action.tooltip + "\n" + action.hotkeyLabel);
            connect(button, &QPushButton::clicked, this, [this, action]() {
                executeAction(action.actionId);
            });
            actionsLayout_->addWidget(button, row, column);
            if (++column == kColumns) {
                column = 0;
                ++row;
            }
        }
        for (int i = 0; i < kColumns; ++i) {
            actionsLayout_->setColumnStretch(i, 1);
        }
    }

    int desiredActionColumns() const {
        const int available = actionStrip_ ? actionStrip_->width() : width();
        if (available < 240) {
            return 1;
        }
        if (available < 520) {
            return 2;
        }
        return 3;
    }

    int actionStripHeight(int columns, int actionCount) const {
        columns = std::max(1, columns);
        const int rows = std::max(1, (actionCount + columns - 1) / columns);
        return (dex_ui::text_editor_metrics::panel_padding_dense * 2)
            + (rows * dex_ui::text_editor_metrics::toolbar_button_height)
            + ((rows - 1) * dex_ui::text_editor_metrics::dense_gap);
    }

    void executeAction(const QString &actionId) {
        const QString documentText = editor_->toPlainText();
        if (documentText.size() > dex_ui::text_editor_content_limits::max_document_chars) {
            const QString message = QString("Document is over action limit: %1 / %2 chars")
                .arg(documentText.size())
                .arg(dex_ui::text_editor_content_limits::max_document_chars);
            setPreviewText(expected_, "content limit", dex_ui::text_editor_content_limits::max_receipt_preview_chars);
            setPreviewText(actual_, message, dex_ui::text_editor_content_limits::max_output_preview_chars);
            fixtureStatus_->setText(message);
            setComponentState(fixtureStatus_, "danger");
            controller_->setLastResult(actionId, "content_limit", message, message, "danger");
            return;
        }

        activeActionId_ = actionId;
        activeActionState_ = "running";
        rebuildActionButtons();

        DexTextEditorRust::ActionRequest request;
        request.actionId = actionId;
        request.documentText = documentText;
        request.selection = currentSelection();
        request.input = currentInput();

        const DexTextEditorRust::ActionResult result =
            DexTextEditorRust::executeActionWithRunner(request);
        if (result.selection.valid) {
            applySelection(result.selection);
        }
        renderRustResult(actionId, result);
        rebuildActionButtons();
        updateEditorStatus();
    }

    void loadSelectedFixture() {
        const QString fixtureId = fixturePicker_->currentData().toString();
        for (const DexTextActions::TextActionFixture &fixture : DexTextActions::textActionFixtures()) {
            if (fixture.fixtureId != fixtureId) {
                continue;
            }
            {
                const QSignalBlocker blockEditor(editor_);
                editor_->setPlainText(fixture.documentText);
            }
            controller_->setActionInput(
                fixture.input.language.isEmpty() ? QString("text") : fixture.input.language,
                fixture.input.source,
                std::max(0, fixture.input.startLine),
                std::max(0, fixture.input.endLine),
                fixture.input.stripAnsiEscapeCodes);
            setPreviewText(expected_, fixture.expectedClipboardText, dex_ui::text_editor_content_limits::max_output_preview_chars);
            actual_->clear();
            fixtureStatus_->setText("Fixture loaded: " + fixture.fixtureId);
            setComponentState(fixtureStatus_, "empty");
            controller_->setFixtureStatus("Fixture loaded: " + fixture.fixtureId, "empty");
            refreshLineControls();
            rebuildActionButtons();
            updateEditorStatus();
            return;
        }
    }

    void runSelectedFixture() {
        renderFixtureResult(DexTextActions::runTextActionFixture(fixturePicker_->currentData().toString()));
    }

    void runAllFixtures() {
        renderFixtureSuiteResult(DexTextActions::runAllTextActionFixtures());
    }

    DexTextActions::TextActionProofInput currentInput() const {
        DexTextActions::TextActionProofInput input;
        const TextEditorWorkspaceState &state = controller_->state();
        input.language = state.language;
        input.source = state.source;
        input.currentLine = editor_->textCursor().blockNumber();
        input.startLine = state.startLine;
        input.endLine = state.endLine;
        input.stripAnsiEscapeCodes = state.stripAnsiEscapeCodes;
        return input;
    }

    QString selectedText() const {
        QString selected = editor_->textCursor().selectedText();
        selected.replace(QChar(0x2029), '\n');
        return selected;
    }

    DexTextEditorRust::Position positionFromOffset(int offset) const {
        const QString prefix = editor_->toPlainText().left(std::max(0, offset));
        const int line = prefix.count('\n');
        const int lastBreak = prefix.lastIndexOf('\n');
        const QString columnText = lastBreak >= 0 ? prefix.mid(lastBreak + 1) : prefix;
        return DexTextEditorRust::Position{line, static_cast<int>(columnText.toUcs4().size())};
    }

    DexTextEditorRust::Selection currentSelection() const {
        const QTextCursor cursor = editor_->textCursor();
        DexTextEditorRust::Selection selection;
        if (cursor.hasSelection()) {
            selection.anchor = positionFromOffset(cursor.selectionStart());
            selection.caret = positionFromOffset(cursor.selectionEnd());
        } else {
            selection.anchor = positionFromOffset(cursor.position());
            selection.caret = selection.anchor;
        }
        selection.valid = true;
        return selection;
    }

    int offsetFromPosition(const DexTextEditorRust::Position &position) const {
        const QString text = editor_->toPlainText();
        int line = 0;
        int column = 0;
        for (int i = 0; i < text.size();) {
            if (line == position.line && column == position.column) {
                return i;
            }

            const QChar character = text.at(i);
            const bool surrogatePair = character.isHighSurrogate()
                && i + 1 < text.size()
                && text.at(i + 1).isLowSurrogate();
            const int step = surrogatePair ? 2 : 1;
            if (character == QChar('\n')) {
                ++line;
                column = 0;
            } else {
                ++column;
            }
            i += step;
        }
        return text.size();
    }

    void applySelection(const DexTextEditorRust::Selection &selection) {
        QTextCursor cursor(editor_->document());
        cursor.setPosition(offsetFromPosition(selection.anchor));
        cursor.setPosition(offsetFromPosition(selection.caret), QTextCursor::KeepAnchor);
        editor_->setTextCursor(cursor);
    }

    void renderRustResult(const QString &requestedActionId, const DexTextEditorRust::ActionResult &result) {
        const QString resultActionId = result.actionId.isEmpty() ? requestedActionId : result.actionId;
        if (!result.ok) {
            setPreviewText(expected_, "runner error", dex_ui::text_editor_content_limits::max_receipt_preview_chars);
            setPreviewText(actual_, result.error, dex_ui::text_editor_content_limits::max_output_preview_chars);
            fixtureStatus_->setText(result.displayText.isEmpty() ? result.error : result.displayText);
            setComponentState(fixtureStatus_, "danger");
            activeActionId_ = requestedActionId;
            activeActionState_ = "danger";
            controller_->setLastResult(
                resultActionId,
                "runner_error",
                result.displayText.isEmpty() ? result.error : result.displayText,
                result.error,
                "danger");
            return;
        }

        const bool shouldWriteClipboard = result.hasClipboardText
            && result.kind != "disabled"
            && result.kind != "none"
            && result.clipboardText.size() <= dex_ui::text_editor_content_limits::max_clipboard_chars;
        if (shouldWriteClipboard) {
            QGuiApplication::clipboard()->setText(result.clipboardText);
        }

        const QString outputText = result.hasClipboardText ? result.clipboardText : result.displayText;
        const QString receiptSummary = resultReceiptText(result);
        setPreviewText(actual_, outputText, dex_ui::text_editor_content_limits::max_output_preview_chars);
        setPreviewText(expected_, receiptSummary, dex_ui::text_editor_content_limits::max_receipt_preview_chars);
        const QString clipboardNote = result.hasClipboardText
            && result.clipboardText.size() > dex_ui::text_editor_content_limits::max_clipboard_chars
            ? " | clipboard skipped: over limit"
            : (shouldWriteClipboard ? " | copied to clipboard" : "");
        fixtureStatus_->setText(resultActionId + " | " + result.kind + " | "
            + result.displayText
            + clipboardNote);
        const QString state = resultStateForKind(result.kind, true);
        activeActionId_ = resultActionId;
        activeActionState_ = state;
        setComponentState(fixtureStatus_, state);
        controller_->setLastResult(
            resultActionId,
            result.kind,
            result.displayText + clipboardNote,
            receiptSummary,
            state);
        QTimer::singleShot(1200, this, [this, resultActionId]() {
            if (activeActionId_ == resultActionId && activeActionState_ != "running") {
                activeActionId_.clear();
                activeActionState_ = "default";
                rebuildActionButtons();
            }
        });
    }

    void renderResult(const DexTextActions::HostActionResult &result) {
        setPreviewText(actual_, result.clipboardText.isEmpty() ? result.displayText : result.clipboardText, dex_ui::text_editor_content_limits::max_output_preview_chars);
        setPreviewText(expected_, receiptText(result.receipt), dex_ui::text_editor_content_limits::max_receipt_preview_chars);
        fixtureStatus_->setText(result.actionId + " | " + result.kind + " | " + result.displayText);
        const QString state = resultStateForKind(result.kind, true);
        setComponentState(fixtureStatus_, state);
        controller_->setLastResult(result.actionId, result.kind, result.displayText, receiptText(result.receipt), state);
    }

    void renderFixtureResult(const DexTextActions::TextActionFixtureResult &result) {
        setPreviewText(expected_, result.expectedClipboardText, dex_ui::text_editor_content_limits::max_output_preview_chars);
        setPreviewText(actual_, result.actualClipboardText, dex_ui::text_editor_content_limits::max_output_preview_chars);
        fixtureStatus_->setText(result.summary);
        const QString state = result.passed ? QString("success") : QString("danger");
        setComponentState(fixtureStatus_, state);
        controller_->setFixtureStatus(result.summary, state);
        controller_->setLastResult(
            result.fixture.actionId,
            "fixture",
            result.summary,
            receiptText(result.actionResult.receipt),
            state);
    }

    void renderFixtureSuiteResult(const DexTextActions::TextActionFixtureSuiteResult &suite) {
        setPreviewText(expected_, "fixture suite", dex_ui::text_editor_content_limits::max_output_preview_chars);
        setPreviewText(actual_, fixtureSummaryText(suite), dex_ui::text_editor_content_limits::max_output_preview_chars);
        fixtureStatus_->setText(suite.summary);
        const QString state = suite.allPassed ? QString("success") : QString("danger");
        setComponentState(fixtureStatus_, state);
        controller_->setFixtureStatus(suite.summary, state);
        controller_->setLastResult("fixtures.run_all", "fixture_suite", suite.summary, fixtureSummaryText(suite), state);
    }

    void refreshLineControls() {
        const int maxLine = std::max(0, editor_->document()->blockCount() - 1);
        const TextEditorWorkspaceState &state = controller_->state();
        if (state.startLine > maxLine || state.endLine > maxLine) {
            controller_->setActionInput(
                state.language,
                state.source,
                std::min(state.startLine, maxLine),
                std::min(state.endLine, maxLine),
                state.stripAnsiEscapeCodes);
        }
    }

    void updateEditorStatus() {
        const QTextCursor cursor = editor_->textCursor();
        const QString selectionText = selectedText();
        cursorStatus_->setText(QString("Ln %1, Col %2        UTF-8        LF        selection: %3")
                                   .arg(cursor.blockNumber() + 1)
                                   .arg(cursor.positionInBlock() + 1)
                                   .arg(selectionText.isEmpty() ? QString("none") : QString("%1 chars").arg(selectionText.size())));
        setComponentState(cursorStatus_, selectionText.isEmpty() ? QString("empty") : QString("active"));
        editorStatus_->setText(QString("actions: %1 | fixtures: %2")
                                   .arg(controller_->state().actionCount)
                                   .arg(DexTextActions::textActionFixtures().size()));
        controller_->setDocumentFacts(
            "scratch.txt",
            editor_->document()->blockCount(),
            editor_->toPlainText().size(),
            QString("Ln %1, Col %2").arg(cursor.blockNumber() + 1).arg(cursor.positionInBlock() + 1),
            selectionText.isEmpty() ? QString("selection: none") : QString("selection: %1 chars").arg(selectionText.size()));
    }

    QPlainTextEdit *editor_ = nullptr;
    QPlainTextEdit *expected_ = nullptr;
    QPlainTextEdit *actual_ = nullptr;
    QComboBox *fixturePicker_ = nullptr;
    QLabel *editorStatus_ = nullptr;
    QLabel *cursorStatus_ = nullptr;
    QLabel *fixtureStatus_ = nullptr;
    QFrame *actionStrip_ = nullptr;
    QGridLayout *actionsLayout_ = nullptr;
    TextEditorWorkspaceController *controller_ = nullptr;
    int actionColumnCount_ = 0;
    QString actionInventorySource_ = "C++ fixture fallback";
    QString activeActionId_;
    QString activeActionState_ = "default";
};

QFrame *makeContextPanel(const QString &title, const QStringList &lines, const QString &uiPath, const QString &state = "empty") {
    auto *panel = makePanel("textEditorContextPanel", uiPath);
    setComponentState(panel, state);
    auto *layout = new QVBoxLayout(panel);
    layout->setContentsMargins(
        dex_ui::text_editor_metrics::panel_padding,
        dex_ui::text_editor_metrics::panel_padding_dense,
        dex_ui::text_editor_metrics::panel_padding,
        dex_ui::text_editor_metrics::panel_padding_dense);
    layout->setSpacing(dex_ui::text_editor_metrics::region_gap);
    layout->addWidget(makeTextEditorLabel(title, "sectionLabel", uiPath + ".title"));
    for (const QString &line : lines) {
        auto *label = makeTextEditorLabel(line, "textEditorSurfaceEmpty", uiPath + ".line");
        label->setWordWrap(true);
        layout->addWidget(label);
    }
    layout->addStretch(1);
    return panel;
}

QFrame *makeOptionsPanel(TextEditorWorkspaceController *controller) {
    const TextEditorWorkspaceState &state = controller->state();
    auto *panel = makePanel("textEditorContextPanel", "workbench.inspector.text_editor.options");
    setComponentState(panel, "default");
    auto *layout = new QVBoxLayout(panel);
    layout->setContentsMargins(
        dex_ui::text_editor_metrics::panel_padding,
        dex_ui::text_editor_metrics::panel_padding_dense,
        dex_ui::text_editor_metrics::panel_padding,
        dex_ui::text_editor_metrics::panel_padding_dense);
    layout->setSpacing(dex_ui::text_editor_metrics::region_gap);
    layout->addWidget(makeTextEditorLabel("ACTION OPTIONS", "sectionLabel", "workbench.inspector.text_editor.options.title"));

    auto *sourceLabel = makeTextEditorLabel("Source", "textEditorSurfaceEmpty", "workbench.inspector.text_editor.options.source_label");
    auto *source = makeTextEditorLineEdit(state.source, "workbench.inspector.text_editor.options.source");
    auto *languageLabel = makeTextEditorLabel("Language", "textEditorSurfaceEmpty", "workbench.inspector.text_editor.options.language_label");
    auto *language = makeTextEditorLineEdit(state.language, "workbench.inspector.text_editor.options.language");

    const int maxLine = std::max(state.documentLines - 1, 0);
    auto *lineRow = new QWidget;
    lineRow->setProperty("uiPath", "workbench.inspector.text_editor.options.line_range");
    auto *lineLayout = new QHBoxLayout(lineRow);
    lineLayout->setContentsMargins(0, 0, 0, 0);
    lineLayout->setSpacing(dex_ui::text_editor_metrics::region_gap);
    auto *startLine = makeTextEditorSpinBox(state.startLine, maxLine, "workbench.inspector.text_editor.options.line_start");
    auto *endLine = makeTextEditorSpinBox(state.endLine, maxLine, "workbench.inspector.text_editor.options.line_end");
    lineLayout->addWidget(startLine);
    lineLayout->addWidget(makeTextEditorLabel("to", "textEditorSurfaceEmpty", "workbench.inspector.text_editor.options.line_to"));
    lineLayout->addWidget(endLine);

    auto *stripAnsi = makeTextEditorCheckBox(
        "Strip ANSI escape codes",
        state.stripAnsiEscapeCodes,
        "workbench.inspector.text_editor.options.strip_ansi");

    const auto applyInput = [controller, source, language, startLine, endLine, stripAnsi]() {
        controller->setActionInput(
            language->text(),
            source->text(),
            startLine->value(),
            endLine->value(),
            stripAnsi->isChecked());
    };

    QObject::connect(source, &QLineEdit::editingFinished, panel, applyInput);
    QObject::connect(language, &QLineEdit::editingFinished, panel, applyInput);
    QObject::connect(startLine, QOverload<int>::of(&QSpinBox::valueChanged), panel, applyInput);
    QObject::connect(endLine, QOverload<int>::of(&QSpinBox::valueChanged), panel, applyInput);
    QObject::connect(stripAnsi, &QCheckBox::toggled, panel, [stripAnsi, applyInput](bool checked) {
        stripAnsi->setProperty("componentState", checked ? "active" : "default");
        stripAnsi->style()->unpolish(stripAnsi);
        stripAnsi->style()->polish(stripAnsi);
        applyInput();
    });

    layout->addWidget(sourceLabel);
    layout->addWidget(source);
    layout->addWidget(languageLabel);
    layout->addWidget(language);
    layout->addWidget(makeTextEditorLabel("Line Range", "textEditorSurfaceEmpty", "workbench.inspector.text_editor.options.line_range_label"));
    layout->addWidget(lineRow);
    layout->addWidget(stripAnsi);
    return panel;
}

QFrame *makeWorkspaceStatePanel(TextEditorWorkspaceController *controller) {
    const TextEditorWorkspaceState &state = controller->state();
    return makeContextPanel(
        "WORKSPACE STATE",
        {
            "detail lens: " + state.detailLens,
            "focused surface: " + state.focusedSurface,
            "document: " + state.documentName,
            lineSummary(state.documentLines, state.documentCharacters),
            QString("limits: doc %1 | clipboard %2 | preview %3")
                .arg(dex_ui::text_editor_content_limits::max_document_chars)
                .arg(dex_ui::text_editor_content_limits::max_clipboard_chars)
                .arg(dex_ui::text_editor_content_limits::max_output_preview_chars),
            state.cursorSummary,
            state.selectionSummary,
        },
        "workbench.inspector.text_editor.context",
        state.componentState);
}

QFrame *makeResultPanel(TextEditorWorkspaceController *controller) {
    const TextEditorWorkspaceState &state = controller->state();
    return makeContextPanel(
        "LAST RESULT",
        {
            "action: " + state.lastActionId,
            "kind: " + state.lastActionKind,
            "summary: " + state.lastResultSummary,
            "fixture: " + state.fixtureStatus,
        },
        "workbench.inspector.text_editor.result",
        state.componentState);
}

QFrame *makeReceiptPanel(TextEditorWorkspaceController *controller) {
    const TextEditorWorkspaceState &state = controller->state();
    return makeContextPanel(
        "RECEIPT",
        {
            state.lastReceiptSummary,
        },
        "workbench.inspector.text_editor.receipts",
        state.componentState);
}

} // namespace

namespace DexTextEditorWorkspace {

QWidget *buildBlankWorkspaceBody(TextEditorWorkspaceController *controller) {
    return new TextEditorWorkspaceBody(controller);
}

void addBlankWorkspaceContext(QVBoxLayout *layout, TextEditorWorkspaceController *controller) {
    auto *ownedController = controller
        ? controller
        : new TextEditorWorkspaceController(layout->parentWidget());
    layout->setSpacing(dex_ui::text_editor_metrics::region_gap);
    layout->addWidget(makeOptionsPanel(ownedController));
    layout->addWidget(makeWorkspaceStatePanel(ownedController));
    layout->addWidget(makeResultPanel(ownedController));
    layout->addWidget(makeReceiptPanel(ownedController), 1);
}

} // namespace DexTextEditorWorkspace

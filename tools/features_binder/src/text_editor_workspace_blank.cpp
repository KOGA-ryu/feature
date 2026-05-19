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
#include <QScrollArea>
#include <QSignalBlocker>
#include <QSizePolicy>
#include <QSpinBox>
#include <QStringList>
#include <QTextCursor>
#include <QTextDocument>
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

QString fixtureSummaryText(const DexTextActions::TextActionFixtureSuiteResult &suite) {
    QStringList lines;
    lines << suite.summary;
    for (const DexTextActions::TextActionFixtureResult &result : suite.results) {
        lines << result.summary;
    }
    return lines.join('\n');
}

class TextEditorWorkspaceBody final : public QWidget {
public:
    explicit TextEditorWorkspaceBody(TextEditorWorkspaceController *controller, QWidget *parent = nullptr)
        : QWidget(parent),
          controller_(controller ? controller : new TextEditorWorkspaceController(this)) {
        setObjectName("textEditorBlankBody");
        setUiPath(this, "workbench.editor.workspace");

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

private:
    QFrame *buildActionStrip() {
        auto *strip = makePanel("textEditorActionStrip", "workbench.toolbar.primary");
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
        setComponentState(shelf, "empty");

        auto *layout = new QGridLayout(shelf);
        layout->setContentsMargins(
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding,
            dex_ui::text_editor_metrics::panel_padding);
        layout->setSpacing(dex_ui::text_editor_metrics::region_gap);

        auto *runner = makePanel("textEditorFixturePanel", "workbench.fixture_bench.runner");
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
        for (const DexTextActions::TextActionFixture &fixture : DexTextActions::textActionFixtures()) {
            fixturePicker_->addItem(fixture.label, fixture.fixtureId);
        }
        auto *loadFixture = makeActionButton("Load Fixture", "workbench.fixture_bench.runner.load");
        auto *runFixture = makeActionButton("Run Fixture", "workbench.fixture_bench.runner.run");
        auto *runAll = makeActionButton("Run All", "workbench.fixture_bench.runner.run_all");
        fixtureRowLayout->addWidget(fixturePicker_, 1);
        runnerLayout->addWidget(fixtureRow);

        auto *fixtureButtonRow = new QWidget;
        auto *fixtureButtonLayout = new QGridLayout(fixtureButtonRow);
        fixtureButtonLayout->setContentsMargins(0, 0, 0, 0);
        fixtureButtonLayout->setSpacing(dex_ui::text_editor_metrics::region_gap);
        fixtureButtonLayout->addWidget(loadFixture, 0, 0);
        fixtureButtonLayout->addWidget(runFixture, 0, 1);
        fixtureButtonLayout->addWidget(runAll, 0, 2);
        fixtureButtonLayout->setColumnStretch(0, 1);
        fixtureButtonLayout->setColumnStretch(1, 1);
        fixtureButtonLayout->setColumnStretch(2, 1);
        runnerLayout->addWidget(fixtureButtonRow);

        fixtureStatus_ = makeTextEditorLabel("No fixture run yet.", "textEditorSurfaceEmpty", "workbench.fixture_bench.runner.status");
        fixtureStatus_->setWordWrap(true);
        runnerLayout->addWidget(fixtureStatus_);
        layout->addWidget(runner, 0, 0);

        expected_ = makeOutputBox("expected output", "workbench.fixture_bench.results.expected");
        actual_ = makeOutputBox("actual output", "workbench.fixture_bench.results.actual");
        layout->addWidget(makeOutputPanel("EXPECTED", expected_, "workbench.fixture_bench.results.expected_panel"), 0, 1);
        layout->addWidget(makeOutputPanel("ACTUAL / ACTION OUTPUT", actual_, "workbench.fixture_bench.results.actual_panel"), 0, 2);
        layout->setColumnStretch(0, 1);
        layout->setColumnStretch(1, 2);
        layout->setColumnStretch(2, 2);

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
        button->setMinimumWidth(0);
        button->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Fixed);
        return button;
    }

    QPlainTextEdit *makeOutputBox(const QString &placeholder, const QString &uiPath) const {
        auto *box = new QPlainTextEdit;
        box->setObjectName("textEditorOutputPreview");
        box->setProperty("uiPath", uiPath);
        box->setReadOnly(true);
        box->setMinimumHeight(112);
        box->setPlaceholderText(placeholder);
        return box;
    }

    QFrame *makeOutputPanel(const QString &title, QPlainTextEdit *output, const QString &uiPath) const {
        auto *panel = makePanel("textEditorFixturePanel", uiPath);
        auto *layout = new QVBoxLayout(panel);
        layout->setContentsMargins(12, 10, 12, 10);
        layout->setSpacing(8);
        layout->addWidget(makeTextEditorLabel(title, "textEditorSurfaceTitle", uiPath + ".title"));
        layout->addWidget(output, 1);
        return panel;
    }

    void rebuildActionButtons() {
        if (!actionsLayout_) {
            return;
        }
        clearTextEditorLayout(actionsLayout_);
        const QVector<DexTextActions::HostActionItem> actions =
            DexTextActions::renderHostActionItems(editor_->toPlainText(), currentInput());
        int row = 0;
        int column = 0;
        for (const DexTextActions::HostActionItem &action : actions) {
            auto *button = makeActionButton(action.shortLabel, "workbench.toolbar.primary." + action.actionId);
            button->setEnabled(action.enabled);
            button->setToolTip(action.hotkeyLabel.isEmpty()
                ? action.tooltip + (action.disabledReason.isEmpty() ? QString() : "\n" + action.disabledReason)
                : action.tooltip + "\n" + action.hotkeyLabel);
            connect(button, &QPushButton::clicked, this, [this, action]() {
                executeAction(action.actionId);
            });
            actionsLayout_->addWidget(button, row, column);
            if (++column == 1) {
                column = 0;
                ++row;
            }
        }
        for (int i = 0; i < 1; ++i) {
            actionsLayout_->setColumnStretch(i, 1);
        }
    }

    void executeAction(const QString &actionId) {
        DexTextEditorRust::ActionRequest request;
        request.actionId = actionId;
        request.documentText = editor_->toPlainText();
        request.selection = currentSelection();
        request.input = currentInput();

        const DexTextEditorRust::ActionResult result =
            DexTextEditorRust::executeActionWithRunner(request);
        if (result.selection.valid) {
            applySelection(result.selection);
        }
        renderRustResult(result);
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
            language_->setText(fixture.input.language.isEmpty() ? QString("text") : fixture.input.language);
            source_->setText(fixture.input.source);
            startLine_->setValue(std::max(0, fixture.input.startLine));
            endLine_->setValue(std::max(0, fixture.input.endLine));
            stripAnsi_->setChecked(fixture.input.stripAnsiEscapeCodes);
            expected_->setPlainText(fixture.expectedClipboardText);
            actual_->clear();
            fixtureStatus_->setText("Fixture loaded: " + fixture.fixtureId);
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
        input.language = language_->text();
        input.source = source_->text();
        input.currentLine = editor_->textCursor().blockNumber();
        input.startLine = startLine_->value();
        input.endLine = endLine_->value();
        input.stripAnsiEscapeCodes = stripAnsi_->isChecked();
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

    void renderRustResult(const DexTextEditorRust::ActionResult &result) {
        if (!result.ok) {
            expected_->setPlainText("runner error");
            actual_->setPlainText(result.error);
            fixtureStatus_->setText(result.displayText.isEmpty() ? result.error : result.displayText);
            return;
        }

        const bool shouldWriteClipboard = result.hasClipboardText
            && result.kind != "disabled"
            && result.kind != "none";
        if (shouldWriteClipboard) {
            QGuiApplication::clipboard()->setText(result.clipboardText);
        }

        actual_->setPlainText(result.hasClipboardText ? result.clipboardText : result.displayText);
        expected_->setPlainText(receiptText(result.receipt));
        fixtureStatus_->setText(result.actionId + " | " + result.kind + " | "
            + result.displayText
            + (shouldWriteClipboard ? " | copied to clipboard" : ""));
    }

    void renderResult(const DexTextActions::HostActionResult &result) {
        actual_->setPlainText(result.clipboardText.isEmpty() ? result.displayText : result.clipboardText);
        expected_->setPlainText(receiptText(result.receipt));
        fixtureStatus_->setText(result.actionId + " | " + result.kind + " | " + result.displayText);
    }

    void renderFixtureResult(const DexTextActions::TextActionFixtureResult &result) {
        expected_->setPlainText(result.expectedClipboardText);
        actual_->setPlainText(result.actualClipboardText);
        fixtureStatus_->setText(result.summary);
    }

    void renderFixtureSuiteResult(const DexTextActions::TextActionFixtureSuiteResult &suite) {
        expected_->setPlainText("fixture suite");
        actual_->setPlainText(fixtureSummaryText(suite));
        fixtureStatus_->setText(suite.summary);
    }

    void refreshLineControls() {
        const int maxLine = std::max(0, editor_->document()->blockCount() - 1);
        startLine_->setMaximum(maxLine);
        endLine_->setMaximum(maxLine);
        if (startLine_->value() > maxLine) {
            startLine_->setValue(maxLine);
        }
        if (endLine_->value() > maxLine) {
            endLine_->setValue(maxLine);
        }
    }

    void updateEditorStatus() {
        const QTextCursor cursor = editor_->textCursor();
        const QString selectionText = selectedText();
        cursorStatus_->setText(QString("Ln %1, Col %2        UTF-8        LF        selection: %3")
                                   .arg(cursor.blockNumber() + 1)
                                   .arg(cursor.positionInBlock() + 1)
                                   .arg(selectionText.isEmpty() ? QString("none") : QString("%1 chars").arg(selectionText.size())));
        editorStatus_->setText(QString("actions: %1 | fixtures: %2 | clipboard writes on")
                                   .arg(DexTextActions::textActionRecords().size())
                                   .arg(DexTextActions::textActionFixtures().size()));
    }

    QPlainTextEdit *editor_ = nullptr;
    QPlainTextEdit *expected_ = nullptr;
    QPlainTextEdit *actual_ = nullptr;
    QLineEdit *language_ = nullptr;
    QLineEdit *source_ = nullptr;
    QSpinBox *startLine_ = nullptr;
    QSpinBox *endLine_ = nullptr;
    QCheckBox *stripAnsi_ = nullptr;
    QComboBox *fixturePicker_ = nullptr;
    QLabel *editorStatus_ = nullptr;
    QLabel *cursorStatus_ = nullptr;
    QLabel *fixtureStatus_ = nullptr;
    QGridLayout *actionsLayout_ = nullptr;
};

QFrame *makeContextPanel(const QString &title, const QStringList &lines, const QString &uiPath) {
    auto *panel = makePanel("textEditorContextPanel", uiPath);
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
    return new TextEditorWorkspaceBody;
}

void addBlankWorkspaceContext(QVBoxLayout *layout, const QString &detailLens) {
    layout->addWidget(makeContextPanel(
        "TEXT ACTION ENGINE",
        {
            "state: Rust runner workspace",
            QString("actions: %1").arg(DexTextActions::textActionRecords().size()),
            QString("fixtures: %1").arg(DexTextActions::textActionFixtures().size()),
            "system clipboard: writes copy/export output",
        },
        "workbench.inspector.text_editor.engine"));
    layout->addWidget(makeContextPanel(
        "ACTIVE LENS",
        {
            "detail lens: " + detailLens,
            "document source: scratch editor or fixture",
            "output target: preview panes",
            "fixture runner: available",
        },
        "workbench.inspector.text_editor.context"));
    layout->addWidget(makeContextPanel(
        "V1 BOUNDARY",
        {
            "copy/export/cleanup actions are functional",
            "select all changes editor selection",
            "Rust runner owns action execution",
        },
        "workbench.inspector.text_editor.receipts"));
}

} // namespace DexTextEditorWorkspace

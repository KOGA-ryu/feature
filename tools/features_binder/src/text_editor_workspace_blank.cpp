#include "text_editor_workspace_blank.h"

#include <QCheckBox>
#include <QClipboard>
#include <QComboBox>
#include <QFrame>
#include <QGuiApplication>
#include <QHBoxLayout>
#include <QEvent>
#include <QKeyEvent>
#include <QKeySequence>
#include <QLabel>
#include <QLineEdit>
#include <QPlainTextEdit>
#include <QResizeEvent>
#include <QScrollArea>
#include <QScrollBar>
#include <QShortcut>
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
#include "text_editor_ui.h"
#include "text_editor_workspace_state.h"
#include "ui_rules.h"

namespace {

using DexTextEditorUi::makeOutputBox;
using DexTextEditorUi::makeOutputPanel;
using DexTextEditorUi::makePanel;
using DexTextEditorUi::makeTextEditorLabel;
using DexTextEditorUi::setComponentState;
using DexTextEditorUi::setUiPath;

QString actionPathSuffix(const QString &actionId) {
    return actionId.startsWith("text.") ? actionId.mid(5) : actionId;
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

void applyPanelDescriptor(QWidget *widget, const QString &uiPath) {
    for (const DexTextEditorUi::TextEditorPanelDescriptor &descriptor : DexTextEditorUi::textEditorPanelDescriptors()) {
        if (descriptor.uiPath != uiPath) {
            continue;
        }
        widget->setProperty("panelKey", descriptor.key);
        widget->setProperty("persistentName", descriptor.persistentName);
        widget->setProperty("panelPosition", DexTextEditorUi::panelPositionName(descriptor.position));
        widget->setProperty("defaultSize", descriptor.defaultSize);
        widget->setProperty("minSize", descriptor.minSize);
        widget->setProperty("startsOpen", descriptor.startsOpen);
        widget->setProperty("enabled", descriptor.enabled);
        widget->setProperty("activationPriority", descriptor.activationPriority);
        return;
    }
}

void setPreviewText(QPlainTextEdit *edit, const QString &text, int limit) {
    if (edit) {
        edit->setPlainText(clippedPreviewText(text, limit));
    }
}

void setCleanupPreviewProperties(
    QPlainTextEdit *actual,
    QPlainTextEdit *receipt,
    bool active,
    const QString &actionId = QString(),
    int changeCount = 0,
    int warningCount = 0) {
    if (actual) {
        actual->setProperty("previewMode", active ? "cleanup_before_after" : "output");
        actual->setProperty("cleanupActionId", active ? actionId : QString("none"));
        actual->setProperty("cleanupChangeCount", active ? changeCount : 0);
        actual->setProperty("cleanupWarningCount", active ? warningCount : 0);
        actual->setProperty("cleanupBeforeLabel", active ? QString("BEFORE") : QString());
        actual->setProperty("cleanupAfterLabel", active ? QString("AFTER") : QString());
    }
    if (receipt) {
        receipt->setProperty("receiptMode", active ? "cleanup_receipt" : "receipt");
        receipt->setProperty("receiptActionId", active ? actionId : QString("none"));
        receipt->setProperty("receiptChangeCount", active ? changeCount : 0);
        receipt->setProperty("receiptWarningCount", active ? warningCount : 0);
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

        bodyLayout->addWidget(buildDocumentSurface(), 1);
        bodyLayout->addWidget(buildFixtureShelf());
        buildCommandPalette();

        auto *paletteShortcut = new QShortcut(QKeySequence("Ctrl+Shift+P"), this);
        connect(paletteShortcut, &QShortcut::activated, this, [this]() {
            showCommandPalette();
        });
        connect(editor_, &QPlainTextEdit::textChanged, this, [this]() {
            refreshLineControls();
            renderCommandPaletteResults();
            updateEditorStatus();
        });
        connect(editor_, &QPlainTextEdit::cursorPositionChanged, this, [this]() {
            updateEditorStatus();
            renderCommandPaletteResults();
        });
        connect(editor_->verticalScrollBar(), &QScrollBar::valueChanged, this, [this]() {
            updateEditorStatus();
        });
        connect(controller_, &TextEditorWorkspaceController::actionInputChanged, this, [this]() {
            refreshLineControls();
            renderCommandPaletteResults();
        });
        connect(controller_, &TextEditorWorkspaceController::commandPaletteRequested, this, [this]() {
            showCommandPalette();
        });
        connect(controller_, &TextEditorWorkspaceController::cleanupPreviewProofRequested, this, [this]() {
            runCleanupPreviewProof();
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
        positionCommandPalette();
    }

    bool eventFilter(QObject *watched, QEvent *event) override {
        if (watched == commandPaletteQuery_ && event->type() == QEvent::KeyPress) {
            auto *keyEvent = static_cast<QKeyEvent *>(event);
            switch (keyEvent->key()) {
            case Qt::Key_Escape:
                hideCommandPalette();
                return true;
            case Qt::Key_Down:
                movePaletteSelection(1);
                return true;
            case Qt::Key_Up:
                movePaletteSelection(-1);
                return true;
            case Qt::Key_Return:
            case Qt::Key_Enter:
                runSelectedPaletteAction();
                return true;
            default:
                break;
            }
        }
        auto *widget = qobject_cast<QWidget *>(watched);
        if (widget && widget->property("paletteActionIndex").isValid()) {
            if (event->type() == QEvent::Enter) {
                const int index = widget->property("paletteActionIndex").toInt();
                if (index >= 0 && index < paletteMatches_.size() && paletteMatches_.at(index).enabled) {
                    paletteSelectedIndex_ = index;
                    updatePaletteRowStates();
                    updatePaletteProofProperties();
                }
                return false;
            }
            if (event->type() == QEvent::MouseButtonRelease) {
                const QString actionId = widget->property("actionId").toString();
                const bool enabled = widget->property("paletteActionEnabled").toBool();
                if (enabled && !actionId.isEmpty()) {
                    hideCommandPalette();
                    executeAction(actionId);
                    return true;
                }
            }
        }
        return QWidget::eventFilter(watched, event);
    }

    QFrame *buildCommandPalette() {
        auto *palette = makePanel("textEditorCommandPalette", "workbench.palette");
        commandPalette_ = palette;
        commandPalette_->setParent(this);
        commandPalette_->setFixedWidth(560);
        commandPalette_->setMaximumHeight(360);
        commandPalette_->setVisible(false);
        setComponentState(palette, "closed");

        auto *layout = new QVBoxLayout(palette);
        layout->setContentsMargins(0, 0, 0, 0);
        layout->setSpacing(0);

        commandPaletteQuery_ = new QLineEdit;
        commandPaletteQuery_->setObjectName("textEditorCommandPaletteInput");
        commandPaletteQuery_->setProperty("uiPath", "workbench.palette.search.input");
        commandPaletteQuery_->setPlaceholderText("Search actions");
        commandPaletteQuery_->installEventFilter(this);
        layout->addWidget(commandPaletteQuery_);

        commandPaletteResults_ = makePanel("textEditorCommandPaletteResults", "workbench.palette.results");
        commandPaletteResultsLayout_ = new QVBoxLayout(commandPaletteResults_);
        commandPaletteResultsLayout_->setContentsMargins(0, 0, 0, 0);
        commandPaletteResultsLayout_->setSpacing(0);
        layout->addWidget(commandPaletteResults_);

        connect(commandPaletteQuery_, &QLineEdit::textChanged, this, [this]() {
            paletteSelectedIndex_ = -1;
            renderCommandPaletteResults();
        });
        renderCommandPaletteResults();
        positionCommandPalette();

        return palette;
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

        auto *tab = DexTextEditorUi::makeDocumentTab("scratch.txt", "workbench.editor.tabs.active_document");
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
        snapshotStatus_ = makeTextEditorLabel(
            "visible rows 1-1 | scroll 0/0 | action none/default",
            "textEditorMonoLabel",
            "workbench.editor.snapshot");
        snapshotStatus_->setProperty("componentState", "empty");
        snapshotStatus_->setContentsMargins(
            dex_ui::text_editor_metrics::panel_padding,
            0,
            dex_ui::text_editor_metrics::panel_padding,
            0);
        layout->addWidget(snapshotStatus_);
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
        fixtureRowLayout->addWidget(fixturePicker_, 1);
        runnerLayout->addWidget(fixtureRow);

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

        connect(fixturePicker_, &QComboBox::currentIndexChanged, this, [this]() {
            loadSelectedFixture();
        });
        return shelf;
    }

    QVector<DexTextActions::HostActionItem> availableActions() {
        if (!editor_) {
            actionInventorySource_ = "C++ fixture fallback";
            return DexTextActions::renderHostActionItems("", {});
        }
        QString actionError;
        QVector<DexTextActions::HostActionItem> actions =
            DexTextEditorRust::renderActionsWithRunner(editor_->toPlainText(), currentInput(), &actionError);
        actionInventorySource_ = actions.isEmpty() ? "C++ fixture fallback" : "Rust action runner";
        if (actions.isEmpty()) {
            actions = DexTextActions::renderHostActionItems(editor_->toPlainText(), currentInput());
        }
        return actions;
    }

    void renderCommandPaletteResults() {
        if (!commandPaletteResultsLayout_) {
            return;
        }
        clearTextEditorLayout(commandPaletteResultsLayout_);
        const QVector<DexTextActions::HostActionItem> actions = availableActions();
        controller_->setActionInventory(actions.size(), DexTextActions::textActionFixtures().size());
        paletteMatches_ =
            DexTextEditorUi::filterCommandPaletteActions(
                actions,
                commandPaletteQuery_ ? commandPaletteQuery_->text() : QString());
        paletteSelectedIndex_ = DexTextEditorUi::moveCommandPaletteSelection(paletteMatches_, paletteSelectedIndex_, 0);
        updatePaletteProofProperties();
        bool sawDisabled = false;
        for (int index = 0; index < paletteMatches_.size(); ++index) {
            const DexTextActions::HostActionItem &action = paletteMatches_.at(index);
            const bool isSelected = index == paletteSelectedIndex_;
            const QString uiPath = QString("workbench.palette.results.action.%1").arg(actionPathSuffix(action.actionId));
            const QString displayText = DexTextEditorUi::commandPaletteRowText(action);
            auto *row = makePanel("textEditorCommandPaletteRow", uiPath);
            row->setProperty("actionId", action.actionId);
            row->setProperty("actionLabel", action.label);
            row->setProperty("displayText", displayText);
            row->setProperty("category", action.category);
            row->setProperty("iconName", action.icon);
            row->setProperty("hotkeyLabel", action.hotkeyLabel);
            row->setProperty("disabledReason", action.disabledReason);
            row->setProperty("paletteRole", "menuitem");
            row->setProperty("paletteActionIndex", index);
            row->setProperty("paletteActionEnabled", action.enabled);
            row->setProperty("componentState", action.enabled ? (isSelected ? "selected" : "default") : "disabled");
            const QString accessibleName = DexTextEditorUi::commandPaletteAccessibleName(action);
            row->setAccessibleName(accessibleName);
            row->setProperty("accessibleLabel", accessibleName);
            row->setToolTip(action.hotkeyLabel.isEmpty()
                ? action.tooltip + (action.disabledReason.isEmpty() ? QString() : "\n" + action.disabledReason)
                : action.tooltip + "\n" + action.hotkeyLabel);
            row->setFixedHeight(22);
            row->setAttribute(Qt::WA_Hover, true);
            row->installEventFilter(this);
            row->setCursor(action.enabled ? Qt::PointingHandCursor : Qt::ArrowCursor);
            auto *rowLayout = new QHBoxLayout(row);
            rowLayout->setContentsMargins(7, 0, 7, 0);
            rowLayout->setSpacing(dex_ui::text_editor_metrics::dense_gap);
            auto *label = makeTextEditorLabel(
                displayText,
                "textEditorCommandPaletteRowText");
            label->setProperty("paletteRole", "menuitem_label");
            label->setAttribute(Qt::WA_TransparentForMouseEvents, true);
            rowLayout->addWidget(label, 1);
            sawDisabled = sawDisabled || !action.enabled;
            commandPaletteResultsLayout_->addWidget(row);
        }
        commandPalette_->setProperty("disabledRowsPreserved", sawDisabled);
        updatePaletteProofProperties();
        positionCommandPalette();
    }

    void updatePaletteProofProperties() {
        if (!commandPalette_) {
            return;
        }
        const bool isOpen = !commandPalette_->isHidden();
        commandPalette_->setProperty("paletteOpen", isOpen);
        commandPalette_->setProperty("focusedSurface", isOpen ? "Palette" : "Editor");
        commandPalette_->setProperty(
            "selectedActionId",
            DexTextEditorUi::commandPaletteSelectedActionId(paletteMatches_, paletteSelectedIndex_));
        commandPalette_->setProperty("selectedRowIndex", paletteSelectedIndex_);
        commandPalette_->setProperty("resultCount", paletteMatches_.size());
        commandPalette_->setProperty("hotkeyOnly", true);
        commandPalette_->setProperty("popout", true);
        commandPalette_->setProperty("hasLauncherButton", false);
        commandPalette_->setProperty("hasTitle", false);
        commandPalette_->setProperty("hasFrameBorder", false);
    }

    void movePaletteSelection(int direction) {
        paletteSelectedIndex_ =
            DexTextEditorUi::moveCommandPaletteSelection(paletteMatches_, paletteSelectedIndex_, direction);
        updatePaletteRowStates();
        updatePaletteProofProperties();
    }

    void runSelectedPaletteAction() {
        const QString actionId = DexTextEditorUi::commandPaletteSelectedActionId(paletteMatches_, paletteSelectedIndex_);
        if (actionId == "none") {
            return;
        }
        hideCommandPalette();
        executeAction(actionId);
    }

    void showCommandPalette() {
        if (!commandPalette_) {
            return;
        }
        QWidget *host = window();
        if (host && commandPalette_->parentWidget() != host) {
            commandPalette_->setParent(host);
        }
        commandPalette_->setVisible(true);
        setComponentState(commandPalette_, "open");
        commandPalette_->raise();
        positionCommandPalette();
        controller_->setFocusedSurface("Palette");
        renderCommandPaletteResults();
        updatePaletteProofProperties();
        if (commandPaletteQuery_) {
            commandPaletteQuery_->setFocus();
            commandPaletteQuery_->selectAll();
        }
    }

    void hideCommandPalette() {
        if (!commandPalette_) {
            return;
        }
        commandPalette_->setVisible(false);
        setComponentState(commandPalette_, "closed");
        controller_->setFocusedSurface("Editor");
        updatePaletteProofProperties();
        if (editor_) {
            editor_->setFocus();
        }
    }

    void updatePaletteRowStates() {
        if (!commandPaletteResultsLayout_) {
            return;
        }
        for (int i = 0; i < commandPaletteResultsLayout_->count(); ++i) {
            QWidget *widget = commandPaletteResultsLayout_->itemAt(i)->widget();
            if (!widget || !widget->property("paletteActionIndex").isValid()) {
                continue;
            }
            const int index = widget->property("paletteActionIndex").toInt();
            const bool enabled = widget->property("paletteActionEnabled").toBool();
            const QString state = enabled
                ? (index == paletteSelectedIndex_ ? QString("selected") : QString("default"))
                : QString("disabled");
            setComponentState(widget, state);
        }
    }

    void positionCommandPalette() {
        if (!commandPalette_) {
            return;
        }
        QWidget *host = commandPalette_->parentWidget() ? commandPalette_->parentWidget() : this;
        const int hostWidth = std::max(1, host->width());
        const int hostHeight = std::max(1, host->height());
        const int paletteWidth = std::min(560, std::max(320, hostWidth - (dex_ui::text_editor_metrics::section_gap * 2)));
        commandPalette_->setFixedWidth(paletteWidth);
        commandPalette_->adjustSize();
        const int paletteHeight = std::min(commandPalette_->sizeHint().height(), 360);
        commandPalette_->setFixedHeight(paletteHeight);
        const int x = std::max(0, (hostWidth - paletteWidth) / 2);
        const int y = std::max(dex_ui::text_editor_metrics::section_gap, hostHeight / 10);
        commandPalette_->move(x, y);
        commandPalette_->setProperty("popoutX", x);
        commandPalette_->setProperty("popoutY", y);
        commandPalette_->setProperty("popoutWidth", paletteWidth);
        commandPalette_->setProperty("popoutHeight", paletteHeight);
        commandPalette_->setProperty("positionAnchor", host == window() ? "main_window" : "workspace");
    }

    void executeAction(const QString &actionId) {
        const QString documentText = editor_->toPlainText();
        if (documentText.size() > dex_ui::text_editor_content_limits::max_document_chars) {
            const QString message = QString("Document is over action limit: %1 / %2 chars")
                .arg(documentText.size())
                .arg(dex_ui::text_editor_content_limits::max_document_chars);
            setCleanupPreviewProperties(actual_, expected_, false);
            setPreviewText(expected_, "content limit", dex_ui::text_editor_content_limits::max_receipt_preview_chars);
            setPreviewText(actual_, message, dex_ui::text_editor_content_limits::max_output_preview_chars);
            fixtureStatus_->setText(message);
            setComponentState(fixtureStatus_, "danger");
            controller_->setLastResult(actionId, "content_limit", message, message, "danger");
            return;
        }

        activeActionId_ = actionId;
        activeActionState_ = "running";
        renderCommandPaletteResults();
        updateEditorStatus();

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
        renderRustResult(actionId, result, documentText);
        renderCommandPaletteResults();
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
            setCleanupPreviewProperties(actual_, expected_, false);
            setPreviewText(expected_, fixture.expectedClipboardText, dex_ui::text_editor_content_limits::max_output_preview_chars);
            actual_->clear();
            fixtureStatus_->setText("Fixture loaded: " + fixture.fixtureId);
            setComponentState(fixtureStatus_, "empty");
            controller_->setFixtureStatus("Fixture loaded: " + fixture.fixtureId, "empty");
            refreshLineControls();
            renderCommandPaletteResults();
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

    void runCleanupPreviewProof() {
        const QString dirtyText = QString("one  \ntwo\t\n") + QChar(0x1b) + "[31mred" + QChar(0x1b) + "[0m";
        {
            const QSignalBlocker blockEditor(editor_);
            editor_->setPlainText(dirtyText);
        }
        controller_->setActionInput("text", "features_binder_cleanup_preview", 0, 0, true);
        refreshLineControls();
        executeAction("text.clean_basic");
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

    void renderRustResult(
        const QString &requestedActionId,
        const DexTextEditorRust::ActionResult &result,
        const QString &documentTextBeforeAction) {
        const QString resultActionId = result.actionId.isEmpty() ? requestedActionId : result.actionId;
        if (!result.ok) {
            setCleanupPreviewProperties(actual_, expected_, false);
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

        const bool cleanupPreview = DexTextEditorUi::isCleanupActionId(resultActionId)
            && result.kind == "clipboard_transform"
            && result.hasClipboardText;
        const QString outputText = cleanupPreview
            ? DexTextEditorUi::cleanupPreviewText(documentTextBeforeAction, result.clipboardText)
            : (result.hasClipboardText ? result.clipboardText : result.displayText);
        const QString receiptSummary = resultReceiptText(result);
        setCleanupPreviewProperties(
            actual_,
            expected_,
            cleanupPreview,
            resultActionId,
            result.receipt.changeCount,
            result.receipt.warningCount);
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
                renderCommandPaletteResults();
                updateEditorStatus();
            }
        });
    }

    void renderResult(const DexTextActions::HostActionResult &result) {
        setCleanupPreviewProperties(actual_, expected_, false);
        setPreviewText(actual_, result.clipboardText.isEmpty() ? result.displayText : result.clipboardText, dex_ui::text_editor_content_limits::max_output_preview_chars);
        setPreviewText(expected_, receiptText(result.receipt), dex_ui::text_editor_content_limits::max_receipt_preview_chars);
        fixtureStatus_->setText(result.actionId + " | " + result.kind + " | " + result.displayText);
        const QString state = resultStateForKind(result.kind, true);
        setComponentState(fixtureStatus_, state);
        controller_->setLastResult(result.actionId, result.kind, result.displayText, receiptText(result.receipt), state);
    }

    void renderFixtureResult(const DexTextActions::TextActionFixtureResult &result) {
        setCleanupPreviewProperties(actual_, expected_, false);
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
        setCleanupPreviewProperties(actual_, expected_, false);
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
        const int lineCount = std::max(1, editor_->document()->blockCount());
        const int characterCount = std::max(0, static_cast<int>(editor_->toPlainText().size()));
        QScrollBar *scrollBar = editor_->verticalScrollBar();
        const int scrollValue = scrollBar ? scrollBar->value() : 0;
        const int scrollMaximum = scrollBar ? scrollBar->maximum() : 0;
        const int visibleStart = std::min(lineCount, scrollValue + 1);
        const int visibleRows = std::max(1, editor_->viewport()->height() / std::max(1, editor_->fontMetrics().height()));
        const int visibleEnd = std::min(lineCount, visibleStart + visibleRows - 1);
        const QString cursorText = QString("Ln %1, Col %2").arg(cursor.blockNumber() + 1).arg(cursor.positionInBlock() + 1);
        const QString selectionSummary = selectionText.isEmpty() ? QString("selection: none") : QString("selection: %1 chars").arg(selectionText.size());
        cursorStatus_->setText(QString("Ln %1, Col %2        UTF-8        LF        selection: %3")
                                   .arg(cursor.blockNumber() + 1)
                                   .arg(cursor.positionInBlock() + 1)
                                   .arg(selectionText.isEmpty() ? QString("none") : QString("%1 chars").arg(selectionText.size())));
        setComponentState(cursorStatus_, selectionText.isEmpty() ? QString("empty") : QString("active"));
        editorStatus_->setText(QString("actions: %1 | fixtures: %2")
                                   .arg(controller_->state().actionCount)
                                   .arg(DexTextActions::textActionFixtures().size()));
        if (snapshotStatus_) {
            snapshotStatus_->setText(QString("visible rows %1-%2 | scroll %3/%4 | action %5/%6")
                .arg(visibleStart)
                .arg(visibleEnd)
                .arg(scrollValue)
                .arg(scrollMaximum)
                .arg(activeActionId_.isEmpty() ? QString("none") : activeActionId_)
                .arg(activeActionState_));
            snapshotStatus_->setProperty("snapshotDocumentName", "scratch.txt");
            snapshotStatus_->setProperty("snapshotLineCount", lineCount);
            snapshotStatus_->setProperty("snapshotCharacterCount", characterCount);
            snapshotStatus_->setProperty("snapshotVisibleBlockStart", visibleStart);
            snapshotStatus_->setProperty("snapshotVisibleBlockEnd", visibleEnd);
            snapshotStatus_->setProperty("snapshotScrollValue", scrollValue);
            snapshotStatus_->setProperty("snapshotScrollMaximum", scrollMaximum);
            snapshotStatus_->setProperty("snapshotActiveActionId", activeActionId_.isEmpty() ? QString("none") : activeActionId_);
            snapshotStatus_->setProperty("snapshotActiveActionState", activeActionState_);
        }
        DexTextEditorWorkspace::TextEditorDisplaySnapshot snapshot;
        snapshot.documentName = "scratch.txt";
        snapshot.lineCount = lineCount;
        snapshot.characterCount = characterCount;
        snapshot.cursorSummary = cursorText;
        snapshot.selectionSummary = selectionSummary;
        snapshot.visibleBlockStart = visibleStart;
        snapshot.visibleBlockEnd = visibleEnd;
        snapshot.verticalScrollValue = scrollValue;
        snapshot.verticalScrollMaximum = scrollMaximum;
        snapshot.activeActionId = activeActionId_.isEmpty() ? QString("none") : activeActionId_;
        snapshot.activeActionState = activeActionState_;
        snapshot.lastResultSummary = controller_->state().lastResultSummary;
        snapshot.lastReceiptSummary = controller_->state().lastReceiptSummary;
        controller_->setEditorSnapshot(snapshot);
    }

    QPlainTextEdit *editor_ = nullptr;
    QPlainTextEdit *expected_ = nullptr;
    QPlainTextEdit *actual_ = nullptr;
    QComboBox *fixturePicker_ = nullptr;
    QLabel *editorStatus_ = nullptr;
    QLabel *cursorStatus_ = nullptr;
    QLabel *snapshotStatus_ = nullptr;
    QLabel *fixtureStatus_ = nullptr;
    QFrame *commandPalette_ = nullptr;
    QLineEdit *commandPaletteQuery_ = nullptr;
    QFrame *commandPaletteResults_ = nullptr;
    QVBoxLayout *commandPaletteResultsLayout_ = nullptr;
    QVector<DexTextActions::HostActionItem> paletteMatches_;
    int paletteSelectedIndex_ = -1;
    TextEditorWorkspaceController *controller_ = nullptr;
    QString actionInventorySource_ = "C++ fixture fallback";
    QString activeActionId_;
    QString activeActionState_ = "default";
};

QFrame *makeContextPanel(const QString &title, const QStringList &lines, const QString &uiPath, const QString &state = "empty") {
    auto *panel = makePanel("textEditorContextPanel", uiPath);
    applyPanelDescriptor(panel, uiPath);
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
    applyPanelDescriptor(panel, "workbench.inspector.text_editor.options");
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
    const DexTextEditorWorkspace::TextEditorDisplaySnapshot snapshot = state.snapshot;
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
            QString("visible rows: %1-%2").arg(snapshot.visibleBlockStart).arg(snapshot.visibleBlockEnd),
            QString("scroll: %1/%2").arg(snapshot.verticalScrollValue).arg(snapshot.verticalScrollMaximum),
            "active action: " + snapshot.activeActionId + " / " + snapshot.activeActionState,
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

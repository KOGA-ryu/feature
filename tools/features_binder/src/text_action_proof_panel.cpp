#include "text_action_proof_panel.h"

#include <QCheckBox>
#include <QGridLayout>
#include <QHBoxLayout>
#include <QLabel>
#include <QLineEdit>
#include <QPlainTextEdit>
#include <QPushButton>
#include <QSpinBox>
#include <QTextCursor>
#include <QVBoxLayout>

#include "binder_page_helpers.h"

namespace {

QLineEdit *smallLineEdit(const QString &value) {
    auto *edit = new QLineEdit;
    edit->setText(value);
    edit->setMinimumHeight(22);
    return edit;
}

void clearLayout(QLayout *layout) {
    while (QLayoutItem *item = layout->takeAt(0)) {
        if (QWidget *widget = item->widget()) {
            widget->deleteLater();
        }
        if (QLayout *child = item->layout()) {
            clearLayout(child);
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

} // namespace

TextActionProofPanel::TextActionProofPanel(QWidget *parent)
    : QWidget(parent) {
    auto *layout = new QVBoxLayout(this);
    layout->setContentsMargins(0, 0, 0, 0);
    layout->setSpacing(8);

    auto *header = DexBinderPages::makeStatsSection("text action host proof", true);
    auto *headerLayout = static_cast<QVBoxLayout *>(header->layout());
    headerLayout->addWidget(DexBinderPages::makeStatsText(
        "Proof surface for reusable text-editor actions. Buttons are rendered from action metadata; this panel does not write to the system clipboard."));
    headerLayout->addWidget(DexBinderPages::makeStatsRow({
        "chain",
        "plain -> clipboard -> actions -> host adapter",
        "host",
        "features binder",
        "clipboard",
        "preview only",
    }, false, false));
    layout->addWidget(header);

    auto *editorSection = DexBinderPages::makeStatsSection("sample text");
    auto *editorLayout = static_cast<QVBoxLayout *>(editorSection->layout());
    editor_ = new QPlainTextEdit;
    editor_->setMinimumHeight(128);
    editor_->setPlainText("Paste or write text here.\nThen run generated actions from the proof panel.");
    editorLayout->addWidget(editor_);
    layout->addWidget(editorSection);

    auto *options = DexBinderPages::makeStatsSection("action inputs");
    auto *optionsLayout = static_cast<QVBoxLayout *>(options->layout());
    auto *line = new QWidget;
    auto *lineLayout = new QHBoxLayout(line);
    lineLayout->setContentsMargins(0, 0, 0, 0);
    language_ = smallLineEdit("text");
    source_ = smallLineEdit("features_binder");
    startLine_ = new QSpinBox;
    startLine_->setRange(0, 9999);
    endLine_ = new QSpinBox;
    endLine_->setRange(0, 9999);
    endLine_->setValue(1);
    stripAnsi_ = new QCheckBox("strip ANSI in Clean Basic");
    lineLayout->addWidget(new QLabel("language"));
    lineLayout->addWidget(language_);
    lineLayout->addWidget(new QLabel("source"));
    lineLayout->addWidget(source_);
    lineLayout->addWidget(new QLabel("line start"));
    lineLayout->addWidget(startLine_);
    lineLayout->addWidget(new QLabel("end"));
    lineLayout->addWidget(endLine_);
    lineLayout->addWidget(stripAnsi_);
    optionsLayout->addWidget(line);
    layout->addWidget(options);

    auto *actions = DexBinderPages::makeStatsSection("generated actions", true);
    actionsLayout_ = new QGridLayout;
    actionsLayout_->setContentsMargins(0, 0, 0, 0);
    actionsLayout_->setSpacing(5);
    static_cast<QVBoxLayout *>(actions->layout())->addLayout(actionsLayout_);
    layout->addWidget(actions);

    auto *outputSection = DexBinderPages::makeStatsSection("result preview");
    auto *outputLayout = static_cast<QVBoxLayout *>(outputSection->layout());
    status_ = DexBinderPages::makeStatsText("No action run yet.");
    receipt_ = DexBinderPages::makeStatsText("receipt: none");
    output_ = new QPlainTextEdit;
    output_->setReadOnly(true);
    output_->setMinimumHeight(112);
    outputLayout->addWidget(status_);
    outputLayout->addWidget(receipt_);
    outputLayout->addWidget(output_);
    layout->addWidget(outputSection);

    connect(editor_, &QPlainTextEdit::textChanged, this, [this]() {
        rebuildActionButtons();
    });
    connect(language_, &QLineEdit::textChanged, this, [this]() {
        rebuildActionButtons();
    });
    connect(source_, &QLineEdit::textChanged, this, [this]() {
        rebuildActionButtons();
    });
    connect(startLine_, &QSpinBox::valueChanged, this, [this](int) {
        rebuildActionButtons();
    });
    connect(endLine_, &QSpinBox::valueChanged, this, [this](int) {
        rebuildActionButtons();
    });
    connect(stripAnsi_, &QCheckBox::toggled, this, [this](bool) {
        rebuildActionButtons();
    });
    rebuildActionButtons();
}

void TextActionProofPanel::rebuildActionButtons() {
    clearLayout(actionsLayout_);
    const QVector<DexTextActions::HostActionItem> actions =
        DexTextActions::renderHostActionItems(editor_->toPlainText(), currentInput());
    int row = 0;
    int column = 0;
    for (const DexTextActions::HostActionItem &action : actions) {
        auto *button = new QPushButton(action.shortLabel);
        button->setObjectName("statsContextAction");
        button->setEnabled(action.enabled);
        button->setToolTip(action.hotkeyLabel.isEmpty()
            ? action.tooltip + (action.disabledReason.isEmpty() ? QString() : "\n" + action.disabledReason)
            : action.tooltip + "\n" + action.hotkeyLabel);
        connect(button, &QPushButton::clicked, this, [this, action]() {
            executeAction(action.actionId);
        });
        actionsLayout_->addWidget(button, row, column);
        if (++column == 4) {
            column = 0;
            ++row;
        }
    }
}

void TextActionProofPanel::executeAction(const QString &actionId) {
    if (actionId == "text.select_all") {
        editor_->selectAll();
    }
    renderResult(DexTextActions::executeTextActionProof(
        actionId,
        editor_->toPlainText(),
        selectedText(),
        currentInput()));
    rebuildActionButtons();
}

DexTextActions::TextActionProofInput TextActionProofPanel::currentInput() const {
    DexTextActions::TextActionProofInput input;
    input.language = language_->text();
    input.source = source_->text();
    input.currentLine = editor_->textCursor().blockNumber();
    input.startLine = startLine_->value();
    input.endLine = endLine_->value();
    input.stripAnsiEscapeCodes = stripAnsi_->isChecked();
    return input;
}

QString TextActionProofPanel::selectedText() const {
    QString selected = editor_->textCursor().selectedText();
    selected.replace(QChar(0x2029), '\n');
    return selected;
}

void TextActionProofPanel::renderResult(const DexTextActions::HostActionResult &result) {
    status_->setText(result.displayText);
    receipt_->setText(receiptText(result.receipt));
    output_->setPlainText(result.clipboardText);
}

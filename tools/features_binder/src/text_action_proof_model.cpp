#include "text_action_proof_model.h"

#include <QRegularExpression>
#include <QSet>

namespace DexTextActions {
namespace {

QString normalizeLineEndings(const QString &input, int *count = nullptr) {
    int changes = input.count("\r\n");
    QString output = input;
    output.replace("\r\n", "\n");
    changes += output.count('\r');
    output.replace('\r', '\n');
    if (count) {
        *count = changes;
    }
    return output;
}

QString trimTrailingWhitespace(const QString &input, int *count = nullptr) {
    const QString normalized = normalizeLineEndings(input);
    QStringList lines = normalized.split('\n');
    int changes = 0;
    for (QString &line : lines) {
        const qsizetype originalSize = line.size();
        while (line.endsWith(' ') || line.endsWith('\t')) {
            line.chop(1);
        }
        if (line.size() != originalSize) {
            ++changes;
        }
    }
    if (count) {
        *count = changes;
    }
    return lines.join('\n');
}

QString stripAnsiEscapeCodes(const QString &input, int *count = nullptr, QStringList *warnings = nullptr) {
    QString output;
    output.reserve(input.size());
    int changes = 0;
    for (qsizetype i = 0; i < input.size(); ++i) {
        const QChar character = input.at(i);
        if (character != QChar(0x1b) || i + 1 >= input.size() || input.at(i + 1) != '[') {
            output.push_back(character);
            continue;
        }

        qsizetype end = i + 2;
        bool terminated = false;
        for (; end < input.size(); ++end) {
            const ushort value = input.at(end).unicode();
            if (value >= '@' && value <= '~') {
                terminated = true;
                break;
            }
        }

        if (!terminated) {
            output.push_back(character);
            if (warnings) {
                warnings->push_back("ansi_escape_sequence_incomplete: warn");
            }
            continue;
        }

        ++changes;
        i = end;
    }
    if (count) {
        *count = changes;
    }
    return output;
}

QString fencedBlock(const QString &language, const QString &text) {
    const QString safeLanguage = language.trimmed().isEmpty() ? QString("text") : language.trimmed();
    return QString("```%1\n%2\n```").arg(safeLanguage, text);
}

QString lineAt(const QString &documentText, int index) {
    const QStringList lines = normalizeLineEndings(documentText).split('\n');
    if (index < 0 || index >= lines.size()) {
        return {};
    }
    return lines.at(index);
}

QString lineRange(const QString &documentText, int startLine, int endLine) {
    const QStringList lines = normalizeLineEndings(documentText).split('\n');
    if (lines.isEmpty() || startLine < 0 || endLine < 0) {
        return {};
    }
    const int start = qBound(0, startLine, lines.size() - 1);
    const int end = qBound(start, endLine, lines.size() - 1);
    QStringList selected;
    for (int i = start; i <= end; ++i) {
        selected.push_back(lines.at(i));
    }
    return selected.join('\n');
}

bool isEnabled(const TextActionRecord &record, const QString &documentText, const TextActionProofInput &input) {
    if (record.enabledRule == "always") {
        return true;
    }
    if (record.enabledRule == "document_has_text") {
        return !documentText.isEmpty();
    }
    if (record.enabledRule == "document_or_input_has_text") {
        return !documentText.isEmpty() || !input.explicitText.isEmpty();
    }
    if (record.enabledRule == "has_line_range") {
        return input.startLine >= 0 && input.endLine >= 0;
    }
    return false;
}

QString disabledReason(const TextActionRecord &record, const QString &documentText, const TextActionProofInput &input) {
    if (isEnabled(record, documentText, input)) {
        return {};
    }
    if (record.enabledRule == "document_has_text") {
        return "document is empty";
    }
    if (record.enabledRule == "document_or_input_has_text") {
        return "document and input text are empty";
    }
    if (record.enabledRule == "has_line_range") {
        return "line range is missing";
    }
    return "action is unavailable";
}

const TextActionRecord *findAction(const QString &actionId) {
    static const QVector<TextActionRecord> records = textActionRecords();
    for (const TextActionRecord &record : records) {
        if (record.actionId == actionId) {
            return &record;
        }
    }
    return nullptr;
}

HostActionResult disabledResult(const QString &actionId, const QString &reason) {
    HostActionResult result;
    result.actionId = actionId;
    result.kind = "disabled";
    result.displayText = "Action disabled: " + reason;
    result.warnings = {reason};
    return result;
}

} // namespace

QVector<TextActionRecord> textActionRecords() {
    return {
        {"text.copy_plain", "Copy Plain", "Copy", "clipboard", "copy",
            "Copy selected text, or the full document when nothing is selected.",
            "document_has_text", {"command_palette", "clipboard_menu", "context_menu"}},
        {"text.copy_markdown_block", "Copy Markdown Block", "Markdown", "clipboard", "clipboard-copy",
            "Copy text as a fenced Markdown block.",
            "document_has_text", {"command_palette", "clipboard_menu", "toolbar"}},
        {"text.copy_prompt_block", "Copy Prompt Block", "Prompt", "clipboard", "clipboard-copy",
            "Copy text as a prompt-safe block with a source header.",
            "document_has_text", {"command_palette", "clipboard_menu", "toolbar"}},
        {"text.copy_code_fence", "Copy Code Fence", "Fence", "clipboard", "braces",
            "Copy text as a fenced code block with a language label.",
            "document_has_text", {"command_palette", "clipboard_menu", "toolbar"}},
        {"text.select_all", "Select All", "All", "selection", "scan-text",
            "Select the full document.",
            "document_has_text", {"command_palette", "edit_menu"}},
        {"text.current_line_text", "Current Line Text", "Line", "lines", "text-cursor-input",
            "Return the current line text without changing editor state.",
            "always", {"command_palette"}},
        {"text.line_range_text", "Line Range Text", "Range", "lines", "rows-3",
            "Return an explicit inclusive range of lines.",
            "has_line_range", {"command_palette"}},
        {"text.trim_trailing_whitespace", "Trim Trailing Whitespace", "Trim", "cleanup", "eraser",
            "Return text with line-end spaces and tabs removed.",
            "document_has_text", {"command_palette", "cleanup_menu"}},
        {"text.clean_basic", "Clean Basic", "Clean", "cleanup", "wand-sparkles",
            "Return cleaned text with a transform receipt.",
            "document_or_input_has_text", {"command_palette", "cleanup_menu"}},
        {"text.normalize_line_endings", "Normalize Line Endings", "LF", "cleanup", "pilcrow",
            "Return text with CRLF/CR line endings normalized to LF.",
            "document_or_input_has_text", {"command_palette", "cleanup_menu"}},
        {"text.strip_ansi_escape_codes", "Strip ANSI Escape Codes", "ANSI", "cleanup", "eraser",
            "Return text with ANSI escape sequences removed.",
            "document_or_input_has_text", {"command_palette", "cleanup_menu"}},
    };
}

QVector<HostActionItem> renderHostActionItems(
    const QString &documentText,
    const TextActionProofInput &input,
    const QString &profile) {
    QVector<HostActionItem> items;
    for (const TextActionRecord &record : textActionRecords()) {
        HostActionItem item;
        item.actionId = record.actionId;
        item.label = record.label;
        item.shortLabel = record.shortLabel;
        item.category = record.category;
        item.icon = record.icon;
        item.tooltip = record.tooltip;
        item.placements = record.placements;
        item.hotkeyLabel = hotkeyLabelForAction(record.actionId, profile);
        item.enabled = isEnabled(record, documentText, input);
        item.disabledReason = disabledReason(record, documentText, input);
        items.push_back(item);
    }
    return items;
}

HostActionResult executeTextActionProof(
    const QString &actionId,
    const QString &documentText,
    const QString &selectedText,
    const TextActionProofInput &input) {
    const TextActionRecord *record = findAction(actionId);
    if (!record) {
        return disabledResult(actionId, "unknown action");
    }
    if (!isEnabled(*record, documentText, input)) {
        return disabledResult(actionId, disabledReason(*record, documentText, input));
    }

    const QString sourceText = selectedTextOrAll(documentText, selectedText);
    HostActionResult result;
    result.actionId = actionId;

    if (actionId == "text.copy_plain") {
        result.kind = "text";
        result.displayText = "Text output ready.";
        result.clipboardText = sourceText;
        return result;
    }
    if (actionId == "text.copy_markdown_block") {
        result.kind = "text";
        result.displayText = "Text output ready.";
        result.clipboardText = fencedBlock(input.language, sourceText);
        return result;
    }
    if (actionId == "text.copy_prompt_block") {
        const QString source = input.source.trimmed().isEmpty() ? QString("unknown") : input.source.trimmed();
        result.kind = "text";
        result.displayText = "Text output ready.";
        result.clipboardText = QString("Source: %1\n\n%2").arg(source, fencedBlock("text", sourceText));
        return result;
    }
    if (actionId == "text.copy_code_fence") {
        result.kind = "clipboard_transform";
        result.displayText = "Clipboard text ready. No changes.";
        result.clipboardText = fencedBlock(input.language, sourceText);
        return result;
    }
    if (actionId == "text.select_all") {
        result.kind = "none";
        result.displayText = "Selection action requested.";
        return result;
    }
    if (actionId == "text.current_line_text") {
        result.kind = "text";
        result.displayText = "Text output ready.";
        result.clipboardText = lineAt(documentText, input.currentLine);
        return result;
    }
    if (actionId == "text.line_range_text") {
        result.kind = "text";
        result.displayText = "Text output ready.";
        result.clipboardText = lineRange(documentText, input.startLine, input.endLine);
        return result;
    }
    if (actionId == "text.trim_trailing_whitespace") {
        result.kind = "text";
        result.displayText = "Text output ready.";
        result.clipboardText = trimTrailingWhitespace(input.explicitText.isEmpty() ? sourceText : input.explicitText);
        return result;
    }

    QString working = input.explicitText.isEmpty() ? sourceText : input.explicitText;
    result.kind = "clipboard_transform";
    if (actionId == "text.clean_basic" || actionId == "text.normalize_line_endings") {
        int count = 0;
        working = normalizeLineEndings(working, &count);
        if (count > 0) {
            result.receipt.changeCount += count;
            result.receipt.changes.push_back(QString("normalized_line_endings: %1").arg(count));
        }
    }
    if (actionId == "text.clean_basic" && input.stripAnsiEscapeCodes) {
        int count = 0;
        QStringList warnings;
        working = stripAnsiEscapeCodes(working, &count, &warnings);
        if (count > 0) {
            result.receipt.changeCount += count;
            result.receipt.changes.push_back(QString("stripped_ansi_escape_codes: %1").arg(count));
        }
        result.receipt.warningCount += warnings.size();
        result.receipt.warnings += warnings;
        result.warnings += warnings;
    }
    if (actionId == "text.strip_ansi_escape_codes") {
        int count = 0;
        QStringList warnings;
        working = stripAnsiEscapeCodes(working, &count, &warnings);
        if (count > 0) {
            result.receipt.changeCount += count;
            result.receipt.changes.push_back(QString("stripped_ansi_escape_codes: %1").arg(count));
        }
        result.receipt.warningCount += warnings.size();
        result.receipt.warnings += warnings;
        result.warnings += warnings;
    }
    if (actionId == "text.clean_basic") {
        int count = 0;
        working = trimTrailingWhitespace(working, &count);
        if (count > 0) {
            result.receipt.changeCount += count;
            result.receipt.changes.push_back(QString("trimmed_trailing_whitespace: %1").arg(count));
        }
    }
    result.clipboardText = working;
    result.displayText = result.receipt.changeCount == 0 && result.receipt.warningCount == 0
        ? QString("Clipboard text ready. No changes.")
        : QString("Clipboard text ready. %1 changes, %2 warnings.")
              .arg(result.receipt.changeCount)
              .arg(result.receipt.warningCount);
    return result;
}

QString selectedTextOrAll(const QString &documentText, const QString &selectedText) {
    return selectedText.isEmpty() ? documentText : selectedText;
}

QString hotkeyLabelForAction(const QString &actionId, const QString &profile) {
    if (profile == "terminal") {
        return actionId == "text.copy_plain" ? QString("Ctrl+Shift+C") : QString();
    }
    if (profile == "macos") {
        if (actionId == "text.copy_plain") {
            return "Cmd+C";
        }
        if (actionId == "text.select_all") {
            return "Cmd+A";
        }
        return {};
    }
    if (actionId == "text.copy_plain") {
        return "Ctrl+C";
    }
    if (actionId == "text.select_all") {
        return "Ctrl+A";
    }
    return {};
}

} // namespace DexTextActions

#pragma once

#include <QString>
#include <QStringList>
#include <QVector>

namespace DexTextActions {

struct TextActionProofInput {
    QString language;
    QString source;
    QString explicitText;
    int currentLine = 0;
    int startLine = -1;
    int endLine = -1;
    bool stripAnsiEscapeCodes = false;
};

struct TextActionRecord {
    QString actionId;
    QString label;
    QString shortLabel;
    QString category;
    QString icon;
    QString tooltip;
    QString enabledRule;
    QStringList placements;
};

struct HostActionItem {
    QString actionId;
    QString label;
    QString shortLabel;
    QString category;
    QString icon;
    QString tooltip;
    QStringList placements;
    QString hotkeyLabel;
    bool enabled = false;
    QString disabledReason;
};

struct ClipboardReceiptSummary {
    int changeCount = 0;
    int warningCount = 0;
    QStringList changes;
    QStringList warnings;
};

struct HostActionResult {
    QString actionId;
    QString kind;
    QString displayText;
    QString clipboardText;
    ClipboardReceiptSummary receipt;
    QStringList warnings;
};

QVector<TextActionRecord> textActionRecords();
QVector<HostActionItem> renderHostActionItems(
    const QString &documentText,
    const TextActionProofInput &input,
    const QString &profile = "linux_desktop");
HostActionResult executeTextActionProof(
    const QString &actionId,
    const QString &documentText,
    const QString &selectedText,
    const TextActionProofInput &input);
QString selectedTextOrAll(const QString &documentText, const QString &selectedText);
QString hotkeyLabelForAction(const QString &actionId, const QString &profile);

} // namespace DexTextActions

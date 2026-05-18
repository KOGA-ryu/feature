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

struct TextActionFixture {
    QString fixtureId;
    QString label;
    QString actionId;
    QString documentText;
    QString selectedText;
    TextActionProofInput input;
    QString expectedClipboardText;
};

struct TextActionFixtureResult {
    TextActionFixture fixture;
    HostActionResult actionResult;
    bool passed = false;
    QString summary;
    QString expectedClipboardText;
    QString actualClipboardText;
};

struct TextActionFixtureSuiteResult {
    QVector<TextActionFixtureResult> results;
    int total = 0;
    int passed = 0;
    int failed = 0;
    bool allPassed = false;
    QString summary;
};

QVector<TextActionRecord> textActionRecords();
QVector<TextActionFixture> textActionFixtures();
QVector<HostActionItem> renderHostActionItems(
    const QString &documentText,
    const TextActionProofInput &input,
    const QString &profile = "linux_desktop");
HostActionResult executeTextActionProof(
    const QString &actionId,
    const QString &documentText,
    const QString &selectedText,
    const TextActionProofInput &input);
TextActionFixtureResult runTextActionFixture(const QString &fixtureId);
TextActionFixtureSuiteResult runAllTextActionFixtures();
QString selectedTextOrAll(const QString &documentText, const QString &selectedText);
QString hotkeyLabelForAction(const QString &actionId, const QString &profile);

} // namespace DexTextActions

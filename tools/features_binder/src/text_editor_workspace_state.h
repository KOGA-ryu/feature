#pragma once

#include <QObject>
#include <QString>
#include <QStringList>

namespace DexTextEditorWorkspace {

struct TextEditorDisplaySnapshot {
    QString documentName = "scratch.txt";
    int lineCount = 1;
    int characterCount = 0;
    QString cursorSummary = "Ln 1, Col 1";
    QString selectionSummary = "selection: none";
    int visibleBlockStart = 1;
    int visibleBlockEnd = 1;
    int verticalScrollValue = 0;
    int verticalScrollMaximum = 0;
    QString activeActionId = "none";
    QString activeActionState = "default";
    QString lastResultSummary = "No action has run.";
    QString lastReceiptSummary = "No receipt yet.";
};

struct TextEditorWorkspaceState {
    QString detailLens = "Dashboard";
    QString focusedSurface = "Editor";
    QString documentName = "scratch.txt";
    int documentLines = 1;
    int documentCharacters = 0;
    QString cursorSummary = "Ln 1, Col 1";
    QString selectionSummary = "selection: none";
    QString language = "text";
    QString source = "features_binder";
    int startLine = 0;
    int endLine = 0;
    bool stripAnsiEscapeCodes = false;
    int actionCount = 0;
    int fixtureCount = 0;
    QString lastActionId = "none";
    QString lastActionKind = "none";
    QString lastResultSummary = "No action has run.";
    QString lastReceiptSummary = "No receipt yet.";
    QString fixtureStatus = "No fixture run yet.";
    QString componentState = "empty";
    TextEditorDisplaySnapshot snapshot;
};

class TextEditorWorkspaceController final : public QObject {
    Q_OBJECT

public:
    explicit TextEditorWorkspaceController(QObject *parent = nullptr);

    const TextEditorWorkspaceState &state() const;
    QStringList requiredUiPaths() const;

    void setDetailLens(const QString &detailLens);
    void setFocusedSurface(const QString &focusedSurface);
    void setActionInventory(int actionCount, int fixtureCount);
    void setDocumentFacts(
        const QString &documentName,
        int documentLines,
        int documentCharacters,
        const QString &cursorSummary,
        const QString &selectionSummary);
    void setEditorSnapshot(const TextEditorDisplaySnapshot &snapshot);
    void setActionInput(
        const QString &language,
        const QString &source,
        int startLine,
        int endLine,
        bool stripAnsiEscapeCodes);
    void setLastResult(
        const QString &actionId,
        const QString &kind,
        const QString &resultSummary,
        const QString &receiptSummary,
        const QString &componentState);
    void setFixtureStatus(const QString &fixtureStatus, const QString &componentState);
    void requestCommandPalette();

signals:
    void actionInputChanged();
    void documentStateChanged();
    void resultStateChanged();
    void workspaceStateChanged();
    void commandPaletteRequested();

private:
    TextEditorWorkspaceState state_;
};

QStringList requiredTextEditorUiPaths();

} // namespace DexTextEditorWorkspace

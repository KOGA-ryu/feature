#include "text_editor_workspace_state.h"

#include <algorithm>

namespace DexTextEditorWorkspace {

namespace {

QString normalizedState(const QString &value, const QString &fallback) {
    const QString trimmed = value.trimmed();
    return trimmed.isEmpty() ? fallback : trimmed;
}

} // namespace

TextEditorWorkspaceController::TextEditorWorkspaceController(QObject *parent)
    : QObject(parent) {
}

const TextEditorWorkspaceState &TextEditorWorkspaceController::state() const {
    return state_;
}

QStringList TextEditorWorkspaceController::requiredUiPaths() const {
    return requiredTextEditorUiPaths();
}

void TextEditorWorkspaceController::setDetailLens(const QString &detailLens) {
    const QString normalized = normalizedState(detailLens, "Dashboard");
    if (state_.detailLens == normalized) {
        return;
    }
    state_.detailLens = normalized;
    emit workspaceStateChanged();
}

void TextEditorWorkspaceController::setFocusedSurface(const QString &focusedSurface) {
    const QString normalized = normalizedState(focusedSurface, "Editor");
    if (state_.focusedSurface == normalized) {
        return;
    }
    state_.focusedSurface = normalized;
    emit documentStateChanged();
}

void TextEditorWorkspaceController::setActionInventory(int actionCount, int fixtureCount) {
    actionCount = std::max(0, actionCount);
    fixtureCount = std::max(0, fixtureCount);
    if (state_.actionCount == actionCount && state_.fixtureCount == fixtureCount) {
        return;
    }
    state_.actionCount = actionCount;
    state_.fixtureCount = fixtureCount;
    emit workspaceStateChanged();
}

void TextEditorWorkspaceController::setDocumentFacts(
    const QString &documentName,
    int documentLines,
    int documentCharacters,
    const QString &cursorSummary,
    const QString &selectionSummary) {
    documentLines = std::max(1, documentLines);
    documentCharacters = std::max(0, documentCharacters);
    const QString normalizedName = normalizedState(documentName, "scratch.txt");
    const QString normalizedCursor = normalizedState(cursorSummary, "Ln 1, Col 1");
    const QString normalizedSelection = normalizedState(selectionSummary, "selection: none");
    if (state_.documentName == normalizedName
        && state_.documentLines == documentLines
        && state_.documentCharacters == documentCharacters
        && state_.cursorSummary == normalizedCursor
        && state_.selectionSummary == normalizedSelection) {
        return;
    }
    state_.documentName = normalizedName;
    state_.documentLines = documentLines;
    state_.documentCharacters = documentCharacters;
    state_.cursorSummary = normalizedCursor;
    state_.selectionSummary = normalizedSelection;
    emit documentStateChanged();
}

void TextEditorWorkspaceController::setActionInput(
    const QString &language,
    const QString &source,
    int startLine,
    int endLine,
    bool stripAnsiEscapeCodes) {
    startLine = std::max(0, startLine);
    endLine = std::max(0, endLine);
    const QString normalizedLanguage = normalizedState(language, "text");
    const QString normalizedSource = normalizedState(source, "features_binder");
    if (state_.language == normalizedLanguage
        && state_.source == normalizedSource
        && state_.startLine == startLine
        && state_.endLine == endLine
        && state_.stripAnsiEscapeCodes == stripAnsiEscapeCodes) {
        return;
    }
    state_.language = normalizedLanguage;
    state_.source = normalizedSource;
    state_.startLine = startLine;
    state_.endLine = endLine;
    state_.stripAnsiEscapeCodes = stripAnsiEscapeCodes;
    emit actionInputChanged();
}

void TextEditorWorkspaceController::setLastResult(
    const QString &actionId,
    const QString &kind,
    const QString &resultSummary,
    const QString &receiptSummary,
    const QString &componentState) {
    state_.lastActionId = normalizedState(actionId, "none");
    state_.lastActionKind = normalizedState(kind, "none");
    state_.lastResultSummary = normalizedState(resultSummary, "No action has run.");
    state_.lastReceiptSummary = normalizedState(receiptSummary, "No receipt yet.");
    state_.componentState = normalizedState(componentState, "empty");
    emit resultStateChanged();
}

void TextEditorWorkspaceController::setFixtureStatus(const QString &fixtureStatus, const QString &componentState) {
    state_.fixtureStatus = normalizedState(fixtureStatus, "No fixture run yet.");
    state_.componentState = normalizedState(componentState, "empty");
    emit resultStateChanged();
}

QStringList requiredTextEditorUiPaths() {
    return {
        "workbench.rail.text_editor.documents",
        "workbench.rail.text_editor.clipboard",
        "workbench.rail.text_editor.drafts",
        "workbench.rail.text_editor.fixtures",
        "workbench.toolbar.primary",
        "workbench.editor.surface.document",
        "workbench.editor.surface.text",
        "workbench.editor.status.cursor_position",
        "workbench.inspector.text_editor.options",
        "workbench.inspector.text_editor.context",
        "workbench.inspector.text_editor.result",
        "workbench.inspector.text_editor.receipts",
        "workbench.fixture_bench",
        "workbench.fixture_bench.runner",
        "workbench.fixture_bench.results.expected",
        "workbench.fixture_bench.results.actual",
    };
}

} // namespace DexTextEditorWorkspace

#pragma once

#include <QJsonObject>
#include <QString>
#include <QStringList>

#include "text_action_proof_model.h"

namespace DexTextEditorRust {

struct Position {
    int line = 0;
    int column = 0;
};

struct Selection {
    Position anchor;
    Position caret;
    bool valid = false;
};

struct ActionRequest {
    QString actionId;
    QString documentText;
    Selection selection;
    DexTextActions::TextActionProofInput input;
};

struct ReceiptSummary {
    int changeCount = 0;
    int warningCount = 0;
    QStringList changes;
    QStringList warnings;
};

struct ActionResult {
    bool ok = false;
    QString error;
    QString actionId;
    QString kind;
    QString displayText;
    QString clipboardText;
    bool hasClipboardText = false;
    ReceiptSummary receipt;
    QStringList warnings;
    QString editorText;
    Selection selection;
};

QString resolveTextEditorActionRunnerPath();
QJsonObject requestToJson(const ActionRequest &request);
QVector<DexTextActions::HostActionItem> parseActionRunnerActions(const QByteArray &payload, QString *error = nullptr);
ActionResult parseActionRunnerResponse(const QByteArray &payload);
QVector<DexTextActions::HostActionItem> renderActionsWithRunner(
    const QString &documentText,
    const DexTextActions::TextActionProofInput &input,
    QString *error = nullptr);
ActionResult executeActionWithRunner(const ActionRequest &request);

} // namespace DexTextEditorRust

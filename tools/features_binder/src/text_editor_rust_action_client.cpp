#include "text_editor_rust_action_client.h"

#include <QCoreApplication>
#include <QDir>
#include <QFileInfo>
#include <QJsonArray>
#include <QJsonDocument>
#include <QProcess>
#include <QProcessEnvironment>

namespace DexTextEditorRust {
namespace {

QJsonObject positionToJson(const Position &position) {
    return QJsonObject{
        {"line", position.line},
        {"column", position.column},
    };
}

QJsonObject selectionToJson(const Selection &selection) {
    return QJsonObject{
        {"anchor", positionToJson(selection.anchor)},
        {"caret", positionToJson(selection.caret)},
    };
}

Position parsePosition(const QJsonObject &object) {
    return Position{
        object.value("line").toInt(0),
        object.value("column").toInt(0),
    };
}

Selection parseSelection(const QJsonObject &object) {
    Selection selection;
    selection.anchor = parsePosition(object.value("anchor").toObject());
    selection.caret = parsePosition(object.value("caret").toObject());
    selection.valid = true;
    return selection;
}

PayloadMetadata parsePayloadMetadata(const QJsonValue &value) {
    PayloadMetadata metadata;
    if (!value.isObject()) {
        return metadata;
    }

    const QJsonObject object = value.toObject();
    metadata.payloadKind = object.value("payload_kind").toString();
    metadata.exportPolicy = object.value("export_policy").toString();
    metadata.usedSelection = object.value("used_selection").toBool(false);
    metadata.fallbackToFullDocument = object.value("fallback_to_full_document").toBool(false);
    metadata.characterCount = object.value("character_count").toInt(0);
    metadata.lineCount = object.value("line_count").toInt(0);
    metadata.valid = true;
    return metadata;
}

QStringList stringArray(const QJsonValue &value) {
    QStringList output;
    for (const QJsonValue &entry : value.toArray()) {
        output.push_back(entry.toString());
    }
    return output;
}

DexTextActions::HostActionItem hostActionItemFromJson(const QJsonObject &object) {
    DexTextActions::HostActionItem item;
    item.actionId = object.value("action_id").toString();
    item.label = object.value("label").toString();
    item.shortLabel = object.value("short_label").toString();
    item.category = object.value("category").toString();
    item.icon = object.value("icon").toString();
    item.tooltip = object.value("tooltip").toString();
    item.placements = stringArray(object.value("placements"));
    item.hotkeyLabel = object.value("hotkey_label").toString();
    item.enabled = object.value("enabled").toBool(false);
    item.disabledReason = object.value("disabled_reason").toString();
    return item;
}

QJsonObject inputToJson(const DexTextActions::TextActionProofInput &input) {
    QJsonObject object;
    if (!input.language.trimmed().isEmpty()) {
        object.insert("language", input.language);
    }
    if (!input.source.trimmed().isEmpty()) {
        object.insert("source", input.source);
    }
    if (!input.explicitText.isEmpty()) {
        object.insert("text", input.explicitText);
    }
    if (input.startLine >= 0) {
        object.insert("start_line", input.startLine);
    }
    if (input.endLine >= 0) {
        object.insert("end_line", input.endLine);
    }
    object.insert("strip_ansi_escape_codes", input.stripAnsiEscapeCodes);
    return object;
}

ActionResult unavailableResult(const QString &message) {
    ActionResult result;
    result.ok = false;
    result.error = message;
    result.displayText = "Rust action runner unavailable: " + message;
    return result;
}

QByteArray runActionRunner(const QJsonObject &request, QString *error) {
    const QString runnerPath = resolveTextEditorActionRunnerPath();
    if (!QFileInfo::exists(runnerPath)) {
        if (error) {
            *error = "missing runner at " + runnerPath;
        }
        return {};
    }

    QProcess process;
    process.start(runnerPath);
    if (!process.waitForStarted(3000)) {
        if (error) {
            *error = "failed to start runner at " + runnerPath;
        }
        return {};
    }

    process.write(QJsonDocument(request).toJson(QJsonDocument::Compact));
    process.closeWriteChannel();

    if (!process.waitForFinished(8000)) {
        process.kill();
        process.waitForFinished(1000);
        if (error) {
            *error = "runner timed out";
        }
        return {};
    }

    if (process.exitStatus() != QProcess::NormalExit || process.exitCode() != 0) {
        const QString stderrText = QString::fromUtf8(process.readAllStandardError()).trimmed();
        if (error) {
            *error = stderrText.isEmpty() ? "runner exited with failure" : stderrText;
        }
        return {};
    }

    return process.readAllStandardOutput();
}

} // namespace

QString resolveTextEditorActionRunnerPath() {
    const QString envPath = QProcessEnvironment::systemEnvironment().value("TEXT_EDITOR_ACTION_RUNNER");
    if (!envPath.isEmpty() && QFileInfo::exists(envPath)) {
        return envPath;
    }

#ifdef DEX_FEATURES_REPO_ROOT
    const QString compiledPath =
        QString::fromUtf8(DEX_FEATURES_REPO_ROOT) + "/target/debug/text_editor_action_runner";
    if (QFileInfo::exists(compiledPath)) {
        return compiledPath;
    }
#endif

    const QString appRelativePath =
        QDir(QCoreApplication::applicationDirPath()).absoluteFilePath("../../../target/debug/text_editor_action_runner");
    if (QFileInfo::exists(appRelativePath)) {
        return QFileInfo(appRelativePath).canonicalFilePath();
    }

    return envPath.isEmpty() ? appRelativePath : envPath;
}

QJsonObject requestToJson(const ActionRequest &request) {
    QJsonObject object{
        {"mode", "execute_action"},
        {"action_id", request.actionId},
        {"document_text", request.documentText},
        {"input", inputToJson(request.input)},
    };
    if (request.selection.valid) {
        object.insert("selection", selectionToJson(request.selection));
    }
    return object;
}

QVector<DexTextActions::HostActionItem> parseActionRunnerActions(const QByteArray &payload, QString *error) {
    QJsonParseError parseError;
    const QJsonDocument document = QJsonDocument::fromJson(payload, &parseError);
    if (parseError.error != QJsonParseError::NoError || !document.isObject()) {
        if (error) {
            *error = "malformed runner json: " + parseError.errorString();
        }
        return {};
    }

    const QJsonObject root = document.object();
    if (!root.value("ok").toBool(false)) {
        if (error) {
            *error = root.value("error").toString("runner returned ok=false");
        }
        return {};
    }

    QVector<DexTextActions::HostActionItem> actions;
    for (const QJsonValue &entry : root.value("actions").toArray()) {
        if (entry.isObject()) {
            actions.push_back(hostActionItemFromJson(entry.toObject()));
        }
    }
    return actions;
}

ActionResult parseActionRunnerResponse(const QByteArray &payload) {
    QJsonParseError parseError;
    const QJsonDocument document = QJsonDocument::fromJson(payload, &parseError);
    if (parseError.error != QJsonParseError::NoError || !document.isObject()) {
        return unavailableResult("malformed runner json: " + parseError.errorString());
    }

    const QJsonObject root = document.object();
    ActionResult output;
    output.ok = root.value("ok").toBool(false);
    output.error = root.value("error").toString();
    output.editorText = root.value("editor_text").toString();
    output.selection = parseSelection(root.value("selection").toObject());

    const QJsonObject result = root.value("result").toObject();
    output.actionId = result.value("action_id").toString();
    output.kind = result.value("kind").toString();
    output.displayText = result.value("display_text").toString();
    if (result.contains("clipboard_text") && !result.value("clipboard_text").isNull()) {
        output.clipboardText = result.value("clipboard_text").toString();
        output.hasClipboardText = true;
    }
    output.payloadMetadata = parsePayloadMetadata(result.value("payload_metadata"));
    output.warnings = stringArray(result.value("warnings"));

    const QJsonObject receipt = result.value("receipt_summary").toObject();
    output.receipt.changeCount = receipt.value("change_count").toInt(0);
    output.receipt.warningCount = receipt.value("warning_count").toInt(0);
    output.receipt.changes = stringArray(receipt.value("changes"));
    output.receipt.warnings = stringArray(receipt.value("warnings"));

    if (!output.ok && output.error.isEmpty()) {
        output.error = "runner returned ok=false";
    }
    return output;
}

QString payloadMetadataSummary(const PayloadMetadata &metadata) {
    if (!metadata.valid) {
        return {};
    }

    const QString source = metadata.usedSelection
        ? QString("selected text")
        : (metadata.fallbackToFullDocument ? QString("full document fallback") : QString("full document"));

    return QStringList{
        "payload metadata:",
        "  kind: " + metadata.payloadKind,
        "  policy: " + metadata.exportPolicy,
        "  source: " + source,
        QString("  characters: %1").arg(metadata.characterCount),
        QString("  lines: %1").arg(metadata.lineCount),
        QString("  fallback_to_full_document: %1").arg(metadata.fallbackToFullDocument ? "true" : "false"),
    }.join('\n');
}

QVector<DexTextActions::HostActionItem> renderActionsWithRunner(
    const QString &documentText,
    const DexTextActions::TextActionProofInput &input,
    QString *error) {
    const QJsonObject request{
        {"mode", "render_host_actions"},
        {"document_text", documentText},
        {"input", inputToJson(input)},
        {"profile", "linux_desktop"},
    };

    const QByteArray payload = runActionRunner(request, error);
    if (payload.isEmpty()) {
        return {};
    }

    return parseActionRunnerActions(payload, error);
}

ActionResult executeActionWithRunner(const ActionRequest &request) {
    QString error;
    const QByteArray payload = runActionRunner(requestToJson(request), &error);
    if (payload.isEmpty()) {
        return unavailableResult(error);
    }

    return parseActionRunnerResponse(payload);
}

} // namespace DexTextEditorRust

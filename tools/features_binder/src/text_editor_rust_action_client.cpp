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

QStringList stringArray(const QJsonValue &value) {
    QStringList output;
    for (const QJsonValue &entry : value.toArray()) {
        output.push_back(entry.toString());
    }
    return output;
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
        {"action_id", request.actionId},
        {"document_text", request.documentText},
        {"input", inputToJson(request.input)},
    };
    if (request.selection.valid) {
        object.insert("selection", selectionToJson(request.selection));
    }
    return object;
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

ActionResult executeActionWithRunner(const ActionRequest &request) {
    const QString runnerPath = resolveTextEditorActionRunnerPath();
    if (!QFileInfo::exists(runnerPath)) {
        return unavailableResult("missing runner at " + runnerPath);
    }

    QProcess process;
    process.start(runnerPath);
    if (!process.waitForStarted(3000)) {
        return unavailableResult("failed to start runner at " + runnerPath);
    }

    const QByteArray payload = QJsonDocument(requestToJson(request)).toJson(QJsonDocument::Compact);
    process.write(payload);
    process.closeWriteChannel();

    if (!process.waitForFinished(8000)) {
        process.kill();
        process.waitForFinished(1000);
        return unavailableResult("runner timed out");
    }
    if (process.exitStatus() != QProcess::NormalExit || process.exitCode() != 0) {
        const QString stderrText = QString::fromUtf8(process.readAllStandardError()).trimmed();
        return unavailableResult(stderrText.isEmpty() ? "runner exited with failure" : stderrText);
    }

    return parseActionRunnerResponse(process.readAllStandardOutput());
}

} // namespace DexTextEditorRust

#include "repo_proof_receipt_state.h"

#include <QFile>
#include <QJsonArray>
#include <QJsonObject>
#include <QJsonParseError>

namespace DexRepoProofReceipt {

QString readStringValue(const QJsonObject &object, const QString &key, const QString &fallback = QString("unknown")) {
    const QJsonValue value = object.value(key);
    return value.isString() ? value.toString() : fallback;
}

int readIntValue(const QJsonObject &object, const QString &key, int fallback = -1) {
    const QJsonValue value = object.value(key);
    return value.isDouble() ? value.toInt() : fallback;
}

RepoProofReceiptState unavailableProofReceipt(const QString &reason) {
    RepoProofReceiptState state;
    state.receiptStatus = "unavailable";
    if (!reason.isEmpty()) {
        state.warnings.push_back(reason);
    }
    state.loaded = false;
    return state;
}

RepoProofReceiptState parseRepoProofReceiptPayload(const QByteArray &payload) {
    QJsonParseError parseError;
    const QJsonDocument document = QJsonDocument::fromJson(payload, &parseError);
    if (parseError.error != QJsonParseError::NoError) {
        return unavailableProofReceipt(QString("Malformed proof receipt JSON: %1").arg(parseError.errorString()));
    }

    const QJsonObject root = document.object();
    if (root.isEmpty()) {
        return unavailableProofReceipt("Malformed proof receipt JSON: expected object");
    }

    RepoProofReceiptState state;
    state.proofType = readStringValue(root, "proof_type", "unknown");
    state.runId = readStringValue(root, "run_id", "unknown");
    state.createdAt = readStringValue(root, "created_at", "unknown");
    state.workcopyRoot = readStringValue(root, "workcopy_root", "");
    state.repoRoot = readStringValue(root, "repo_root", "");
    state.projectId = readStringValue(root, "project_id", "unknown");
    state.tab = readStringValue(root, "tab", "unknown");
    state.detailLens = readStringValue(root, "detail_lens", "unknown");
    state.receiptStatus = readStringValue(root, "receipt_status", "attached");
    state.latestManifestPath = readStringValue(root, "latest_manifest_path", "");
    state.runManifestPath = readStringValue(root, "run_manifest_path", "");
    state.loaded = true;

    const QJsonArray screenshots = root.value("screenshots").toArray();
    for (const QJsonValue &value : screenshots) {
        ProofScreenshotRecord screenshot;
        if (value.isString()) {
            screenshot.path = value.toString();
            screenshot.status = "attached";
        } else if (value.isObject()) {
            const QJsonObject object = value.toObject();
            screenshot.path = readStringValue(object, "path", "unknown");
            screenshot.viewport = readStringValue(object, "viewport", "unknown");
            screenshot.subject = readStringValue(object, "subject", "unknown");
            screenshot.status = readStringValue(object, "status", "unknown");
        } else {
            continue;
        }
        state.screenshots.push_back(screenshot);
    }

    const QJsonArray receipts = root.value("command_receipts").toArray();
    for (const QJsonValue &value : receipts) {
        if (!value.isObject()) {
            continue;
        }
        const QJsonObject object = value.toObject();
        CommandReceiptRecord receipt;
        receipt.receiptId = readStringValue(object, "receipt_id", "unknown");
        receipt.command = readStringValue(object, "command", "unknown");
        receipt.status = readStringValue(object, "status", "unknown");
        receipt.exitCode = readIntValue(object, "exit_code", -1);
        receipt.logPath = readStringValue(object, "log_path", "");
        state.commandReceipts.push_back(receipt);
    }

    const QJsonArray warnings = root.value("warnings").toArray();
    for (const QJsonValue &value : warnings) {
        if (value.isString()) {
            state.warnings.push_back(value.toString());
        }
    }

    return state;
}

RepoProofReceiptState parseRepoProofReceiptDocument(const QJsonDocument &document) {
    if (document.isNull() || document.isEmpty()) {
        return unavailableProofReceipt("Malformed proof receipt JSON: empty payload");
    }
    return parseRepoProofReceiptPayload(QByteArray(QJsonDocument(document).toJson(QJsonDocument::Compact)));
}

RepoProofReceiptState loadRepoProofReceiptFile(const QString &path) {
    QFile file(path);
    if (!file.exists()) {
        return unavailableProofReceipt(QString("proof receipt not found: %1").arg(path));
    }
    if (!file.open(QIODevice::ReadOnly)) {
        return unavailableProofReceipt(QString("failed to read proof receipt: %1").arg(path));
    }
    return parseRepoProofReceiptPayload(file.readAll());
}

} // namespace DexRepoProofReceipt

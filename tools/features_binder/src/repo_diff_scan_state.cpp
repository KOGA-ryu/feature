#include "repo_diff_scan_state.h"

#include <QFile>
#include <QJsonArray>
#include <QJsonObject>
#include <QJsonParseError>

namespace DexRepoDiffScan {

QString readStringValue(const QJsonObject &object, const QString &key, const QString &fallback = QString("unknown")) {
    const QJsonValue value = object.value(key);
    return value.isString() ? value.toString() : fallback;
}

bool isUnknown(const QString &value) {
    return value.isEmpty() || value == "unknown";
}

QString readUnknownFallback(const QJsonValue &value, const QString &fallback = QString("unknown")) {
    if (value.isString()) {
        return value.toString();
    }
    if (value.isBool()) {
        return value.toBool() ? QString("true") : QString("false");
    }
    return fallback;
}

QStringList readStringListValue(const QJsonObject &object, const QString &key) {
    QStringList values;
    const QJsonValue value = object.value(key);
    if (!value.isArray()) {
        return values;
    }
    for (const QJsonValue &item : value.toArray()) {
        if (item.isString()) {
            values.push_back(item.toString());
        }
    }
    return values;
}

QStringList readStringListValue(const QJsonValue &value) {
    QStringList values;
    if (!value.isArray()) {
        return values;
    }
    for (const QJsonValue &item : value.toArray()) {
        if (item.isString()) {
            values.push_back(item.toString());
        }
    }
    return values;
}

QStringList gatherBoundaryPaths(const QJsonObject &root, const QStringList &keys) {
    for (const QString &key : keys) {
        const QStringList paths = readStringListValue(root, key);
        if (!paths.isEmpty()) {
            return paths;
        }
    }
    return {};
}

RepoDiffScanState unavailableScanState(const QString &reason) {
    RepoDiffScanState state;
    state.scanStatus = "unavailable";
    state.riskLevel = "unknown";
    state.recommendedNextStep = "unknown";
    if (!reason.isEmpty()) {
        state.warnings.push_back(reason);
    }
    state.available = false;
    return state;
}

RepoDiffScanState parseRepoDiffScanPayload(const QByteArray &payload) {
    QJsonParseError parseError;
    const QJsonDocument document = QJsonDocument::fromJson(payload, &parseError);
    if (parseError.error != QJsonParseError::NoError) {
        return unavailableScanState(QString("Malformed scanner JSON: %1").arg(parseError.errorString()));
    }

    RepoDiffScanState state;
    const QJsonObject root = document.object();
    if (root.isEmpty()) {
        return unavailableScanState("Malformed scanner JSON: expected object");
    }

    state.scanStatus = readStringValue(root, "scan_status", "unknown");
    state.riskLevel = readStringValue(root, "risk_level", "unknown");
    state.recommendedNextStep = readStringValue(root, "recommended_next_step", "unknown");

    state.warnings = readStringListValue(root.value("warnings"));
    state.testObligations = readStringListValue(root.value("test_obligations"));
    state.proofObligations = readStringListValue(root.value("proof_obligations"));
    state.protectedBoundaryPaths = gatherBoundaryPaths(root, {"protected_touched", "protected_boundaries", "protected_boundary_paths", "protected"});
    if (state.protectedBoundaryPaths.isEmpty()) {
        state.protectedBoundaryPaths = gatherBoundaryPaths(root, {"protected_file_paths", "protected_paths"});
    }
    state.generatedBoundaryPaths = gatherBoundaryPaths(root, {"generated_touched", "generated_boundaries", "generated_boundary_paths", "generated"});
    if (state.generatedBoundaryPaths.isEmpty()) {
        state.generatedBoundaryPaths = gatherBoundaryPaths(root, {"generated_file_paths", "generated_paths"});
    }
    state.unknownBoundaryPaths = gatherBoundaryPaths(root, {"unknown_zone_touched", "unknown_boundaries", "unknown_boundary_paths", "unknown"});

    const QJsonArray changedFiles = root.value("changed_files").toArray();
    for (const QJsonValue &value : changedFiles) {
        if (value.isObject()) {
            const QJsonObject changed = value.toObject();
            ChangedFileRecord row;
            row.path = readStringValue(changed, "path", "unknown");
            row.boundary = readStringValue(changed, "boundary", "unknown");
            row.changeType = readStringValue(changed, "change_type", readUnknownFallback(changed.value("status")));
            row.riskLevel = readStringValue(changed, "risk_level", readUnknownFallback(changed.value("risk")));
            state.changedFiles.push_back(row);
            continue;
        }
        if (value.isString()) {
            state.changedFiles.push_back({value.toString(), "unknown", "unknown", "unknown"});
        }
    }
    state.available = (state.scanStatus == "available" || state.scanStatus == "ok");
    return state;
}

RepoDiffScanState parseRepoDiffScanDocument(const QJsonDocument &document) {
    if (document.isNull() || document.isEmpty()) {
        return unavailableScanState("Malformed scanner JSON: empty payload");
    }
    return parseRepoDiffScanPayload(QByteArray(QJsonDocument(document).toJson(QJsonDocument::Compact)));
}

} // namespace DexRepoDiffScan

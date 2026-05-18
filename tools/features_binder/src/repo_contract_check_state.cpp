#include "repo_contract_check_state.h"

#include <QJsonArray>
#include <QJsonObject>
#include <QJsonParseError>

namespace DexRepoContractCheck {

QString readStringValue(const QJsonObject &object, const QString &key, const QString &fallback = QString("unknown")) {
    const QJsonValue value = object.value(key);
    return value.isString() ? value.toString() : fallback;
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

RepoContractCheckState unavailableContractState(const QString &reason) {
    RepoContractCheckState state;
    state.scanStatus = "unavailable";
    state.riskLevel = "unknown";
    state.contractStatus = "unavailable";
    state.recommendedNextStep = "resolve_scan_unavailable";
    if (!reason.isEmpty()) {
        state.warnings.push_back(reason);
    }
    state.available = false;
    return state;
}

RepoContractCheckState parseRepoContractCheckPayload(const QByteArray &payload) {
    QJsonParseError parseError;
    const QJsonDocument document = QJsonDocument::fromJson(payload, &parseError);
    if (parseError.error != QJsonParseError::NoError) {
        return unavailableContractState(QString("Malformed contract-check JSON: %1").arg(parseError.errorString()));
    }

    const QJsonObject root = document.object();
    if (root.isEmpty()) {
        return unavailableContractState("Malformed contract-check JSON: expected object");
    }

    RepoContractCheckState state;
    state.scanStatus = readStringValue(root, "scan_status", "unknown");
    state.riskLevel = readStringValue(root, "risk_level", "unknown");
    state.contractStatus = readStringValue(root, "contract_status", "unknown");
    state.recommendedNextStep = readStringValue(root, "recommended_next_step", "unknown");
    state.warnings = readStringListValue(root.value("warnings"));

    const QJsonArray violations = root.value("violations").toArray();
    for (const QJsonValue &value : violations) {
        if (!value.isObject()) {
            continue;
        }
        const QJsonObject object = value.toObject();
        ContractViolationRecord violation;
        violation.contractId = readStringValue(object, "contract_id", "unknown");
        violation.rule = readStringValue(object, "rule", "unknown");
        violation.severity = readStringValue(object, "severity", "unknown");
        violation.status = readStringValue(object, "status", "unknown");
        violation.detectionMethod = readStringValue(object, "detection_method", "unknown");
        violation.paths = readStringListValue(object.value("paths"));
        violation.requiredAction = readStringValue(object, "required_action", "unknown");
        state.violations.push_back(violation);
    }

    state.available = (state.scanStatus == "ok" || state.scanStatus == "available")
        && state.contractStatus != "unavailable";
    return state;
}

RepoContractCheckState parseRepoContractCheckDocument(const QJsonDocument &document) {
    if (document.isNull() || document.isEmpty()) {
        return unavailableContractState("Malformed contract-check JSON: empty payload");
    }
    return parseRepoContractCheckPayload(QByteArray(QJsonDocument(document).toJson(QJsonDocument::Compact)));
}

} // namespace DexRepoContractCheck

#include "repo_promotion_report_state.h"

#include <QFile>
#include <QJsonArray>
#include <QJsonObject>
#include <QJsonParseError>

namespace DexRepoPromotionReport {

QString readStringValue(const QJsonObject &object, const QString &key, const QString &fallback = QString("unknown")) {
    const QJsonValue value = object.value(key);
    return value.isString() ? value.toString() : fallback;
}

bool readBoolValue(const QJsonObject &object, const QString &key, bool fallback = false) {
    const QJsonValue value = object.value(key);
    return value.isBool() ? value.toBool() : fallback;
}

int readIntValue(const QJsonObject &object, const QString &key, int fallback = 0) {
    const QJsonValue value = object.value(key);
    return value.isDouble() ? value.toInt() : fallback;
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

RepoPromotionReportState unavailablePromotionReport(const QString &reason) {
    RepoPromotionReportState state;
    state.mode = "unavailable";
    if (!reason.isEmpty()) {
        state.warnings.push_back(reason);
    }
    state.loaded = false;
    return state;
}

RepoPromotionReportState parseRepoPromotionReportPayload(const QByteArray &payload) {
    QJsonParseError parseError;
    const QJsonDocument document = QJsonDocument::fromJson(payload, &parseError);
    if (parseError.error != QJsonParseError::NoError) {
        return unavailablePromotionReport(QString("Malformed promotion report JSON: %1").arg(parseError.errorString()));
    }

    const QJsonObject root = document.object();
    if (root.isEmpty()) {
        return unavailablePromotionReport("Malformed promotion report JSON: expected object");
    }

    RepoPromotionReportState state;
    state.createdAt = readStringValue(root, "created_at", "unknown");
    state.mode = readStringValue(root, "mode", "unknown");
    state.dryRunOnly = readBoolValue(root, "dry_run_only", true);
    state.workcopyRoot = readStringValue(root, "workcopy_root", "");
    state.authorityRoot = readStringValue(root, "authority_root", "");

    const QJsonObject summary = root.value("summary").toObject();
    state.comparedCount = readIntValue(summary, "compared_count", 0);
    state.blockedCount = readIntValue(summary, "blocked_count", 0);

    const QJsonArray comparedRows = root.value("compared").toArray();
    for (const QJsonValue &value : comparedRows) {
        if (!value.isObject()) {
            continue;
        }
        const QJsonObject object = value.toObject();
        PromotionComparedFile file;
        file.source = readStringValue(object, "source", "unknown");
        file.destination = readStringValue(object, "destination", "unknown");
        file.status = readStringValue(object, "status", "unknown");
        state.compared.push_back(file);
    }

    const QJsonArray blockedRows = root.value("blocked").toArray();
    for (const QJsonValue &value : blockedRows) {
        if (!value.isObject()) {
            continue;
        }
        const QJsonObject object = value.toObject();
        PromotionBlockedFile file;
        file.requestedFile = readStringValue(object, "requested_file", "unknown");
        file.reason = readStringValue(object, "reason", "unknown");
        state.blocked.push_back(file);
    }

    const QJsonObject policy = root.value("policy").toObject();
    state.allowedTargets = readStringListValue(policy.value("allowed_targets"));
    state.blockedTargets = readStringListValue(policy.value("blocked_targets"));
    state.authorityWrites = readStringValue(policy, "authority_writes", "unknown");
    state.promotionRequires = readStringValue(policy, "promotion_requires", "unknown");

    if (state.comparedCount == 0 && !state.compared.isEmpty()) {
        state.comparedCount = state.compared.size();
    }
    if (state.blockedCount == 0 && !state.blocked.isEmpty()) {
        state.blockedCount = state.blocked.size();
    }

    state.loaded = true;
    return state;
}

RepoPromotionReportState parseRepoPromotionReportDocument(const QJsonDocument &document) {
    if (document.isNull() || document.isEmpty()) {
        return unavailablePromotionReport("Malformed promotion report JSON: empty payload");
    }
    return parseRepoPromotionReportPayload(QByteArray(QJsonDocument(document).toJson(QJsonDocument::Compact)));
}

RepoPromotionReportState loadRepoPromotionReportFile(const QString &path) {
    QFile file(path);
    if (!file.exists()) {
        return unavailablePromotionReport(QString("promotion report not found: %1").arg(path));
    }
    if (!file.open(QIODevice::ReadOnly)) {
        return unavailablePromotionReport(QString("failed to read promotion report: %1").arg(path));
    }
    return parseRepoPromotionReportPayload(file.readAll());
}

} // namespace DexRepoPromotionReport

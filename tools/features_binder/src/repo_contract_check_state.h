#pragma once

#include <QByteArray>
#include <QJsonDocument>
#include <QString>
#include <QStringList>
#include <QVector>

namespace DexRepoContractCheck {

struct ContractViolationRecord {
    QString contractId;
    QString rule;
    QString severity;
    QString status;
    QString detectionMethod;
    QStringList paths;
    QString requiredAction;
};

struct RepoContractCheckState {
    QString scanStatus = "unavailable";
    QString riskLevel = "unknown";
    QString contractStatus = "unavailable";
    QString recommendedNextStep = "unknown";
    QVector<ContractViolationRecord> violations;
    QStringList warnings;
    bool available = false;
};

RepoContractCheckState unavailableContractState(const QString &reason = QString("malformed contract-check data"));
RepoContractCheckState parseRepoContractCheckPayload(const QByteArray &payload);
RepoContractCheckState parseRepoContractCheckDocument(const QJsonDocument &document);

} // namespace DexRepoContractCheck

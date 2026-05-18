#include "repo_binder_pages.h"

#include <QGridLayout>
#include <QHBoxLayout>
#include <QSizePolicy>
#include <QVBoxLayout>
#include <QtGlobal>

#include "binder_page_helpers.h"
#include "repo_binder_page_helpers.h"
#include "render_helpers.h"
#include "repo_binder_page_helpers.h"
#include "repo_cleanup_queue_state.h"

namespace DexBinderPages {
namespace {

void addRepoContractsPage(
    QVBoxLayout *layout,
    const QString &lens,
    const DexRepoContractCheck::RepoContractCheckState &contracts) {
    layout->addWidget(makeStatsText("Purpose: repo rules with detection, severity, status, and linked evidence. Contracts turn organization into enforceable behavior."));
    auto *summary = makeStatsSection("contract-check status", true);
    auto *summaryLayout = static_cast<QVBoxLayout *>(summary->layout());
    summaryLayout->addWidget(makeStatsRow(
        {"scan", contracts.scanStatus,
         "risk", contracts.riskLevel,
         "status", contracts.contractStatus,
         "next", contracts.recommendedNextStep},
        true,
        false));
    layout->addWidget(summary);

    if (lens == "Dashboard" || lens == "Rules") {
        auto *rules = makeStatsSection("repo contracts", true);
        auto *rulesLayout = static_cast<QVBoxLayout *>(rules->layout());
        rulesLayout->addWidget(makeStatsRow({"contract", "rule", "applies to", "severity", "detection", "status"}, false, false));
        rulesLayout->addWidget(makeStatsRow({"PROTECTED-001", "protected paths need explicit approval", "protected boundary", "high", "repo diff-scan", "enforced"}, true, false));
        rulesLayout->addWidget(makeStatsRow({"GENERATED-001", "generated paths need generator proof", "generated boundary", "medium", "repo diff-scan", "enforced"}, false, false));
        rulesLayout->addWidget(makeStatsRow({"UNKNOWN-001", "unknown paths must be classified", "unknown boundary", "medium", "repo diff-scan", "enforced"}, true, false));
        rulesLayout->addWidget(makeStatsRow({"TEST-001", "test obligations must be handled", "changed files", "medium", "repo diff-scan", "enforced"}, false, false));
        rulesLayout->addWidget(makeStatsRow({"PROOF-001", "risk proof must be attached", "medium/high risk", "medium", "repo diff-scan", "enforced"}, true, false));
        layout->addWidget(rules);
    }

    if (lens == "Dashboard" || lens == "Violations" || lens == "Evidence") {
        auto *violations = makeStatsSection("current contract violations");
        auto *violationsLayout = static_cast<QVBoxLayout *>(violations->layout());
        violationsLayout->addWidget(makeStatsRow({"contract", "severity", "status", "paths", "action"}, false, false));
        if (contracts.violations.isEmpty()) {
            violationsLayout->addWidget(makeStatsText(contracts.contractStatus == "unavailable"
                ? "contract-check unavailable"
                : "no contract violations"));
        } else {
            for (const DexRepoContractCheck::ContractViolationRecord &violation : contracts.violations) {
                QString action = violation.requiredAction;
                if (violation.contractId == "PROTECTED-001") {
                    action = "approval required";
                } else if (violation.contractId == "GENERATED-001") {
                    action = "verify generator";
                } else if (violation.contractId == "UNKNOWN-001") {
                    action = "classify path";
                } else if (violation.contractId == "TEST-001") {
                    action = "run/assign tests";
                } else if (violation.contractId == "PROOF-001") {
                    action = "attach proof";
                }
                violationsLayout->addWidget(makeStatsRow(
                    {
                        violation.contractId,
                        violation.severity,
                        violation.status,
                        pathCountLabel(violation.paths),
                        action,
                    },
                    true,
                    false));
            }
        }
        if (!contracts.warnings.isEmpty()) {
            violationsLayout->addWidget(makeStatsText("warnings"));
            for (const QString &warning : contracts.warnings) {
                violationsLayout->addWidget(makeStatsRow({"warning", warning}, true, false));
            }
        }
        layout->addWidget(violations);
    }
}

} // namespace

QWidget *buildRepoContractsPage(
    const CockpitState &state,
    const QString &workerId,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &detailLens) {
    return buildCompactPage([state, workerId, project, detailLens, contracts = state.repoContractCheck](QVBoxLayout *layout) {
        addSelectedWorkerLens(layout, state, workerId, project, "Contracts", detailLens);
        addRepoContractsPage(layout, detailLens, contracts);
    });
}

} // namespace DexBinderPages

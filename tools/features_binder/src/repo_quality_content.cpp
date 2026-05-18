#include "repo_quality_sections.h"

#include <QVBoxLayout>

#include "binder_page_helpers.h"
#include "render_helpers.h"
#include "repo_binder_page_helpers.h"
#include "repo_cleanup_queue_state.h"

namespace DexBinderPages {

void addRepoQualityContent(
    QVBoxLayout *layout,
    const QString &lens,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt,
    const DexRepoBinderTemplate::BinderTemplate &binderTemplate) {
    const DexRepoCleanupQueue::RepoCleanupQueueState cleanupQueue =
        DexRepoCleanupQueue::deriveRepoCleanupQueue(scan, contracts);
    layout->addWidget(makeStatsText("Purpose: repo health, repo grade, repo stats, warnings, proof coverage, and build reproducibility."));
    addTemplateSections(layout, binderTemplate, "Quality", lens, project, scan, contracts, proofReceipt);
    auto *tests = makeStatsSection("test / build status", true);
    auto *testsLayout = static_cast<QVBoxLayout *>(tests->layout());
    testsLayout->addWidget(makeStatsRow({"run id", compactReceiptLabel(proofReceipt.runId), "created", proofReceipt.createdAt, "manifest", proofReceipt.runManifestPath.isEmpty() ? QString("latest only") : compactReceiptLabel(proofReceipt.runManifestPath)}, false, false));
    testsLayout->addWidget(makeStatsRow({"receipt status", proofReceipt.receiptStatus, "commands", QString::number(proofReceipt.commandReceipts.size()), "screenshots", QString::number(proofReceipt.screenshots.size())}, false, false));
    if (proofReceipt.commandReceipts.isEmpty()) {
        testsLayout->addWidget(makeStatsText(proofReceipt.loaded ? "command receipts: none in manifest" : compactLines(proofReceipt.warnings).join(" | "), "statsRiskText"));
    } else {
        testsLayout->addWidget(makeStatsRow({"receipt", "command", "status", "exit"}, false, false));
        for (const DexRepoProofReceipt::CommandReceiptRecord &receipt : proofReceipt.commandReceipts) {
            testsLayout->addWidget(makeStatsRow(
                {receipt.receiptId, compactCommandLabel(receipt.command), receipt.status, QString::number(receipt.exitCode)},
                receipt.status != "succeeded",
                false));
        }
    }
    layout->addWidget(tests);
    auto *cleanup = makeStatsSection("cleanup debt", true);
    auto *cleanupLayout = static_cast<QVBoxLayout *>(cleanup->layout());
    cleanupLayout->addWidget(makeStatsRow(
        {
            "queue", cleanupQueue.queueStatus,
            "items", QString::number(cleanupQueue.items.size()),
            "high", QString::number(cleanupQueue.highCount),
            "medium", QString::number(cleanupQueue.mediumCount),
            "blocked/action", QString::number(cleanupQueue.actionRequiredCount),
        },
        cleanupQueue.queueStatus == "open",
        false));
    if (lens == "Dashboard" || lens == "Stats" || lens == "Warnings") {
        addCleanupQueueRows(cleanupLayout, cleanupQueue);
    }
    layout->addWidget(cleanup);
    if (lens == "Dashboard" || lens == "Stats") {
        auto *section = makeStatsSection("repo scan quality signals", true);
        auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
        sectionLayout->addWidget(makeStatsRow(
            {"risk level", scan.riskLevel,
             "changed", QString::number(scan.changedFiles.size()),
             "contract status", contracts.contractStatus,
             "recommended", contracts.recommendedNextStep.isEmpty() ? QString("unknown") : contracts.recommendedNextStep},
            true,
            false));
        sectionLayout->addWidget(makeStatsRow(
            {"protected", QString::number(scan.protectedBoundaryPaths.size()),
             "generated", QString::number(scan.generatedBoundaryPaths.size()),
             "unknown", QString::number(scan.unknownBoundaryPaths.size()),
             "violations", QString::number(contracts.violations.size())},
            true,
            false));
        sectionLayout->addWidget(makeStatsRow({"test obligations", QString::number(scan.testObligations.size()), "proof obligations", QString::number(scan.proofObligations.size())}, true, false));
        if (scan.testObligations.isEmpty()) {
            sectionLayout->addWidget(makeStatsText("test obligations: none"));
        } else {
            addListRows(sectionLayout, "test obligation", scan.testObligations);
        }
        if (scan.proofObligations.isEmpty()) {
            sectionLayout->addWidget(makeStatsText("proof obligations: none"));
        } else {
            addListRows(sectionLayout, "proof obligation", scan.proofObligations);
        }
        if (!scan.warnings.isEmpty()) {
            sectionLayout->addWidget(makeStatsText("warnings"));
            for (const QString &warning : scan.warnings) {
                sectionLayout->addWidget(makeStatsRow({"warning", warning}, true, false));
            }
        }
        if (!contracts.violations.isEmpty()) {
            sectionLayout->addWidget(makeStatsText("contract violations"));
            for (const DexRepoContractCheck::ContractViolationRecord &violation : contracts.violations) {
                sectionLayout->addWidget(makeStatsRow(
                    {violation.contractId, violation.severity, violation.status, violation.requiredAction},
                    true,
                    false));
            }
        }
        layout->addWidget(section);
    }
    if (lens == "Dashboard" || lens == "Warnings") {
        auto *warnings = makeStatsSection("known warnings");
        auto *warningsLayout = static_cast<QVBoxLayout *>(warnings->layout());
        if (scan.warnings.isEmpty() && contracts.warnings.isEmpty()) {
            warningsLayout->addWidget(makeStatsText("warnings: none reported"));
        }
        for (const QString &warning : scan.warnings) {
            warningsLayout->addWidget(makeStatsRow({"diff-scan", warning}, true, false));
        }
        for (const QString &warning : contracts.warnings) {
            warningsLayout->addWidget(makeStatsRow({"contract-check", warning}, true, false));
        }
        layout->addWidget(warnings);
    }
    if (lens == "Dashboard" || lens == "Grade") {
        auto *grade = makeStatsSection("repo grade", true);
        auto *gradeLayout = static_cast<QVBoxLayout *>(grade->layout());
        gradeLayout->addWidget(makeStatsText("repo grade is not wired. Quality shows scan, contract, cleanup, and proof inputs only."));
        layout->addWidget(grade);
    }
    if (lens == "Dashboard" || lens == "Stats") {
        auto *stats = makeStatsSection("repo stats", true);
        auto *statsLayout = static_cast<QVBoxLayout *>(stats->layout());
        statsLayout->addWidget(makeStatsRow({"changed files", QString::number(scan.changedFiles.size()), "protected", QString::number(scan.protectedBoundaryPaths.size()), "generated", QString::number(scan.generatedBoundaryPaths.size()), "unknown", QString::number(scan.unknownBoundaryPaths.size())}, !scan.unknownBoundaryPaths.isEmpty() || !scan.protectedBoundaryPaths.isEmpty(), false));
        statsLayout->addWidget(makeStatsRow({"test obligations", QString::number(scan.testObligations.size()), "proof obligations", QString::number(scan.proofObligations.size()), "contract violations", QString::number(contracts.violations.size())}, !contracts.violations.isEmpty(), false));
        layout->addWidget(stats);
    }
    if (lens == "Dashboard" || lens == "Proof") {
        auto *proof = makeStatsSection("proof coverage");
        auto *proofLayout = static_cast<QVBoxLayout *>(proof->layout());
        proofLayout->addWidget(makeStatsRow({"run", compactReceiptLabel(proofReceipt.runId), "created", proofReceipt.createdAt, "path", compactReceiptLabel(proofReceipt.runManifestPath)}, false, false));
        proofLayout->addWidget(makeStatsRow({"receipt", proofReceipt.receiptStatus, "screenshots", QString::number(proofReceipt.screenshots.size()), "commands", QString::number(proofReceipt.commandReceipts.size())}, false, false));
        if (proofReceipt.screenshots.isEmpty()) {
            proofLayout->addWidget(makeStatsText(proofReceipt.loaded ? "screenshots: none in manifest" : compactLines(proofReceipt.warnings).join(" | "), "statsRiskText"));
        } else {
            proofLayout->addWidget(makeStatsRow({"screenshot", "viewport", "subject", "status"}, false, false));
            for (const DexRepoProofReceipt::ProofScreenshotRecord &screenshot : proofReceipt.screenshots) {
                proofLayout->addWidget(makeStatsRow(
                    {screenshot.path, screenshot.viewport, screenshot.subject, screenshot.status},
                    screenshot.status != "attached",
                    false));
            }
        }
        layout->addWidget(proof);
    }
}

} // namespace DexBinderPages

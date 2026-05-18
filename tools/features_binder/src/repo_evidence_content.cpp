#include "repo_evidence_sections.h"

#include <QVBoxLayout>

#include "binder_page_helpers.h"
#include "render_helpers.h"
#include "repo_binder_page_helpers.h"

namespace DexBinderPages {

void addRepoEvidenceContent(
    QVBoxLayout *layout,
    const QString &lens,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt,
    const DexRepoPromotionReport::RepoPromotionReportState &promotionReport) {
    layout->addWidget(makeStatsText("Purpose: receipts for repo changes: screenshots, commands, changed files, failures, corrections, promotions, and repo relationships."));
    if (lens == "Dashboard" || lens == "Proof") {
        auto *proof = makeStatsSection("proof screenshots", true);
        auto *proofLayout = static_cast<QVBoxLayout *>(proof->layout());
        proofLayout->addWidget(makeStatsRow({"run", compactReceiptLabel(proofReceipt.runId), "created", proofReceipt.createdAt, "manifest", compactReceiptLabel(proofReceipt.runManifestPath)}, false, false));
        proofLayout->addWidget(makeStatsRow({"receipt status", proofReceipt.receiptStatus, "proof type", proofReceipt.proofType, "tab", proofReceipt.tab}, false, false));
        proofLayout->addWidget(makeStatsRow({"path", "viewport", "subject", "status"}, false, false));
        if (proofReceipt.screenshots.isEmpty()) {
            proofLayout->addWidget(makeStatsText(proofReceipt.loaded ? "no screenshots in proof manifest" : compactLines(proofReceipt.warnings).join(" | "), "statsRiskText"));
        } else {
            for (const DexRepoProofReceipt::ProofScreenshotRecord &screenshot : proofReceipt.screenshots) {
                proofLayout->addWidget(makeStatsRow(
                    {screenshot.path, screenshot.viewport, screenshot.subject, screenshot.status},
                    screenshot.status != "attached",
                    false));
            }
        }
        layout->addWidget(proof);
    }
    if (lens == "Dashboard" || lens == "Commands") {
        auto *commands = makeStatsSection("command receipts");
        auto *commandsLayout = static_cast<QVBoxLayout *>(commands->layout());
        commandsLayout->addWidget(makeStatsRow({"receipt", "command", "status", "exit", "log"}, false, false));
        if (proofReceipt.commandReceipts.isEmpty()) {
            commandsLayout->addWidget(makeStatsText(proofReceipt.loaded ? "no command receipts in proof manifest" : compactLines(proofReceipt.warnings).join(" | "), "statsRiskText"));
        } else {
            for (const DexRepoProofReceipt::CommandReceiptRecord &receipt : proofReceipt.commandReceipts) {
                commandsLayout->addWidget(makeStatsRow(
                    {receipt.receiptId, compactCommandLabel(receipt.command), receipt.status, QString::number(receipt.exitCode), receipt.logPath},
                    receipt.status != "succeeded",
                    false));
            }
        }
        layout->addWidget(commands);
    }
    if (lens == "Dashboard" || lens == "Failures") {
        auto *failures = makeStatsSection("failure records");
        auto *failureLayout = static_cast<QVBoxLayout *>(failures->layout());
        failureLayout->addWidget(makeStatsText("failure records are not wired for repo binder v1."));
        layout->addWidget(failures);
    }
    if (lens == "Dashboard" || lens == "Corrections") {
        auto *corrections = makeStatsSection(lens == "Promotion" ? "promotion receipts" : "correction records", true);
        auto *correctionLayout = static_cast<QVBoxLayout *>(corrections->layout());
        correctionLayout->addWidget(makeStatsText("correction records are not wired. Evidence shows proof and promotion receipts only when those reports exist."));
        layout->addWidget(corrections);
    }
    if (lens == "Dashboard" || lens == "Promotion") {
        auto *promotion = makeStatsSection("promotion dry-run report", true);
        auto *promotionLayout = static_cast<QVBoxLayout *>(promotion->layout());
        promotionLayout->addWidget(makeStatsRow(
            {
                "mode", promotionReport.mode,
                "created", promotionReport.createdAt,
                "compared", QString::number(promotionReport.comparedCount),
                "blocked", QString::number(promotionReport.blockedCount),
            },
            promotionReport.blockedCount > 0 || promotionReport.mode == "unavailable",
            false));
        promotionLayout->addWidget(makeStatsRow({"authority writes", promotionReport.authorityWrites, "dry run", promotionReport.dryRunOnly ? QString("true") : QString("false")}, false, false));
        if (!promotionReport.loaded) {
            promotionLayout->addWidget(makeStatsText(compactLines(promotionReport.warnings).join(" | "), "statsRiskText"));
        } else {
            promotionLayout->addWidget(makeStatsRow({"source", "status", "destination"}, false, false));
            for (const DexRepoPromotionReport::PromotionComparedFile &file : promotionReport.compared) {
                promotionLayout->addWidget(makeStatsRow(
                    {
                        file.source,
                        file.status,
                        compactPathLabel(file.destination, 44),
                    },
                    file.status != "identical",
                    false));
            }
            if (promotionReport.blocked.isEmpty()) {
                promotionLayout->addWidget(makeStatsText("blocked files: none"));
            } else {
                promotionLayout->addWidget(makeStatsRow({"blocked file", "reason"}, false, false));
                for (const DexRepoPromotionReport::PromotionBlockedFile &file : promotionReport.blocked) {
                    promotionLayout->addWidget(makeStatsRow({file.requestedFile, file.reason}, true, false));
                }
            }
        }
        layout->addWidget(promotion);
    }
    if (lens == "Dashboard" || lens == "Relations") {
        auto *relations = makeStatsSection("repo relationships", true);
        auto *relationsLayout = static_cast<QVBoxLayout *>(relations->layout());
        relationsLayout->addWidget(makeStatsText("repo relationships are not wired. Add saved registry fields or backend relationship reports before showing rows here."));
        layout->addWidget(relations);
    }
}

} // namespace DexBinderPages

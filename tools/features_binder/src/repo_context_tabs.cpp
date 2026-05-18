#include "repo_context_sections.h"

#include <QVBoxLayout>

#include "render_helpers.h"
#include "right_context_render_helpers.h"

namespace DexRightContext {
namespace {

void addInventoryContext(QVBoxLayout *contentLayout, const DexProjects::ProjectRegistryEntry &project) {
    addContextSection(contentLayout, "registry inventory");
    addWrappedLine(contentLayout, "state                          registry fields only");
    addWrappedLine(contentLayout, "safe zones                     " + QString::number(project.safeEditZones.size()));
    addWrappedLine(contentLayout, "protected zones                " + QString::number(project.protectedZones.size()));
    addWrappedLine(contentLayout, "generated zones                " + QString::number(project.generatedZones.size()));
    addWrappedLine(contentLayout, "inspect support                " + QString(project.fileInspectSupported ? "yes" : "reserved"));
}

void addProfileContext(QVBoxLayout *contentLayout, const DexProjects::ProjectRegistryEntry &project) {
    addContextSection(contentLayout, "repo profile");
    addWrappedLine(contentLayout, "runtime                        " + (project.projectType.isEmpty() ? QString("not set") : project.projectType));
    addWrappedLine(contentLayout, "primary proof                  " + (project.proofCommands.isEmpty() ? QString("not set") : compactLines(project.proofCommands).join(" | ")));
    addWrappedLine(contentLayout, "worker ids                     " + (project.workerIds.isEmpty() ? QString("not set") : compactLines(project.workerIds).join(" | ")));
}

void addMapContext(QVBoxLayout *contentLayout, const DexProjects::ProjectRegistryEntry &project) {
    addContextSection(contentLayout, "selected zone");
    addWrappedLine(contentLayout, "safe edit zones                " + compactLines(project.safeEditZones).join(" | "));
    addWrappedLine(contentLayout, "protected zones                " + compactLines(project.protectedZones).join(" | "));
    addWrappedLine(contentLayout, "agent edit rule                registry boundaries only");
}

void addAuthorityContext(QVBoxLayout *contentLayout, const DexProjects::ProjectRegistryEntry &project) {
    addContextSection(contentLayout, "selected authority");
    addWrappedLine(contentLayout, "authority status               " + (project.authority.isEmpty() ? QString("not set") : project.authority));
    addWrappedLine(contentLayout, "protected zones                " + compactLines(project.protectedZones).join(" | "));
    addWrappedLine(contentLayout, "drift status                   from scanner only");
}

void addContractsContext(QVBoxLayout *contentLayout, const DexRepoContractCheck::RepoContractCheckState &contracts) {
    addContextSection(contentLayout, "selected contract");
    if (contracts.violations.isEmpty()) {
        addWrappedLine(contentLayout, "contract                       none selected");
        addWrappedLine(contentLayout, "status                         " + contracts.contractStatus);
        return;
    }
    const DexRepoContractCheck::ContractViolationRecord violation = contracts.violations.first();
    addWrappedLine(contentLayout, "contract                       " + violation.contractId);
    addWrappedLine(contentLayout, "rule                           " + violation.rule);
    addWrappedLine(contentLayout, "severity                       " + violation.severity);
    addWrappedLine(contentLayout, "status                         " + violation.status);
    addWrappedLine(contentLayout, "action                         " + violation.requiredAction);
}

void addActivityContext(
    QVBoxLayout *contentLayout,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoCleanupQueue::RepoCleanupQueueState &cleanupQueue) {
    addContextSection(contentLayout, "activity state");
    addWrappedLine(contentLayout, "scan                           " + scan.scanStatus);
    addWrappedLine(contentLayout, "monitor                        not wired");
    addWrappedLine(contentLayout, "cleanup queue                  " + cleanupQueue.queueStatus);
    addWrappedLine(contentLayout, "cleanup items                  " + QString::number(cleanupQueue.items.size()));
}

void addQualityContext(
    QVBoxLayout *contentLayout,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoCleanupQueue::RepoCleanupQueueState &cleanupQueue) {
    addContextSection(contentLayout, "quality state");
    addWrappedLine(contentLayout, "repo grade                     not wired");
    addWrappedLine(contentLayout, "contract status                " + contracts.contractStatus);
    addWrappedLine(contentLayout, "unknown boundary count         " + QString::number(scan.unknownBoundaryPaths.size()));
    addWrappedLine(contentLayout, "proof obligations              " + QString::number(scan.proofObligations.size()));
    addWrappedLine(contentLayout, "cleanup debt                   " + QString::number(cleanupQueue.items.size()));
}

void addEvidenceContext(
    QVBoxLayout *contentLayout,
    const DexRepoPromotionReport::RepoPromotionReportState &promotionReport,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt) {
    addContextSection(contentLayout, "evidence state");
    addWrappedLine(contentLayout, "proof screenshots              " + QString::number(proofReceipt.screenshots.size()));
    addWrappedLine(contentLayout, "command receipts               " + QString::number(proofReceipt.commandReceipts.size()));
    addWrappedLine(contentLayout, "receipt status                 " + proofReceipt.receiptStatus);
    addWrappedLine(contentLayout, "promotion compared            " + QString::number(promotionReport.comparedCount));
    addWrappedLine(contentLayout, "promotion blocked             " + QString::number(promotionReport.blockedCount));
    addWrappedLine(contentLayout, "relationships                  not wired");
    addWrappedLine(contentLayout, "promotion                      explicit only");
}

} // namespace

void addRepoTopTabContext(
    QVBoxLayout *contentLayout,
    const QString &selectedTopTab,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoPromotionReport::RepoPromotionReportState &promotionReport,
    const DexRepoCleanupQueue::RepoCleanupQueueState &cleanupQueue,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt) {
    if (selectedTopTab == "Inventory") {
        addInventoryContext(contentLayout, project);
    } else if (selectedTopTab == "Profile") {
        addProfileContext(contentLayout, project);
    } else if (selectedTopTab == "Map") {
        addMapContext(contentLayout, project);
    } else if (selectedTopTab == "Authority") {
        addAuthorityContext(contentLayout, project);
    } else if (selectedTopTab == "Contracts") {
        addContractsContext(contentLayout, contracts);
    } else if (selectedTopTab == "Activity") {
        addActivityContext(contentLayout, scan, cleanupQueue);
    } else if (selectedTopTab == "Quality") {
        addQualityContext(contentLayout, scan, contracts, cleanupQueue);
    } else {
        addEvidenceContext(contentLayout, promotionReport, proofReceipt);
    }
}

} // namespace DexRightContext

#include "repo_context_sections.h"

#include <QVBoxLayout>

#include "render_helpers.h"
#include "right_context_render_helpers.h"

namespace DexRightContext {
namespace {

QString renderContextTemplateText(
    QString text,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt) {
    auto replace = [&text](const QString &token, const QString &value) {
        text.replace("{{" + token + "}}", value);
    };
    replace("project.name", DexProjects::displayName(project));
    replace("project.path", project.path);
    replace("project.role", project.role.isEmpty() ? QString("unknown") : project.role);
    replace("project.authority", project.authority.isEmpty() ? QString("unknown") : project.authority);
    replace("project.project_type", project.projectType.isEmpty() ? QString("unknown") : project.projectType);
    replace("project.binder_template", projectBinderTemplate(project));
    replace("features.feature_pack_total", QString::number(project.featurePackTotal));
    replace("features.feature_pack_groups", featurePackGroupSummary(project));
    replace("scan.status", scan.scanStatus);
    replace("scan.risk_level", scan.riskLevel);
    replace("scan.changed_count", QString::number(scan.changedFiles.size()));
    replace("scan.protected_count", QString::number(scan.protectedBoundaryPaths.size()));
    replace("scan.generated_count", QString::number(scan.generatedBoundaryPaths.size()));
    replace("scan.unknown_count", QString::number(scan.unknownBoundaryPaths.size()));
    replace("contracts.status", contracts.contractStatus);
    replace("contracts.next_step", contracts.recommendedNextStep);
    replace("proof.receipt_status", proofReceipt.receiptStatus);
    return text;
}

} // namespace

void addTemplateContextLines(
    QVBoxLayout *contentLayout,
    const DexRepoBinderTemplate::BinderTemplate &binderTemplate,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt) {
    if (binderTemplate.contextLines.isEmpty()) {
        return;
    }
    addContextSection(contentLayout, "binder template");
    for (const QString &line : binderTemplate.contextLines) {
        addWrappedLine(contentLayout, renderContextTemplateText(line, project, scan, contracts, proofReceipt));
    }
}

} // namespace DexRightContext

#include "repo_binder_page_helpers.h"

#include <QVBoxLayout>

#include "binder_page_helpers.h"
#include "render_helpers.h"

namespace DexBinderPages {

QString renderTemplateText(
    QString text,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt) {
    auto replace = [&text](const QString &token, const QString &value) {
        text.replace("{{" + token + "}}", value);
    };

    replace("project.id", project.projectId);
    replace("project.name", DexProjects::displayName(project));
    replace("project.path", project.path);
    replace("project.role", project.role.isEmpty() ? QString("unknown") : project.role);
    replace("project.status", project.status.isEmpty() ? QString("unknown") : project.status);
    replace("project.authority", project.authority.isEmpty() ? QString("unknown") : project.authority);
    replace("project.project_type", project.projectType.isEmpty() ? QString("unknown") : project.projectType);
    replace("project.binder_template", projectBinderTemplate(project));
    replace("project.safe_edit_zones", compactLines(project.safeEditZones).join(" | "));
    replace("project.protected_zones", compactLines(project.protectedZones).join(" | "));
    replace("project.generated_zones", compactLines(project.generatedZones).join(" | "));
    replace("project.safe_edit_zone_count", QString::number(project.safeEditZones.size()));
    replace("project.protected_zone_count", QString::number(project.protectedZones.size()));
    replace("project.generated_zone_count", QString::number(project.generatedZones.size()));

    replace("features.feature_pack_total", QString::number(project.featurePackTotal));
    replace("features.feature_pack_logic", QString::number(project.featurePackLogic));
    replace("features.feature_pack_sim", QString::number(project.featurePackSim));
    replace("features.feature_pack_ui", QString::number(project.featurePackUi));
    replace("features.feature_pack_workflows", QString::number(project.featurePackWorkflows));
    replace("features.feature_pack_groups", featurePackGroupSummary(project));

    replace("scan.status", scan.scanStatus);
    replace("scan.risk_level", scan.riskLevel);
    replace("scan.changed_count", QString::number(scan.changedFiles.size()));
    replace("scan.protected_count", QString::number(scan.protectedBoundaryPaths.size()));
    replace("scan.generated_count", QString::number(scan.generatedBoundaryPaths.size()));
    replace("scan.unknown_count", QString::number(scan.unknownBoundaryPaths.size()));
    replace("scan.test_obligation_count", QString::number(scan.testObligations.size()));
    replace("scan.proof_obligation_count", QString::number(scan.proofObligations.size()));

    replace("contracts.status", contracts.contractStatus);
    replace("contracts.violation_count", QString::number(contracts.violations.size()));
    replace("contracts.next_step", contracts.recommendedNextStep);

    replace("proof.receipt_status", proofReceipt.receiptStatus);
    replace("proof.screenshot_count", QString::number(proofReceipt.screenshots.size()));
    return text;
}

QStringList renderTemplateCells(
    const QStringList &cells,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt) {
    QStringList rendered;
    for (const QString &cell : cells) {
        rendered.push_back(renderTemplateText(cell, project, scan, contracts, proofReceipt));
    }
    return rendered;
}

void addTemplateSections(
    QVBoxLayout *layout,
    const DexRepoBinderTemplate::BinderTemplate &binderTemplate,
    const QString &tab,
    const QString &lens,
    const DexProjects::ProjectRegistryEntry &project,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt) {
    for (const DexRepoBinderTemplate::BinderTemplateSection &templateSection : binderTemplate.sections) {
        if (templateSection.tab != tab) {
            continue;
        }
        if (!templateSection.lenses.isEmpty() && !templateSection.lenses.contains(lens)) {
            continue;
        }

        auto *section = makeStatsSection(templateSection.title, templateSection.subtle);
        auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
        if (!templateSection.purpose.isEmpty()) {
            sectionLayout->addWidget(makeStatsText(renderTemplateText(templateSection.purpose, project, scan, contracts, proofReceipt)));
        }
        for (const QString &line : templateSection.lines) {
            sectionLayout->addWidget(makeStatsText(renderTemplateText(line, project, scan, contracts, proofReceipt)));
        }
        for (const QString &badge : templateSection.badges) {
            sectionLayout->addWidget(makeStatsRow({"badge", renderTemplateText(badge, project, scan, contracts, proofReceipt)}, false, false));
        }
        if (templateSection.rows.isEmpty() && !templateSection.emptyState.isEmpty()) {
            sectionLayout->addWidget(makeStatsText(renderTemplateText(templateSection.emptyState, project, scan, contracts, proofReceipt)));
        }
        for (const DexRepoBinderTemplate::BinderTemplateRow &row : templateSection.rows) {
            sectionLayout->addWidget(makeStatsRow(
                renderTemplateCells(row.cells, project, scan, contracts, proofReceipt),
                row.risk,
                false));
        }
        layout->addWidget(section);
    }
}

} // namespace DexBinderPages

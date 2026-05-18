#include "repo_inventory_sections.h"

#include <QVBoxLayout>

#include "binder_page_helpers.h"
#include "render_helpers.h"
#include "repo_binder_page_helpers.h"

namespace DexBinderPages {

void addRepoInventoryContent(
    QVBoxLayout *layout,
    const DexProjects::ProjectRegistryEntry &project,
    const QString &lens,
    const DexRepoDiffScan::RepoDiffScanState &scan,
    const DexRepoContractCheck::RepoContractCheckState &contracts,
    const DexRepoProofReceipt::RepoProofReceiptState &proofReceipt,
    const DexRepoBinderTemplate::BinderTemplate &binderTemplate) {
    layout->addWidget(makeStatsText("Purpose: file/folder map. Every row answers: what is it, who owns it, how dangerous is it, and what happens next."));
    addTemplateSections(layout, binderTemplate, "Inventory", lens, project, scan, contracts, proofReceipt);
    auto *summary = makeStatsSection("repo inventory summary", true);
    auto *summaryLayout = static_cast<QVBoxLayout *>(summary->layout());
    summaryLayout->addWidget(makeStatsRow({
        "repo: " + DexProjects::displayName(project),
        "project registered",
        "scan status: " + scan.scanStatus,
        "risk: " + scan.riskLevel,
    }, false, false));
    summaryLayout->addWidget(makeStatsRow({
        "authority: " + (project.authority.isEmpty() ? QString("unknown") : project.authority),
        "path: " + project.path,
        "safe zones: " + QString::number(project.safeEditZones.size()),
        "protected zones: " + QString::number(project.protectedZones.size()),
    }, true, false));
    layout->addWidget(summary);

    if (lens == "Dashboard" || lens == "Tree" || lens == "Files") {
        auto *section = makeStatsSection("registered zones");
        auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
        sectionLayout->addWidget(makeStatsRow({"zone", "source", "status", "next action"}, false, false));
        const QStringList safeZones = compactLines(project.safeEditZones);
        const QStringList protectedZones = compactLines(project.protectedZones);
        const QStringList generatedZones = compactLines(project.generatedZones);
        if (safeZones.isEmpty() && protectedZones.isEmpty() && generatedZones.isEmpty()) {
            sectionLayout->addWidget(makeStatsRow({"zones", "registry", "not set", "open Settings"}, false, false));
        }
        for (const QString &zone : safeZones) {
            sectionLayout->addWidget(makeStatsRow({zone, "registry", "safe edit zone", "scan later"}, false, false));
        }
        for (const QString &zone : protectedZones) {
            sectionLayout->addWidget(makeStatsRow({zone, "registry", "protected", "inspect first"}, true, false));
        }
        for (const QString &zone : generatedZones) {
            sectionLayout->addWidget(makeStatsRow({zone, "registry", "generated", "do not hand edit"}, true, false));
        }
        layout->addWidget(section);
    }
    if (lens == "Dashboard" || lens == "Folders") {
        auto *section = makeStatsSection("folder detail");
        auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
        sectionLayout->addWidget(makeStatsRow({"project path", project.path.isEmpty() ? QString("not set") : project.path, "registered only", "no filesystem inspection"}, false, false));
        sectionLayout->addWidget(makeStatsRow({"safe edit zones", QString::number(project.safeEditZones.size()), "from registry", "select Project File Inspect later"}, false, false));
        sectionLayout->addWidget(makeStatsRow({"generated zones", QString::number(project.generatedZones.size()), "from registry", "keep generated containment"}, true, false));
        layout->addWidget(section);
    }
    if (lens == "Dashboard" || lens == "Files") {
        auto *section = makeStatsSection("changed files by boundary");
        auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
        sectionLayout->addWidget(makeStatsRow({"path", "change type", "boundary", "risk"}, false, false));
        if (!scan.protectedBoundaryPaths.isEmpty()) {
            sectionLayout->addWidget(makeStatsRow(
                {"protected boundaries", QString::number(scan.protectedBoundaryPaths.size()), "status", "scanner bucket"},
                true,
                false));
        }
        if (!scan.generatedBoundaryPaths.isEmpty()) {
            sectionLayout->addWidget(makeStatsRow(
                {"generated boundaries", QString::number(scan.generatedBoundaryPaths.size()), "status", "scanner bucket"},
                true,
                false));
        }
        if (!scan.unknownBoundaryPaths.isEmpty()) {
            sectionLayout->addWidget(makeStatsRow(
                {"unknown boundaries", QString::number(scan.unknownBoundaryPaths.size()), "status", "scanner bucket"},
                true,
                false));
        }
        if (scan.changedFiles.isEmpty()) {
            sectionLayout->addWidget(makeStatsText("no changed files"));
        } else {
            QStringList seenBoundaries;
            for (const DexRepoDiffScan::ChangedFileRecord &record : scan.changedFiles) {
                if (!seenBoundaries.contains(record.boundary)) {
                    seenBoundaries.push_back(record.boundary);
                }
            }
            for (const QString &boundary : seenBoundaries) {
                sectionLayout->addWidget(makeStatsText("boundary: " + (boundary.isEmpty() ? QString("unknown") : boundary)));
                for (const DexRepoDiffScan::ChangedFileRecord &record : scan.changedFiles) {
                    if (record.boundary == boundary) {
                        sectionLayout->addWidget(makeStatsRow(
                            {
                                record.path,
                                record.changeType.isEmpty() ? QString("unknown") : record.changeType,
                                record.boundary.isEmpty() ? QString("unknown") : record.boundary,
                                record.riskLevel.isEmpty() ? QString("unknown") : record.riskLevel,
                            },
                            true,
                            false));
                    }
                }
            }
        }
        layout->addWidget(section);
    }
    if (lens == "Dashboard" || lens == "Unknown") {
        auto *section = makeStatsSection("unknown and inspect-first paths", true);
        auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
        sectionLayout->addWidget(makeStatsRow({"unknown file rows", scan.unknownBoundaryPaths.isEmpty() ? QString("none reported") : QString::number(scan.unknownBoundaryPaths.size()), "source", "diff-scan"}, !scan.unknownBoundaryPaths.isEmpty(), false));
        sectionLayout->addWidget(makeStatsRow({"protected zones", project.protectedZones.isEmpty() ? QString("not set") : project.protectedZones.join(", "), "source", "registry"}, !project.protectedZones.isEmpty(), false));
        layout->addWidget(section);
    }
    if (lens == "Dashboard" || lens == "Detail") {
        auto *section = makeStatsSection("selected file detail", true);
        auto *sectionLayout = static_cast<QVBoxLayout *>(section->layout());
        sectionLayout->addWidget(makeStatsRow({"file detail", "not wired", "registry only", "no scan facts"}, true, false));
        sectionLayout->addWidget(makeStatsRow({
            "inspect supported: " + QString(project.fileInspectSupported ? "yes" : "no"),
            "edit supported: " + QString(project.fileEditSupported ? "yes" : "no"),
            "editor seam: " + (project.editorCoreStatus.isEmpty() ? QString("unknown") : project.editorCoreStatus),
        }, true, false));
        layout->addWidget(section);
    }
}

} // namespace DexBinderPages

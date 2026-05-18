#include "main_window.h"

#include "project_registry.h"
#include "repo_binder_template.h"
#include "repo_proof_receipt_state.h"
#include "repo_promotion_report_state.h"

#include <algorithm>

void DexHomeV2Window::loadProjectRegistryIntoState() {
    const DexProjects::ProjectRegistry registry = DexProjects::loadProjectRegistryFile(projectRegistryPath_);
    state_.projectRegistrySource = registry.sourcePath;
    state_.projectRegistryLoaded = registry.loaded;
    state_.projectRegistryError = registry.error;
    if (state_.projectRegistryLoaded) {
        state_.registryProjects = registry.projects;
        state_.registryWorkers = registry.workers;
        for (const DexProjects::WorkerRegistryEntry &registryWorker : registry.workers) {
            auto found = std::find_if(
                state_.workers.begin(),
                state_.workers.end(),
                [&registryWorker](const WorkerSummary &worker) {
                    return worker.id == registryWorker.workerId;
                });
            const WorkerSummary worker = {
                registryWorker.workerId,
                registryWorker.role,
                registryWorker.displayName,
                registryWorker.status,
                registryWorker.codexSessionId,
                registryWorker.codexModel,
                registryWorker.codexDirectory,
                registryWorker.codexPermissions,
                registryWorker.codexContextWindow,
                registryWorker.codexAccount,
            };
            if (found == state_.workers.end()) {
                state_.workers.push_back(worker);
            } else {
                *found = worker;
            }
        }
    } else {
        state_.registryProjects.clear();
        state_.registryWorkers.clear();
    }
}

void DexHomeV2Window::loadBinderTemplatesIntoState() {
    state_.binderTemplateStore = DexRepoBinderTemplate::loadBinderTemplateStore(binderTemplateDirPath_);
}

void DexHomeV2Window::loadProofReceiptIntoState() {
    state_.repoProofReceipt = DexRepoProofReceipt::loadRepoProofReceiptFile(proofReceiptPath_);
}

void DexHomeV2Window::loadPromotionReportIntoState() {
    state_.repoPromotionReport = DexRepoPromotionReport::loadRepoPromotionReportFile(promotionReportPath_);
}

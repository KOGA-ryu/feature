#include "repo_binder_template.h"

namespace DexRepoBinderTemplate {

BinderTemplate defaultBinderTemplate() {
    BinderTemplate templateRecord;
    templateRecord.templateId = QString::fromUtf8(kDefaultTemplateId);
    templateRecord.displayName = "Default Repo Binder";
    templateRecord.loaded = true;
    templateRecord.topTabs = {"Profile", "Inventory", "Map", "Authority", "Contracts", "Activity", "Quality", "Evidence"};
    templateRecord.detailLenses.insert("Profile", {"Dashboard", "Identity", "Team", "Commands", "Boundaries"});
    templateRecord.detailLenses.insert("Inventory", {"Dashboard", "Tree", "Files", "Folders", "Unknown", "Detail"});
    return templateRecord;
}

} // namespace DexRepoBinderTemplate

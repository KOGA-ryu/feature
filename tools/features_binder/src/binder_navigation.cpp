#include "binder_navigation.h"

#include "binder_state.h"

QStringList repoBinderTopTabs() {
    return {"Profile", "Inventory", "Map", "Authority", "Contracts", "Activity", "Quality", "Evidence"};
}

QStringList topTabsFor(bool repoMode) {
    return repoMode ? repoBinderTopTabs() : DexBinder::binderTopTabs();
}

QStringList detailLensTabsFor(const QString &topTab, bool repoMode) {
    if (repoMode) {
        if (topTab == "Profile") {
            return {"Dashboard", "Identity", "Team", "Commands", "Boundaries"};
        }
        if (topTab == "Inventory") {
            return {"Dashboard", "Tree", "Files", "Folders", "Unknown", "Detail"};
        }
        if (topTab == "Map") {
            return {"Dashboard", "Zones", "Folders", "Generated", "Proof", "Archive"};
        }
        if (topTab == "Authority") {
            return {"Dashboard", "Law", "Contracts", "Registries", "Drift", "Proof"};
        }
        if (topTab == "Contracts") {
            return {"Dashboard", "Rules", "Detection", "Violations", "Scope", "Evidence"};
        }
        if (topTab == "Activity") {
            return {"Dashboard", "Scan", "Monitor", "Report", "Changes", "Cleanup"};
        }
        if (topTab == "Quality") {
            return {"Dashboard", "Tests", "Warnings", "Grade", "Stats", "Proof"};
        }
        if (topTab == "Evidence") {
            return {"Dashboard", "Proof", "Commands", "Failures", "Corrections", "Promotion", "Relations"};
        }
        return {"Dashboard"};
    }
    if (topTab == "Profile") {
        return {"Dashboard", "Scope", "Model", "Evidence", "Rules", "Maint", "Future"};
    }
    if (topTab == "Stats") {
        return {"Dashboard", "Routing", "Tasks", "Models", "Input", "Evidence"};
    }
    if (topTab == "Relationship") {
        return {"Dashboard", "Flow", "Inbound", "Outbound", "Failures", "Rules", "Metrics", "Strategy"};
    }
    if (topTab == "Grade") {
        return {"Dashboard", "Verdict", "Packet", "Model Fit", "Evidence", "Routing", "Workflow", "Rubric"};
    }
    if (topTab == "Transcript") {
        return {"Dashboard", "Segments", "Excerpts", "Tools", "Artifacts", "Missing", "Warnings", "Scan", "Monitor", "Report"};
    }
    if (topTab == "Evidence") {
        return {"Dashboard", "Proof", "Failures", "Corrections", "Acceptance", "Chains", "Missing"};
    }
    return {"Dashboard"};
}

QString detailLensShortLabel(const QString &lens) {
    if (lens == "Dashboard") {
        return "Dash";
    }
    if (lens == "Model Fit") {
        return "Fit";
    }
    if (lens == "Acceptance") {
        return "Accept";
    }
    if (lens.size() <= 6) {
        return lens;
    }
    return lens.left(4);
}

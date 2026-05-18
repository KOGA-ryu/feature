#include "rust_cockpit_backend.h"

#include <QDateTime>

#include "binder_state.h"

CockpitState RustCockpitBackend::fallbackState(const QString &error) const {
    CockpitState state;
    state.backendAvailable = false;
    state.backendError = error;
    state.binder.selectedWorkerId = "planner";
    state.binder.statsSnapshot = DexBinder::noDataStats("planner");
    state.projects.push_back({"dex_home", "dex_home", repoRoot_, "backend_missing", true, {"planner"}});
    state.workers.push_back({"planner", "planner", "Planner", "hold"});
    state.ledgerEntries.push_back({
        "fallback-0001",
        "01",
        QDateTime::currentDateTimeUtc().toString(Qt::ISODate),
        "context_note_added",
        "dex_home",
        repoRoot_,
        "planner",
        "backend-fallback",
        "Rust cockpit core unavailable.",
        error,
        "hold",
        "n/a",
        {error},
        {},
        {},
        {},
        {},
        "",
        "",
        "missing",
        {"fallback"},
        "qt_fallback",
    });
    state.contextPanel.title = "Backend unavailable";
    state.contextPanel.subtitle = binaryPath_;
    state.contextPanel.lines = {error};
    return state;
}

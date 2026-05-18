#include "rust_cockpit_backend.h"

#include <utility>

RustCockpitBackend::RustCockpitBackend(QString repoRoot, QString binaryPath)
    : repoRoot_(std::move(repoRoot)), binaryPath_(std::move(binaryPath)) {}

const QString &RustCockpitBackend::repoRoot() const {
    return repoRoot_;
}

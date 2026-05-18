#include "rust_cockpit_backend.h"

#include <QCoreApplication>
#include <QDir>
#include <QFileInfo>
#include <QStringList>

QString RustCockpitBackend::defaultRepoRoot() {
    const QString envRoot = qEnvironmentVariable("DEX_HOME_REPO_ROOT");
    if (!envRoot.isEmpty()) {
        return QDir(envRoot).absolutePath();
    }

    const QStringList starts = {QDir::currentPath(), QCoreApplication::applicationDirPath()};
    for (const QString &start : starts) {
        QDir dir(start);
        for (int depth = 0; depth < 8; ++depth) {
            if (QFileInfo::exists(dir.filePath("rust/Cargo.toml")) &&
                QFileInfo::exists(dir.filePath("native/dex_home_v2"))) {
                return dir.absolutePath();
            }
            if (!dir.cdUp()) {
                break;
            }
        }
    }
    const QString localAuthority = QDir::home().filePath("dev/dex_home");
    if (QFileInfo::exists(QDir(localAuthority).filePath("rust/Cargo.toml")) &&
        QFileInfo::exists(QDir(localAuthority).filePath("native/dex_home_v2"))) {
        return QDir(localAuthority).absolutePath();
    }
    return QDir::currentPath();
}

QString RustCockpitBackend::defaultBinaryPath(const QString &repoRoot) {
    const QString envBinary = qEnvironmentVariable("DEX_COCKPIT_CORE_BIN");
    if (!envBinary.isEmpty()) {
        return envBinary;
    }

    const QStringList candidates = {
        QDir(repoRoot).filePath("rust/target/debug/dex-cockpit-core"),
        QDir(QCoreApplication::applicationDirPath()).filePath("../../../rust/target/debug/dex-cockpit-core"),
    };
    for (const QString &candidate : candidates) {
        if (QFileInfo::exists(candidate)) {
            return QDir::cleanPath(candidate);
        }
    }
    return QDir(repoRoot).filePath("rust/target/debug/dex-cockpit-core");
}

#include "rust_cockpit_backend.h"

#include <QJsonParseError>
#include <QProcess>

namespace {
constexpr int kBackendTimeoutMs = 8000;
}

QJsonDocument RustCockpitBackend::runJson(const QStringList &arguments, const QByteArray &input, QString *error) const {
    if (error) {
        error->clear();
    }
    QProcess process;
    process.setProgram(binaryPath_);
    process.setArguments(arguments);
    process.setProcessChannelMode(QProcess::SeparateChannels);
    process.start();
    if (!process.waitForStarted(3000)) {
        if (error) {
            *error = QString("Failed to start Rust cockpit core: %1").arg(binaryPath_);
        }
        return {};
    }

    if (!input.isEmpty()) {
        process.write(input);
    }
    process.closeWriteChannel();

    if (!process.waitForFinished(kBackendTimeoutMs)) {
        process.kill();
        if (error) {
            *error = "Rust cockpit core timed out.";
        }
        return {};
    }

    const QString stderrText = QString::fromUtf8(process.readAllStandardError()).trimmed();
    const QByteArray stdoutBytes = process.readAllStandardOutput();
    if (process.exitStatus() != QProcess::NormalExit || process.exitCode() != 0) {
        if (error) {
            *error = stderrText.isEmpty() ? "Rust cockpit core failed." : stderrText;
        }
        return {};
    }

    QJsonParseError parseError;
    const QJsonDocument document = QJsonDocument::fromJson(stdoutBytes, &parseError);
    if (parseError.error != QJsonParseError::NoError) {
        if (error) {
            *error = QString("Rust cockpit core returned invalid JSON: %1").arg(parseError.errorString());
        }
        return {};
    }
    return document;
}

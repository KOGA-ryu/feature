#include <QApplication>
#include <QCommandLineOption>
#include <QCommandLineParser>
#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QIODevice>
#include <QPixmap>
#include <QRect>
#include <QSize>
#include <QStringList>
#include <QTimer>
#include <QVariant>
#include <QWidget>

#include "main_window.h"
#include "render_helpers.h"
#include "rust_cockpit_backend.h"
#include "ui_rules.h"

namespace {

constexpr int kDesignWidth = dex_ui::metrics::design_width;
constexpr int kDesignHeight = dex_ui::metrics::design_height;
QSize parseSize(const QString &value) {
    const QStringList parts = value.split('x');
    if (parts.size() != 2) {
        return QSize(kDesignWidth, kDesignHeight);
    }
    bool okW = false;
    bool okH = false;
    const int width = parts.at(0).toInt(&okW);
    const int height = parts.at(1).toInt(&okH);
    if (!okW || !okH || width <= 0 || height <= 0) {
        return QSize(kDesignWidth, kDesignHeight);
    }
    return QSize(width, height);
}

QJsonObject dumpWidgetTree(QWidget *widget) {
    QJsonObject node;
    node.insert("className", widget->metaObject()->className());
    node.insert("objectName", widget->objectName());
    node.insert("visible", widget->isVisible());
    const QVariant uiPath = widget->property("uiPath");
    if (uiPath.isValid()) {
        node.insert("uiPath", uiPath.toString());
    }
    const QVariant componentState = widget->property("componentState");
    if (componentState.isValid()) {
        node.insert("componentState", componentState.toString());
    }
    const QVariant workspace = widget->property("workspace");
    if (workspace.isValid()) {
        node.insert("workspace", workspace.toString());
    }

    const QRect geometry = widget->geometry();
    node.insert("geometry", QJsonObject{
        {"x", geometry.x()},
        {"y", geometry.y()},
        {"width", geometry.width()},
        {"height", geometry.height()},
    });

    QJsonArray children;
    for (QObject *child : widget->children()) {
        auto *childWidget = qobject_cast<QWidget *>(child);
        if (!childWidget || !childWidget->isVisible()) {
            continue;
        }
        children.append(dumpWidgetTree(childWidget));
    }
    node.insert("children", children);
    return node;
}

bool writeWidgetTreeDump(QWidget *root, const QString &path) {
    const QFileInfo info(path);
    QDir().mkpath(info.absolutePath());
    QFile file(path);
    if (!file.open(QIODevice::WriteOnly | QIODevice::Truncate)) {
        return false;
    }
    const QJsonDocument document(dumpWidgetTree(root));
    file.write(document.toJson(QJsonDocument::Indented));
    return true;
}

} // namespace

int main(int argc, char **argv) {
    QApplication app(argc, argv);
    QApplication::setStyle("Fusion");
    app.setFont(dex_ui::app_font());
    app.setStyleSheet(appStyleSheet());

    QCommandLineParser parser;
    parser.setApplicationDescription("Dex Home v2 native Project Rail / Worker Ledger cockpit");
    parser.addHelpOption();
    const QCommandLineOption screenshotOption("screenshot", "Save a screenshot and exit.", "path");
    const QCommandLineOption sizeOption("size", "Window size for normal/proof launch, e.g. 1280x800.", "size", "1280x800");
    const QCommandLineOption hideRailOption("hide-rail", "Start with the project rail hidden.");
    const QCommandLineOption hideContextOption("hide-inspector", "Start with the right context surface hidden.");
    const QCommandLineOption repoRootOption("repo-root", "Dex Home repo root for Rust state.", "path");
    const QCommandLineOption cockpitCoreOption("cockpit-core-bin", "Path to dex-cockpit-core binary.", "path");
    const QCommandLineOption projectRegistryOption("project-registry", "Read-only project registry JSON path.", "path");
    const QCommandLineOption proofReceiptOption("proof-receipt", "Read-only proof receipt manifest path.", "path");
    const QCommandLineOption projectOption("project", "Initial selected project id.", "project-id");
    const QCommandLineOption tabOption("tab", "Initial binder top tab.", "tab", "Profile");
    const QCommandLineOption detailLensOption("detail-lens", "Initial right detail lens.", "lens");
    const QCommandLineOption workerOption("worker", "Initial worker lens.", "worker-id");
    const QCommandLineOption repoBinderOption("repo-binder", "Start in repo binder mode.");
    const QCommandLineOption settingsOption("settings", "Start on a Settings workspace.");
    const QCommandLineOption noSettingsOption("no-settings", "Start on the normal repo binder instead of the active Settings workbench.");
    const QCommandLineOption settingsFeatureOption("settings-feature", "Initial Settings feature, e.g. Project Spec.", "feature", "Project Spec");
    const QCommandLineOption uiTreeDumpOption("ui-tree-dump", "Save visible UI tree JSON and exit.", "path");
    parser.addOption(screenshotOption);
    parser.addOption(sizeOption);
    parser.addOption(hideRailOption);
    parser.addOption(hideContextOption);
    parser.addOption(repoRootOption);
    parser.addOption(cockpitCoreOption);
    parser.addOption(projectRegistryOption);
    parser.addOption(proofReceiptOption);
    parser.addOption(projectOption);
    parser.addOption(tabOption);
    parser.addOption(detailLensOption);
    parser.addOption(workerOption);
    parser.addOption(repoBinderOption);
    parser.addOption(settingsOption);
    parser.addOption(noSettingsOption);
    parser.addOption(settingsFeatureOption);
    parser.addOption(uiTreeDumpOption);
    parser.process(app);

    const QString repoRoot = parser.isSet(repoRootOption)
        ? QDir(parser.value(repoRootOption)).absolutePath()
        : RustCockpitBackend::defaultRepoRoot();
    const QString cockpitCore = parser.isSet(cockpitCoreOption)
        ? parser.value(cockpitCoreOption)
        : RustCockpitBackend::defaultBinaryPath(repoRoot);

    const QString projectRegistry = parser.isSet(projectRegistryOption)
        ? parser.value(projectRegistryOption)
        : QString();
    const QString proofReceipt = parser.isSet(proofReceiptOption)
        ? parser.value(proofReceiptOption)
        : QString();

    DexHomeV2Window window(repoRoot, cockpitCore, projectRegistry, proofReceipt);
    if (parser.isSet(repoBinderOption)) {
        window.setRepoBinderMode(true);
    }
    if (parser.isSet(projectOption)) {
        window.setSelectedProject(parser.value(projectOption));
    }
    if (parser.isSet(workerOption)) {
        window.setSelectedWorker(parser.value(workerOption));
    }
    window.setTopTab(parser.value(tabOption));
    if (parser.isSet(detailLensOption)) {
        window.setDetailLens(parser.value(detailLensOption));
    }
    const bool startSettings = parser.isSet(settingsOption) || !parser.isSet(noSettingsOption);
    if (startSettings) {
        window.setSettingsFeature(parser.value(settingsFeatureOption));
        window.setSettingsMode(true);
    }
    if (parser.isSet(hideRailOption)) {
        window.setProjectRailVisible(false);
    }
    if (parser.isSet(hideContextOption)) {
        window.setRightContextVisible(false);
    }
    const QSize size = parseSize(parser.value(sizeOption));
    window.resize(size);
    window.show();

    if (parser.isSet(screenshotOption) || parser.isSet(uiTreeDumpOption)) {
        const QString path = parser.value(screenshotOption);
        const QString uiTreePath = parser.value(uiTreeDumpOption);
        const bool shouldScreenshot = parser.isSet(screenshotOption);
        const bool shouldDumpUiTree = parser.isSet(uiTreeDumpOption);
        QTimer::singleShot(350, &app, [&window, path, uiTreePath, shouldScreenshot, shouldDumpUiTree]() {
            if (shouldScreenshot) {
                const QFileInfo info(path);
                QDir().mkpath(info.absolutePath());
                QPixmap pixmap = window.grab();
                pixmap.save(path);
            }
            if (shouldDumpUiTree) {
                writeWidgetTreeDump(&window, uiTreePath);
            }
            QApplication::quit();
        });
    }

    return app.exec();
}

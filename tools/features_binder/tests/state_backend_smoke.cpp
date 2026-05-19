#include <QFileInfo>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QProcess>
#include <QTemporaryDir>
#include <QtTest/QtTest>

#include <algorithm>

#include "binder_state.h"
#include "app_state_helpers.h"
#include "binder_navigation.h"
#include "project_registry.h"
#include "project_registry_spec_model.h"
#include "repo_binder_template.h"
#include "settings_shortcuts_model.h"
#include "text_action_proof_model.h"
#include "text_editor_rust_action_client.h"

class StateBackendSmoke final : public QObject {
    Q_OBJECT

private:
    void runJsonCommand(const QString &binary, const QStringList &arguments, QJsonDocument *document) {
        QProcess process;
        process.start(binary, arguments);
        QVERIFY(process.waitForStarted(3000));
        QVERIFY(process.waitForFinished(8000));
        QCOMPARE(process.exitStatus(), QProcess::NormalExit);
        QCOMPARE(process.exitCode(), 0);

        QJsonParseError parseError;
        *document = QJsonDocument::fromJson(process.readAllStandardOutput(), &parseError);
        QCOMPARE(parseError.error, QJsonParseError::NoError);
    }

private slots:
    void derivesStateFromRustCore() {
        const QString binary = QString::fromUtf8(DEX_COCKPIT_CORE_BIN);
        if (!QFileInfo::exists(binary)) {
            QSKIP(qPrintable("Missing dex-cockpit-core binary: " + binary));
        }

        QTemporaryDir repoRoot;
        QVERIFY(repoRoot.isValid());

        QJsonDocument document;
        runJsonCommand(binary, {"state", "derive", "--repo-root", repoRoot.path()}, &document);

        const QJsonObject root = document.object();
        QCOMPARE(root.value("selected_project_id").toString(), QString("dex_home"));
        QCOMPARE(root.value("selected_worker_id").toString(), QString("planner"));
        QCOMPARE(root.value("projects").toArray().size(), 1);
        QVERIFY(root.value("workers").toArray().size() >= 4);
        QVERIFY(!root.value("ledger_entries").toArray().isEmpty());
        QCOMPARE(root.value("sessions").toArray().size(), 0);
        QCOMPARE(root.value("packets").toArray().size(), 0);
        QVERIFY(root.value("command_plans").toArray().size() >= 4);
        QVERIFY(!root.value("profiles").toArray().isEmpty());
        QCOMPARE(root.value("selected_profile_id").toString(), QString("dex_terminal_default"));
        QCOMPARE(root.value("profile_graph").toObject().value("nodes").toArray().size(), 6);
        QCOMPARE(root.value("launch_plan_preview").toObject().value("instances").toArray().size(), 6);

        bool sawBuildProtocol = false;
        for (const QJsonValue &value : root.value("bench_candidates").toArray()) {
            if (value.toObject().value("candidate_id").toString() == "build_protocol") {
                sawBuildProtocol = true;
            }
            QVERIFY(value.toObject().contains("expected_artifact_outputs"));
        }
        QVERIFY(sawBuildProtocol);

        QVERIFY(root.contains("stats_snapshot"));
        QVERIFY(root.contains("grade_records"));
        QVERIFY(root.contains("transcript_records"));
        QVERIFY(root.contains("evidence_records"));
        const DexBinder::BinderState binder = DexBinder::parseBinderState(root);
        QCOMPARE(binder.statsSnapshot.confidence, QString("no_data"));
        QCOMPARE(binder.gradeRecords.size(), 0);
        QCOMPARE(binder.transcriptRecords.size(), 0);
        QCOMPARE(binder.evidenceRecords.size(), 0);
    }

    void exposesProfilePlannerCommands() {
        const QString binary = QString::fromUtf8(DEX_COCKPIT_CORE_BIN);
        if (!QFileInfo::exists(binary)) {
            QSKIP(qPrintable("Missing dex-cockpit-core binary: " + binary));
        }

        QTemporaryDir repoRoot;
        QVERIFY(repoRoot.isValid());

        QJsonDocument listDocument;
        runJsonCommand(binary, {"profile", "list", "--repo-root", repoRoot.path()}, &listDocument);
        const QJsonArray profiles = listDocument.array();
        QVERIFY(!profiles.isEmpty());
        const QString profileId = profiles.first().toObject().value("profile_id").toString();
        QCOMPARE(profileId, QString("dex_terminal_default"));

        QJsonDocument graphDocument;
        runJsonCommand(
            binary,
            {"profile", "graph", "--repo-root", repoRoot.path(), "--profile-id", profileId},
            &graphDocument);
        const QJsonObject graph = graphDocument.object();
        QCOMPARE(graph.value("nodes").toArray().size(), profiles.first().toObject().value("roles").toArray().size());
        QVERIFY(!graph.value("edges").toArray().isEmpty());

        QJsonDocument launchDocument;
        runJsonCommand(
            binary,
            {"profile", "launch-plan", "--repo-root", repoRoot.path(), "--profile-id", profileId, "--count", "6"},
            &launchDocument);
        const QJsonObject launchPlan = launchDocument.object();
        QCOMPARE(launchPlan.value("instances").toArray().size(), 6);
        QVERIFY(launchPlan.value("instances").toArray().first().toObject().value("startup_brief").toString().contains("Dex Role:"));
    }

    void binderTopTabsAreStable() {
        QCOMPARE(
            DexBinder::binderTopTabs(),
            QStringList({"Profile", "Stats", "Relationship", "Grade", "Transcript", "Evidence"}));
    }

    void parsesDefaultRepoBinderTemplate() {
        const QString path = QString::fromUtf8(DEX_HOME_V2_SOURCE_DIR)
            + "/data/binder_templates/repo_default_binder_v1.json";
        const DexRepoBinderTemplate::BinderTemplate templateRecord =
            DexRepoBinderTemplate::loadBinderTemplateFile(path);

        QVERIFY2(templateRecord.loaded, qPrintable(templateRecord.error));
        QCOMPARE(templateRecord.templateId, QString("repo_default_binder_v1"));
        QVERIFY(templateRecord.topTabs.contains("Profile"));
        QVERIFY(templateRecord.detailLenses.value("Inventory").contains("Files"));
        QCOMPARE(templateRecord.contextLines.size(), 0);
        QCOMPARE(templateRecord.sections.size(), 0);
    }

    void parsesFeaturesRepoBinderTemplate() {
        const QString path = QString::fromUtf8(DEX_HOME_V2_SOURCE_DIR)
            + "/data/binder_templates/features_feature_foundry_v1.json";
        const DexRepoBinderTemplate::BinderTemplate templateRecord =
            DexRepoBinderTemplate::loadBinderTemplateFile(path);

        QVERIFY2(templateRecord.loaded, qPrintable(templateRecord.error));
        QCOMPARE(templateRecord.templateId, QString("features_feature_foundry_v1"));
        QCOMPARE(templateRecord.displayName, QString("Features Feature Foundry Binder"));
        QVERIFY(templateRecord.topTabs.contains("Inventory"));
        QVERIFY(templateRecord.contextLines.contains("feature packs {{features.feature_pack_total}}"));
        QVERIFY(templateRecord.sections.size() >= 4);
    }

    void productionProjectRegistryIsFeaturesBinder() {
        const QString path = QString::fromUtf8(DEX_HOME_V2_SOURCE_DIR) + "/data/projects.json";
        const DexProjects::ProjectRegistry registry = DexProjects::loadProjectRegistryFile(path);

        QVERIFY2(registry.loaded, qPrintable(registry.error));
        QCOMPARE(registry.projects.size(), 1);
        QCOMPARE(registry.projects.first().projectId, QString("features"));
        QCOMPARE(registry.projects.first().path, QString("/Users/kogaryu/dev/features"));
        QCOMPARE(registry.projects.first().binderTemplate, QString("features_feature_foundry_v1"));
        QCOMPARE(registry.projects.first().featurePackTotal, 48);
        QCOMPARE(registry.projects.first().featurePackLogic, 17);
        QCOMPARE(registry.projects.first().featurePackSim, 6);
        QCOMPARE(registry.projects.first().featurePackUi, 16);
        QCOMPARE(registry.projects.first().featurePackWorkflows, 9);
        QCOMPARE(registry.projects.first().workerIds, QStringList({"organizer", "planner", "stager"}));
        QCOMPARE(registry.workers.size(), 3);
        QCOMPARE(registry.workers.first().codexSessionId, QString());
    }

    void binderTemplateParserHandlesMissingOptionalsAndUnknownFields() {
        const QJsonDocument document(QJsonObject{
            {"template_id", "minimal_template"},
            {"unknown_future_field", "ignored"},
            {"sections", QJsonArray{QJsonObject{
                {"tab", "Profile"},
                {"title", "minimal section"},
                {"unknown_section_field", "ignored"},
            }}},
        });

        const DexRepoBinderTemplate::BinderTemplate templateRecord =
            DexRepoBinderTemplate::parseBinderTemplateDocument(document);

        QVERIFY(templateRecord.loaded);
        QCOMPARE(templateRecord.templateId, QString("minimal_template"));
        QCOMPARE(templateRecord.topTabs.size(), 0);
        QCOMPARE(templateRecord.contextLines.size(), 0);
        QCOMPARE(templateRecord.sections.size(), 1);
        QCOMPARE(templateRecord.sections.first().rows.size(), 0);
        QCOMPARE(templateRecord.sections.first().lenses.size(), 0);
    }

    void binderTemplateStoreFallsBackToDefault() {
        QTemporaryDir emptyDir;
        QVERIFY(emptyDir.isValid());

        const DexRepoBinderTemplate::BinderTemplateStore store =
            DexRepoBinderTemplate::loadBinderTemplateStore(emptyDir.path());
        const DexRepoBinderTemplate::BinderTemplate resolved =
            DexRepoBinderTemplate::resolveTemplateForProject(store, "missing_template");

        QCOMPARE(resolved.templateId, QString("repo_default_binder_v1"));
    }

    void binderTemplateSaveRoundTripsEditableSections() {
        QTemporaryDir dir;
        QVERIFY(dir.isValid());

        DexRepoBinderTemplate::BinderTemplate templateRecord;
        templateRecord.templateId = "custom_repo_v1";
        templateRecord.displayName = "Custom Repo";
        templateRecord.topTabs = {"Profile", "Inventory", "Quality"};
        templateRecord.detailLenses.insert("Profile", {"Dashboard", "Team"});
        templateRecord.contextLines = {"template custom_repo_v1", "source settings"};
        DexRepoBinderTemplate::BinderTemplateSection section;
        section.tab = "Profile";
        section.lenses = {"Dashboard"};
        section.title = "custom section";
        section.subtle = true;
        section.rows = {
            DexRepoBinderTemplate::BinderTemplateRow{{"field", "value"}, false},
            DexRepoBinderTemplate::BinderTemplateRow{{"risk", "inspect first"}, true},
        };
        templateRecord.sections = {section};

        const QString path = dir.filePath("custom_repo_v1.json");
        QString error;
        QVERIFY2(DexRepoBinderTemplate::saveBinderTemplateFile(templateRecord, path, &error), qPrintable(error));

        const DexRepoBinderTemplate::BinderTemplate loaded =
            DexRepoBinderTemplate::loadBinderTemplateFile(path);
        QVERIFY2(loaded.loaded, qPrintable(loaded.error));
        QCOMPARE(loaded.templateId, QString("custom_repo_v1"));
        QCOMPARE(loaded.contextLines, QStringList({"template custom_repo_v1", "source settings"}));
        QCOMPARE(loaded.sections.size(), 1);
        QCOMPARE(loaded.sections.first().title, QString("custom section"));
        QCOMPARE(loaded.sections.first().rows.size(), 2);
        QVERIFY(loaded.sections.first().rows.at(1).risk);
    }

    void selectedProjectResolvesExpectedBinderTemplate() {
        const QJsonDocument registryDocument(QJsonObject{{"projects", QJsonArray{
            QJsonObject{{"project_id", "plain"}, {"name", "Plain Repo"}},
            QJsonObject{
                {"project_id", "features"},
                {"name", "features"},
                {"binder_template", "features_feature_foundry_v1"},
                {"worker_ids", QJsonArray{"organizer", "planner", "stager"}},
            },
        }}});
        const DexProjects::ProjectRegistry registry = DexProjects::parseProjectRegistryDocument(registryDocument);

        const DexProjects::ProjectRegistryEntry *plain = DexProjects::findProjectById(registry.projects, "plain");
        const DexProjects::ProjectRegistryEntry *features = DexProjects::findProjectById(registry.projects, "features");
        QVERIFY(plain != nullptr);
        QVERIFY(features != nullptr);
        QCOMPARE(plain->binderTemplate, QString());
        QCOMPARE(features->binderTemplate, QString("features_feature_foundry_v1"));
        QCOMPARE(features->workerIds, QStringList({"organizer", "planner", "stager"}));
    }

    void textActionProofModelRendersGeneratedActions() {
        const QVector<DexTextActions::HostActionItem> actions =
            DexTextActions::renderHostActionItems("hello", {});

        QCOMPARE(actions.size(), DexTextActions::textActionRecords().size());
        const auto copyIt = std::find_if(actions.begin(), actions.end(), [](const DexTextActions::HostActionItem &item) {
            return item.actionId == "text.copy_plain";
        });
        QVERIFY(copyIt != actions.end());
        QCOMPARE(copyIt->label, QString("Copy Plain"));
        QCOMPARE(copyIt->hotkeyLabel, QString("Ctrl+C"));
        QVERIFY(copyIt->enabled);
        QVERIFY(copyIt->disabledReason.isEmpty());
    }

    void parsesTextEditorRustRunnerResponses() {
        const QByteArray payload = R"({
            "ok": true,
            "result": {
                "action_id": "text.clean_basic",
                "kind": "clipboard_transform",
                "display_text": "Clipboard text ready. 2 changes, 0 warnings.",
                "clipboard_text": "clean",
                "receipt_summary": {
                    "change_count": 2,
                    "warning_count": 0,
                    "changes": ["normalized_line_endings: 1", "trimmed_trailing_whitespace: 1"],
                    "warnings": []
                },
                "warnings": []
            },
            "editor_text": "dirty",
            "selection": {
                "anchor": {"line": 0, "column": 0},
                "caret": {"line": 0, "column": 5}
            }
        })";

        const DexTextEditorRust::ActionResult result =
            DexTextEditorRust::parseActionRunnerResponse(payload);

        QVERIFY(result.ok);
        QCOMPARE(result.actionId, QString("text.clean_basic"));
        QCOMPARE(result.kind, QString("clipboard_transform"));
        QVERIFY(result.hasClipboardText);
        QCOMPARE(result.clipboardText, QString("clean"));
        QCOMPARE(result.receipt.changeCount, 2);
        QCOMPARE(result.receipt.changes.size(), 2);
        QVERIFY(result.selection.valid);
        QCOMPARE(result.selection.caret.column, 5);
    }

    void malformedTextEditorRustRunnerResponseIsUnavailable() {
        const DexTextEditorRust::ActionResult result =
            DexTextEditorRust::parseActionRunnerResponse("not json");

        QVERIFY(!result.ok);
        QVERIFY(result.error.contains("malformed runner json"));
        QVERIFY(!result.hasClipboardText);
    }

    void repoBinderPromotesTextEditorAsMainSubject() {
        const QStringList tabs = repoBinderTopTabs();
        const int contractsIndex = tabs.indexOf("Contracts");
        const int textEditorIndex = tabs.indexOf("Text Editor");
        const int activityIndex = tabs.indexOf("Activity");

        QVERIFY(contractsIndex >= 0);
        QVERIFY(textEditorIndex >= 0);
        QVERIFY(activityIndex >= 0);
        QVERIFY(contractsIndex < textEditorIndex);
        QVERIFY(textEditorIndex < activityIndex);

        QCOMPARE(
            detailLensTabsFor("Text Editor", true),
            QStringList({"Dashboard", "Editor", "Actions", "Inspector", "Fixtures", "Receipts", "Proof"}));
    }

    void textActionProofModelDisablesEmptyCleanupHonestly() {
        const QVector<DexTextActions::HostActionItem> actions =
            DexTextActions::renderHostActionItems("", {});
        const auto cleanIt = std::find_if(actions.begin(), actions.end(), [](const DexTextActions::HostActionItem &item) {
            return item.actionId == "text.clean_basic";
        });

        QVERIFY(cleanIt != actions.end());
        QVERIFY(!cleanIt->enabled);
        QCOMPARE(cleanIt->disabledReason, QString("document and input text are empty"));
    }

    void textActionProofModelReportsCleanupReceipts() {
        DexTextActions::TextActionProofInput input;
        input.stripAnsiEscapeCodes = true;

        const DexTextActions::HostActionResult result =
            DexTextActions::executeTextActionProof(
                "text.clean_basic",
                "one\r\n\u001b[31mtwo\u001b[0m",
                {},
                input);

        QCOMPARE(result.kind, QString("clipboard_transform"));
        QCOMPARE(result.clipboardText, QString("one\ntwo"));
        QCOMPARE(result.receipt.changeCount, 3);
        QCOMPARE(
            result.receipt.changes,
            QStringList({"normalized_line_endings: 1", "stripped_ansi_escape_codes: 2"}));
    }

    void textActionFixtureCatalogContainsExpectedCases() {
        const QVector<DexTextActions::TextActionFixture> fixtures = DexTextActions::textActionFixtures();

        QVERIFY(fixtures.size() >= 5);
        const auto promptIt = std::find_if(fixtures.begin(), fixtures.end(), [](const DexTextActions::TextActionFixture &fixture) {
            return fixture.fixtureId == "copy_prompt_block_source";
        });
        QVERIFY(promptIt != fixtures.end());
        QCOMPARE(promptIt->actionId, QString("text.copy_prompt_block"));
        QVERIFY(promptIt->expectedClipboardText.contains("Source: features_binder"));
    }

    void textActionFixturesPassExpectedOutputChecks() {
        for (const DexTextActions::TextActionFixture &fixture : DexTextActions::textActionFixtures()) {
            const DexTextActions::TextActionFixtureResult result =
                DexTextActions::runTextActionFixture(fixture.fixtureId);
            QVERIFY2(result.passed, qPrintable(result.summary));
            QCOMPARE(result.actualClipboardText, fixture.expectedClipboardText);
        }
    }

    void textActionFixtureSuiteAggregatesPassFailCounts() {
        const DexTextActions::TextActionFixtureSuiteResult suite =
            DexTextActions::runAllTextActionFixtures();

        QVERIFY(suite.allPassed);
        QCOMPARE(suite.total, DexTextActions::textActionFixtures().size());
        QCOMPARE(suite.passed, suite.total);
        QCOMPARE(suite.failed, 0);
        QVERIFY(suite.summary.startsWith("PASS all fixtures"));
    }

    void textActionFixtureRunnerReportsMissingFixture() {
        const DexTextActions::TextActionFixtureResult result =
            DexTextActions::runTextActionFixture("missing_fixture");

        QVERIFY(!result.passed);
        QCOMPARE(result.summary, QString("Fixture not found."));
        QVERIFY(result.actualClipboardText.isEmpty());
    }

    void settingsShortcutTabsIncludeCodexAppShortcuts() {
        const QVector<DexSettingsShortcuts::ShortcutCommandTab> tabs =
            DexSettingsShortcuts::shortcutCommandTabs();
        const auto textEditorTab = std::find_if(tabs.begin(), tabs.end(), [](const DexSettingsShortcuts::ShortcutCommandTab &tab) {
            return tab.tabName == "Text Editor";
        });
        const auto oldTextActionsTab = std::find_if(tabs.begin(), tabs.end(), [](const DexSettingsShortcuts::ShortcutCommandTab &tab) {
            return tab.tabName == "Text Actions";
        });
        QVERIFY(textEditorTab != tabs.end());
        QVERIFY(oldTextActionsTab == tabs.end());

        const auto codexTab = std::find_if(tabs.begin(), tabs.end(), [](const DexSettingsShortcuts::ShortcutCommandTab &tab) {
            return tab.tabName == "OpenAI Codex";
        });

        QVERIFY(codexTab != tabs.end());
        QVERIFY(codexTab->records.size() >= 60);
        const auto sessionIt = std::find_if(
            codexTab->records.begin(),
            codexTab->records.end(),
            [](const DexSettingsShortcuts::ShortcutCommandRecord &record) {
                return record.command == "Copy session id";
            });
        QVERIFY(sessionIt != codexTab->records.end());
        QCOMPARE(sessionIt->shortcut, QString("⌥⌘C"));
        const auto shortcutsIt = std::find_if(
            codexTab->records.begin(),
            codexTab->records.end(),
            [](const DexSettingsShortcuts::ShortcutCommandRecord &record) {
                return record.command == "Show keyboard shortcuts";
            });
        QVERIFY(shortcutsIt != codexTab->records.end());
        QCOMPARE(shortcutsIt->shortcut, QString("⌘?"));
    }

    void projectRegistryParsesWorkerCatalog() {
        const QJsonDocument registryDocument(QJsonObject{
            {"workers", QJsonArray{
                QJsonObject{
                    {"worker_id", "organizer"},
                    {"role", "repo_organizer"},
                    {"display_name", "Organizer"},
                    {"status", "ready"},
                },
            }},
            {"projects", QJsonArray{}},
        });
        const DexProjects::ProjectRegistry registry = DexProjects::parseProjectRegistryDocument(registryDocument);

        QVERIFY(registry.loaded);
        QCOMPARE(registry.workers.size(), 1);
        QCOMPARE(registry.workers.first().workerId, QString("organizer"));
        QCOMPARE(registry.workers.first().role, QString("repo_organizer"));
        QCOMPARE(registry.workers.first().displayName, QString("Organizer"));
        QCOMPARE(registry.workers.first().status, QString("ready"));
        QCOMPARE(registry.workers.first().codexSessionId, QString());
        QCOMPARE(registry.workers.first().codexModel, QString());
    }

    void projectRegistryParsesWorkerCodexSessionFields() {
        const QJsonDocument registryDocument(QJsonObject{
            {"workers", QJsonArray{
                QJsonObject{
                    {"worker_id", "planner"},
                    {"role", "repo_planner"},
                    {"display_name", "Planner"},
                    {"status", "ready"},
                    {"codex_session_id", "019e3782-3d83-7da0-83bc-ab69315c9675"},
                    {"codex_model", "gpt-5.3-codex-spark"},
                    {"codex_directory", "~/dev/features"},
                    {"codex_permissions", "Workspace (on-request)"},
                    {"codex_context_window", "73% left (41.7K used / 122K)"},
                    {"codex_account", "dethislikethewind@gmail.com (Pro)"},
                },
            }},
            {"projects", QJsonArray{}},
        });
        const DexProjects::ProjectRegistry registry = DexProjects::parseProjectRegistryDocument(registryDocument);

        QVERIFY(registry.loaded);
        QCOMPARE(registry.workers.size(), 1);
        QCOMPARE(registry.workers.first().codexSessionId, QString("019e3782-3d83-7da0-83bc-ab69315c9675"));
        QCOMPARE(registry.workers.first().codexModel, QString("gpt-5.3-codex-spark"));
        QCOMPARE(registry.workers.first().codexDirectory, QString("~/dev/features"));
        QCOMPARE(registry.workers.first().codexPermissions, QString("Workspace (on-request)"));
        QCOMPARE(registry.workers.first().codexContextWindow, QString("73% left (41.7K used / 122K)"));
        QCOMPARE(registry.workers.first().codexAccount, QString("dethislikethewind@gmail.com (Pro)"));
    }

    void projectRegistrySaveRoundTripsWorkersAndProjects() {
        QTemporaryDir dir;
        QVERIFY(dir.isValid());

        DexProjects::ProjectRegistry registry;
        registry.loaded = true;
        registry.sourcePath = dir.filePath("projects.json");
        DexProjects::WorkerRegistryEntry worker;
        worker.workerId = "organizer";
        worker.role = "repo_organizer";
        worker.displayName = "Organizer";
        worker.status = "ready";
        worker.codexSessionId = "019e3782-3d83-7da0-83bc-ab69315c9675";
        worker.codexModel = "gpt-5.3-codex-spark";
        worker.codexDirectory = "~/dev/features";
        worker.codexPermissions = "Workspace (on-request)";
        worker.codexContextWindow = "73% left (41.7K used / 122K)";
        worker.codexAccount = "dethislikethewind@gmail.com (Pro)";
        registry.workers.push_back(worker);
        DexProjects::ProjectRegistryEntry project;
        project.projectId = "features";
        project.name = "features";
        project.path = "/Users/kogaryu/dev/features";
        project.role = "feature foundry";
        project.status = "active";
        project.pinned = true;
        project.workerIds = {"organizer"};
        registry.projects.push_back(project);

        QString error;
        QVERIFY2(DexProjects::saveProjectRegistryFile(registry, &error), qPrintable(error));
        const DexProjects::ProjectRegistry loaded = DexProjects::loadProjectRegistryFile(registry.sourcePath);

        QVERIFY2(loaded.loaded, qPrintable(loaded.error));
        QCOMPARE(loaded.workers.size(), 1);
        QCOMPARE(loaded.workers.first().workerId, QString("organizer"));
        QCOMPARE(loaded.workers.first().codexSessionId, QString("019e3782-3d83-7da0-83bc-ab69315c9675"));
        QCOMPARE(loaded.workers.first().codexModel, QString("gpt-5.3-codex-spark"));
        QCOMPARE(loaded.workers.first().codexDirectory, QString("~/dev/features"));
        QCOMPARE(loaded.workers.first().codexPermissions, QString("Workspace (on-request)"));
        QCOMPARE(loaded.workers.first().codexContextWindow, QString("73% left (41.7K used / 122K)"));
        QCOMPARE(loaded.workers.first().codexAccount, QString("dethislikethewind@gmail.com (Pro)"));
        QCOMPARE(loaded.projects.size(), 1);
        QCOMPARE(loaded.projects.first().projectId, QString("features"));
        QCOMPARE(loaded.projects.first().workerIds, QStringList{"organizer"});
    }

    void codexStatusImportParsesStatusOutput() {
        const QString statusText = QString::fromUtf8(R"(╭──────────────────────────────────────────────────────────────────────────╮
│  Model:                       gpt-5.3-codex-spark (reasoning xhigh, summ │
│  Directory:                   ~/dev/features                             │
│  Permissions:                 Workspace (on-request)                     │
│  Account:                     dethislikethewind@gmail.com (Pro)          │
│  Session:                     019e3782-3d83-7da0-83bc-ab69315c9675       │
│  Context window:              73% left (41.7K used / 122K)               │
╰──────────────────────────────────────────────────────────────────────────╯)");

        const DexProjects::CodexStatusImport status =
            DexProjects::parseCodexStatusText(statusText);

        QVERIFY(status.hasAnyValue());
        QCOMPARE(status.sessionId, QString("019e3782-3d83-7da0-83bc-ab69315c9675"));
        QCOMPARE(status.model, QString("gpt-5.3-codex-spark (reasoning xhigh, summ"));
        QCOMPARE(status.directory, QString("~/dev/features"));
        QCOMPARE(status.permissions, QString("Workspace (on-request)"));
        QCOMPARE(status.contextWindow, QString("73% left (41.7K used / 122K)"));
    }

    void codexStatusImportUpdatesOnlySelectedWorkerRow() {
        QVector<DexProjects::ProjectRegistrySpecWorkerRow> workers = {
            {"organizer", "organizer", "Organizer", "ready"},
            {"planner", "planner", "Planner", "ready"},
        };
        const DexProjects::CodexStatusImport status =
            DexProjects::parseCodexStatusText("Session: 019e3782-3d83-7da0-83bc-ab69315c9675\nModel: gpt-5.3-codex-spark\nDirectory: ~/dev/features\nPermissions: Workspace (on-request)");

        DexProjects::applyCodexStatusImport(&workers[1], status);

        QCOMPARE(workers.at(0).codexSessionId, QString());
        QCOMPARE(workers.at(1).codexSessionId, QString("019e3782-3d83-7da0-83bc-ab69315c9675"));
        QCOMPARE(workers.at(1).codexModel, QString("gpt-5.3-codex-spark"));
        QCOMPARE(workers.at(1).codexDirectory, QString("~/dev/features"));
        QCOMPARE(workers.at(1).codexPermissions, QString("Workspace (on-request)"));
    }

    void projectRegistrySpecDraftRoundTripsProjectFields() {
        DexProjects::ProjectRegistry registry;
        registry.loaded = true;
        DexProjects::ProjectRegistrySpecDraft draft =
            DexProjects::projectRegistrySpecDraft(registry, QString());
        draft.project.projectId = "features";
        draft.project.name = "features";
        draft.project.path = "/Users/kogaryu/dev/features";
        draft.project.role = "feature foundry";
        draft.project.status = "active";
        draft.project.authority = "sandbox";
        draft.project.projectType = "feature_library";
        draft.project.pinned = true;
        draft.project.binderTemplate = "features_feature_foundry_v1";
        draft.project.safeEditZones = {"feature_packs/**"};
        draft.project.protectedZones = {"Cargo.toml", "Cargo.lock"};
        draft.project.generatedZones = {"target/**"};
        draft.project.scratchZones = {".dex/**"};
        draft.project.sourceDocs = {"README.md"};
        draft.project.buildCommands = {"cargo build"};
        draft.project.testCommands = {"cargo test"};
        draft.project.proofCommands = {"cargo test --workspace"};
        draft.project.editorCoreStatus = "reserved";
        draft.project.editorCorePath = "dex-editor-core";
        draft.project.fileInspectSupported = false;
        draft.project.fileEditSupported = false;

        const DexProjects::ProjectRegistry next =
            DexProjects::applyProjectRegistrySpecDraft(registry, draft);

        QCOMPARE(next.projects.size(), 1);
        const DexProjects::ProjectRegistryEntry project = next.projects.first();
        QCOMPARE(project.projectId, QString("features"));
        QCOMPARE(project.safeEditZones, QStringList{"feature_packs/**"});
        QCOMPARE(project.protectedZones, QStringList({"Cargo.toml", "Cargo.lock"}));
        QCOMPARE(project.buildCommands, QStringList{"cargo build"});
        QCOMPARE(project.binderTemplate, QString("features_feature_foundry_v1"));
    }

    void blankProjectRegistrySpecDraftHasNoProjectDefaults() {
        DexProjects::ProjectRegistry registry;
        registry.loaded = true;
        const DexProjects::ProjectRegistrySpecDraft draft =
            DexProjects::projectRegistrySpecDraft(registry, QString());

        QCOMPARE(draft.project.projectId, QString());
        QCOMPARE(draft.project.name, QString());
        QCOMPARE(draft.project.path, QString());
        QCOMPARE(draft.project.status, QString());
        QCOMPARE(draft.project.authority, QString());
        QCOMPARE(draft.project.projectType, QString());
        QVERIFY(!draft.project.pinned);
        QCOMPARE(draft.workers.size(), 0);
    }

    void projectRegistrySpecSaveWithoutWorkersAddsNoWorkerRows() {
        DexProjects::ProjectRegistry registry;
        registry.loaded = true;
        DexProjects::ProjectRegistrySpecDraft draft =
            DexProjects::projectRegistrySpecDraft(registry, QString());
        draft.project.projectId = "features";
        draft.project.name = "features";

        const DexProjects::ProjectRegistry next =
            DexProjects::applyProjectRegistrySpecDraft(registry, draft);

        QCOMPARE(next.projects.size(), 1);
        QCOMPARE(next.projects.first().workerIds.size(), 0);
        QCOMPARE(next.workers.size(), 0);
    }

    void projectRegistrySpecWorkersUpdateCatalogAndProjectIds() {
        DexProjects::ProjectRegistry registry;
        registry.loaded = true;
        DexProjects::ProjectRegistrySpecDraft draft =
            DexProjects::projectRegistrySpecDraft(registry, QString());
        draft.project.projectId = "features";
        draft.project.name = "features";
        draft.workers.push_back({"curator", "feature_curator", "Feature Curator", "ready"});
        draft.workers.push_back({"builder", "feature_builder", "Feature Builder", "hold"});

        const DexProjects::ProjectRegistry next =
            DexProjects::applyProjectRegistrySpecDraft(registry, draft);

        QCOMPARE(next.projects.first().workerIds, QStringList({"curator", "builder"}));
        QCOMPARE(next.workers.size(), 2);
        QCOMPARE(next.workers.at(0).workerId, QString("curator"));
        QCOMPARE(next.workers.at(0).role, QString("feature_curator"));
        QCOMPARE(next.workers.at(1).displayName, QString("Feature Builder"));
    }

    void projectRegistrySpecWorkersPreserveCodexSessionFields() {
        DexProjects::ProjectRegistry registry;
        registry.loaded = true;
        DexProjects::ProjectRegistrySpecDraft draft =
            DexProjects::projectRegistrySpecDraft(registry, QString());
        draft.project.projectId = "features";
        draft.project.name = "features";
        DexProjects::ProjectRegistrySpecWorkerRow worker;
        worker.workerId = "planner";
        worker.role = "planner";
        worker.displayName = "Planner";
        worker.status = "ready";
        worker.codexSessionId = "019e3782-3d83-7da0-83bc-ab69315c9675";
        worker.codexModel = "gpt-5.3-codex-spark";
        worker.codexDirectory = "~/dev/features";
        worker.codexPermissions = "Workspace (on-request)";
        worker.codexContextWindow = "73% left (41.7K used / 122K)";
        worker.codexAccount = "dethislikethewind@gmail.com (Pro)";
        draft.workers.push_back(worker);

        const DexProjects::ProjectRegistry next =
            DexProjects::applyProjectRegistrySpecDraft(registry, draft);

        QCOMPARE(next.workers.size(), 1);
        QCOMPARE(next.workers.first().codexSessionId, QString("019e3782-3d83-7da0-83bc-ab69315c9675"));
        QCOMPARE(next.workers.first().codexModel, QString("gpt-5.3-codex-spark"));
        const DexProjects::ProjectRegistrySpecDraft loadedDraft =
            DexProjects::projectRegistrySpecDraft(next, "features");
        QCOMPARE(loadedDraft.workers.size(), 1);
        QCOMPARE(loadedDraft.workers.first().codexDirectory, QString("~/dev/features"));
        QCOMPARE(loadedDraft.workers.first().codexAccount, QString("dethislikethewind@gmail.com (Pro)"));
    }

    void projectRegistrySpecRemovedWorkerLeavesCatalogOnlyWhenShared() {
        DexProjects::ProjectRegistry registry;
        registry.loaded = true;
        registry.workers = {
            {"curator", "feature_curator", "Feature Curator", "ready"},
            {"shared", "shared_role", "Shared Worker", "ready"},
        };
        DexProjects::ProjectRegistryEntry features;
        features.projectId = "features";
        features.name = "features";
        features.workerIds = {"curator", "shared"};
        DexProjects::ProjectRegistryEntry other;
        other.projectId = "other";
        other.name = "other";
        other.workerIds = {"shared"};
        registry.projects = {features, other};

        DexProjects::ProjectRegistrySpecDraft draft =
            DexProjects::projectRegistrySpecDraft(registry, "features");
        draft.workers = {{"shared", "shared_role", "Shared Worker", "ready"}};

        const DexProjects::ProjectRegistry next =
            DexProjects::applyProjectRegistrySpecDraft(registry, draft);

        QCOMPARE(next.projects.first().workerIds, QStringList{"shared"});
        QCOMPARE(next.workers.size(), 1);
        QCOMPARE(next.workers.first().workerId, QString("shared"));
    }

    void projectScopedWorkersUseRegistryWorkerIds() {
        CockpitState state;
        state.projectRegistryLoaded = true;
        DexProjects::ProjectRegistryEntry features;
        features.projectId = "features";
        features.name = "features";
        features.workerIds = {"organizer", "stager"};
        state.registryProjects = {features};
        state.workers = {
            {"organizer", "organizer", "Organizer", "ready"},
            {"planner", "planner", "Planner", "ready"},
            {"stager", "stager", "Stager", "hold"},
        };

        const QVector<WorkerSummary> workers = workersForProject(state, "features");
        QCOMPARE(workers.size(), 2);
        QCOMPARE(workers.at(0).id, QString("organizer"));
        QCOMPARE(workers.at(1).id, QString("stager"));
    }

    void loadedProjectRegistryDoesNotInjectBackendWorkersWhenUnset() {
        CockpitState state;
        state.projectRegistryLoaded = true;
        DexProjects::ProjectRegistryEntry plain;
        plain.projectId = "plain";
        plain.name = "Plain Repo";
        state.registryProjects = {plain};
        state.workers = {
            {"planner", "planner", "Planner", "ready"},
            {"builder", "builder", "Builder", "busy"},
        };

        const QVector<WorkerSummary> workers = workersForProject(state, "plain");
        QCOMPARE(workers.size(), 0);
    }

    void missingProjectRegistryCanStillFallbackToBackendWorkersWhenUnset() {
        CockpitState state;
        state.projectRegistryLoaded = false;
        ProjectSummary plain;
        plain.id = "plain";
        plain.name = "Plain Repo";
        state.projects = {plain};
        state.workers = {
            {"planner", "planner", "Planner", "ready"},
            {"builder", "builder", "Builder", "busy"},
        };

        const QVector<WorkerSummary> workers = workersForProject(state, "plain");
        QCOMPARE(workers.size(), 2);
        QCOMPARE(workers.at(0).id, QString("planner"));
        QCOMPARE(workers.at(1).id, QString("builder"));
    }

    void registryWorkerIdsWithoutCatalogStillRenderStatefulPlaceholders() {
        CockpitState state;
        state.projectRegistryLoaded = true;
        DexProjects::ProjectRegistryEntry plain;
        plain.projectId = "plain";
        plain.name = "Plain Repo";
        plain.workerIds = {"organizer"};
        state.registryProjects = {plain};
        state.workers = {
            {"planner", "planner", "Planner", "ready"},
        };

        const QVector<WorkerSummary> workers = workersForProject(state, "plain");
        QCOMPARE(workers.size(), 1);
        QCOMPARE(workers.at(0).id, QString("organizer"));
        QCOMPARE(workers.at(0).role, QString("organizer"));
        QCOMPARE(workers.at(0).status, QString("unknown"));
    }

    void emptyProjectRegistryParsesAsLoadedBlankSlate() {
        const QJsonDocument registryDocument(QJsonObject{{"projects", QJsonArray{}}});
        const DexProjects::ProjectRegistry registry = DexProjects::parseProjectRegistryDocument(registryDocument);

        QVERIFY(registry.loaded);
        QCOMPARE(registry.projects.size(), 0);
    }

    void loadedEmptyProjectRegistryDoesNotFallbackToRustProjects() {
        CockpitState state;
        ProjectSummary rustProject;
        rustProject.id = "dex_home";
        rustProject.name = "dex_home";
        state.projects.push_back(rustProject);
        state.projectRegistryLoaded = true;
        state.registryProjects.clear();

        const QVector<DexProjects::ProjectRegistryEntry> projects = registryProjectsForState(state);
        QCOMPARE(projects.size(), 0);
    }

    void missingProjectRegistryStillFallsBackToRustProjects() {
        CockpitState state;
        ProjectSummary rustProject;
        rustProject.id = "dex_home";
        rustProject.name = "dex_home";
        state.projects.push_back(rustProject);
        state.projectRegistryLoaded = false;

        const QVector<DexProjects::ProjectRegistryEntry> projects = registryProjectsForState(state);
        QCOMPARE(projects.size(), 1);
        QCOMPARE(projects.first().projectId, QString("dex_home"));
    }

    void parsesBinderStateRecordsAndIgnoresUnknownFields() {
        QJsonObject root;
        root["selected_worker_id"] = "builder";
        root["unknown_future_field"] = "ignored";
        root["stats_snapshot"] = QJsonObject{
            {"worker_id", "builder"},
            {"worker_role", "builder"},
            {"generated_from", QJsonObject{{"grade_records", 1}, {"transcript_records", 1}, {"evidence_records", 1}}},
            {"routing_summary", QJsonObject{{"current_trust", "conditional"}, {"best_scope", "bounded_execution"}, {"weak_scope", "visual_teardown"}, {"sample_size", 1}, {"confidence", "medium"}}},
            {"outcome_by_task_shape", QJsonArray{QJsonObject{{"task_shape", "bounded_ui_primitive"}, {"runs", 1}, {"pass", 1}, {"partial", 0}, {"fail", 0}, {"recommended_route", QJsonArray{"builder"}}, {"confidence", "low"}}}},
            {"model_fit", QJsonArray{QJsonObject{{"model_id", "fast-code"}, {"best_task_shapes", QJsonArray{"bounded_ui_primitive"}}, {"weak_task_shapes", QJsonArray{"visual_teardown"}}}}},
            {"input_quality_impact", QJsonArray{QJsonObject{{"input_condition", "complete_packet"}, {"runs", 1}, {"observed_outcome", "high_pass_rate"}}}},
            {"evidence_health", QJsonObject{{"graded_runs", 1}, {"runs_with_proof_refs", 1}, {"data_confidence", "medium"}}},
            {"routing_recommendation", QJsonObject{{"default_route", "builder_direct"}, {"require_planner_when", QJsonArray{"ambiguity"}}, {"require_reviewer_when", QJsonArray{"proof_gap"}}, {"block_builder_when", QJsonArray{"no_stop_condition"}}}},
        };
        root["grade_records"] = QJsonArray{QJsonObject{
            {"record_id", "grade-1"},
            {"timestamp", "2026-05-15T23:45:00-06:00"},
            {"worker_id", "builder"},
            {"worker_role", "builder"},
            {"model_profile", QJsonObject{{"model_id", "fast-code"}}},
            {"task_scope", QJsonObject{{"task_shape", "bounded_ui_primitive"}}},
            {"input_quality", QJsonObject{{"input_verdict", "complete_packet"}}},
            {"adjusted_verdict", QJsonObject{{"worker_fault", "low"}}},
            {"scores", QJsonObject{{"scope_control", "B"}}},
            {"evidence", QJsonObject{{"proof_refs", QJsonArray{"proof.png"}}}},
            {"routing_decision", QJsonObject{{"routing_verdict", "builder_direct"}}},
            {"learned_rules", QJsonArray{QJsonObject{{"rule_id", "bounded_builder"}}}},
            {"final_grade", "B"},
            {"outcome", "pass"},
            {"grade_confidence", "medium"},
        }};
        root["transcript_records"] = QJsonArray{QJsonObject{
            {"record_id", "transcript-1"},
            {"session_id", "session-1"},
            {"worker_id", "builder"},
            {"worker_role", "builder"},
            {"model_id", "fast-code"},
            {"source_type", "chat_thread"},
            {"record_status", "partial"},
            {"time_metadata", QJsonObject{{"local_timestamp", "2026-05-15T23:43:40-06:00"}, {"timezone", "America/Regina"}}},
            {"environment_metadata", QJsonObject{{"tool_access_state", "partial"}}},
            {"segments", QJsonArray{QJsonObject{{"segment_id", "seg_001"}, {"raw_excerpt", "raw evidence"}}}},
            {"tool_receipts", QJsonArray{QJsonObject{{"receipt_id", "tool_001"}, {"result", "passed"}}}},
            {"artifact_refs", QJsonObject{{"screenshots", QJsonArray{"after.png"}}}},
            {"missing_data", QJsonArray{"exact_model_id"}},
            {"collection_warnings", QJsonArray{"visual_evidence_partial"}},
        }};
        root["evidence_records"] = QJsonArray{QJsonObject{
            {"record_id", "evidence-1"},
            {"session_id", "session-1"},
            {"worker_id", "builder"},
            {"evidence_id", "ev_001"},
            {"proof_refs", QJsonArray{QJsonObject{{"type", "screenshot"}, {"path", "after.png"}, {"status", "attached"}}}},
            {"failure", QJsonObject{{"failure_id", "fail_001"}, {"type", "scope_drift"}, {"severity", "high"}}},
            {"correction", QJsonObject{{"correction_id", "corr_001"}, {"status", "accepted"}}},
            {"acceptance", QJsonObject{{"accepted", true}}},
        }};
        root["profiles"] = QJsonArray{QJsonObject{{"profile_id", "p1"}, {"name", "Profile One"}, {"roles", QJsonArray{QJsonObject{{"role_id", "builder"}, {"label", "Builder"}, {"role_kind", "builder"}}}}}};
        root["selected_profile_id"] = "p1";
        root["launch_plan_preview"] = QJsonObject{{"count", 1}};

        const DexBinder::BinderState binder = DexBinder::parseBinderState(root);
        QCOMPARE(binder.statsSnapshot.sampleSize, 1);
        QCOMPARE(binder.statsSnapshot.confidence, QString("medium"));
        QCOMPARE(binder.gradeRecords.size(), 1);
        QCOMPARE(binder.gradeRecords.first().routingVerdict, QString("builder_direct"));
        QCOMPARE(binder.transcriptRecords.size(), 1);
        QCOMPARE(binder.transcriptRecords.first().missingData.size(), 1);
        QCOMPARE(binder.evidenceRecords.size(), 1);
        QCOMPARE(binder.evidenceRecords.first().acceptedState, QString("true"));
        QCOMPARE(binder.profileState.name, QString("Profile One"));
    }

    void parserHandlesMissingStatsAndEmptyArrays() {
        QJsonObject root;
        root["selected_worker_id"] = "builder";
        root["grade_records"] = QJsonArray{};
        root["transcript_records"] = QJsonArray{};
        root["evidence_records"] = QJsonArray{};

        const DexBinder::BinderState binder = DexBinder::parseBinderState(root);
        QCOMPARE(binder.statsSnapshot.confidence, QString("no_data"));
        QCOMPARE(binder.statsSnapshot.sampleSize, 0);
        QCOMPARE(binder.gradeRecords.size(), 0);
        QCOMPARE(binder.transcriptRecords.size(), 0);
        QCOMPARE(binder.evidenceRecords.size(), 0);
        QVERIFY(!binder.profileState.present);
    }
};

QTEST_MAIN(StateBackendSmoke)
#include "state_backend_smoke.moc"

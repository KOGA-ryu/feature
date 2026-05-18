#pragma once

#include <QMap>
#include <QString>
#include <QStringList>
#include <QVector>

class QJsonDocument;
class QJsonObject;

namespace DexRepoBinderTemplate {

inline constexpr const char *kDefaultTemplateId = "repo_default_binder_v1";

struct BinderTemplateRow {
    QStringList cells;
    bool risk = false;
};

struct BinderTemplateSection {
    QString tab;
    QStringList lenses;
    QString title;
    QString purpose;
    QStringList lines;
    QVector<BinderTemplateRow> rows;
    QStringList badges;
    QString emptyState;
    bool subtle = false;
};

struct BinderTemplate {
    QString templateId = QString::fromUtf8(kDefaultTemplateId);
    QString displayName;
    QString sourcePath;
    QString error;
    bool loaded = false;
    QStringList topTabs;
    QMap<QString, QStringList> detailLenses;
    QVector<BinderTemplateSection> sections;
    QStringList contextLines;
};

struct BinderTemplateStore {
    QVector<BinderTemplate> templates;
    QString sourceDir;
    bool loaded = false;
    QStringList warnings;
};

BinderTemplate parseBinderTemplateDocument(const QJsonDocument &document, const QString &sourcePath = QString());
BinderTemplate loadBinderTemplateFile(const QString &path);
bool saveBinderTemplateFile(const BinderTemplate &templateRecord, const QString &path, QString *error = nullptr);
BinderTemplateStore loadBinderTemplateStore(const QString &templateDir);
BinderTemplate defaultBinderTemplate();
const BinderTemplate *findTemplateById(const BinderTemplateStore &store, const QString &templateId);
BinderTemplate resolveTemplateForProject(const BinderTemplateStore &store, const QString &templateId);

} // namespace DexRepoBinderTemplate

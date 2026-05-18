#include "repo_binder_template.h"

#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QJsonDocument>
#include <QJsonParseError>

namespace DexRepoBinderTemplate {

BinderTemplate loadBinderTemplateFile(const QString &path) {
    BinderTemplate templateRecord;
    templateRecord.sourcePath = path;
    QFile file(path);
    if (!file.exists()) {
        templateRecord.error = "binder template not found: " + path;
        return templateRecord;
    }
    if (!file.open(QIODevice::ReadOnly)) {
        templateRecord.error = "binder template could not be opened: " + path;
        return templateRecord;
    }

    QJsonParseError parseError;
    const QJsonDocument document = QJsonDocument::fromJson(file.readAll(), &parseError);
    if (parseError.error != QJsonParseError::NoError) {
        templateRecord.error = "binder template JSON error: " + parseError.errorString();
        return templateRecord;
    }
    return parseBinderTemplateDocument(document, QFileInfo(path).absoluteFilePath());
}

BinderTemplateStore loadBinderTemplateStore(const QString &templateDir) {
    BinderTemplateStore store;
    store.sourceDir = templateDir;
    QDir dir(templateDir);
    if (!dir.exists()) {
        store.warnings.push_back("binder template directory not found: " + templateDir);
        store.templates.push_back(defaultBinderTemplate());
        store.loaded = false;
        return store;
    }

    const QStringList files = dir.entryList({"*.json"}, QDir::Files, QDir::Name);
    for (const QString &fileName : files) {
        BinderTemplate templateRecord = loadBinderTemplateFile(dir.filePath(fileName));
        if (templateRecord.loaded && !templateRecord.templateId.isEmpty()) {
            store.templates.push_back(templateRecord);
        } else {
            store.warnings.push_back(templateRecord.error);
        }
    }

    if (!findTemplateById(store, QString::fromUtf8(kDefaultTemplateId))) {
        store.templates.push_back(defaultBinderTemplate());
    }
    store.loaded = !store.templates.isEmpty();
    return store;
}

const BinderTemplate *findTemplateById(const BinderTemplateStore &store, const QString &templateId) {
    for (const BinderTemplate &templateRecord : store.templates) {
        if (templateRecord.templateId == templateId) {
            return &templateRecord;
        }
    }
    return nullptr;
}

BinderTemplate resolveTemplateForProject(const BinderTemplateStore &store, const QString &templateId) {
    const QString requested = templateId.isEmpty() ? QString::fromUtf8(kDefaultTemplateId) : templateId;
    if (const BinderTemplate *templateRecord = findTemplateById(store, requested)) {
        return *templateRecord;
    }
    if (const BinderTemplate *defaultRecord = findTemplateById(store, QString::fromUtf8(kDefaultTemplateId))) {
        return *defaultRecord;
    }
    return defaultBinderTemplate();
}

} // namespace DexRepoBinderTemplate

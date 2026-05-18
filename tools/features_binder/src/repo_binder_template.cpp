#include "repo_binder_template.h"

#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QJsonValue>
#include <QSaveFile>

namespace DexRepoBinderTemplate {
namespace {

QString readString(const QJsonObject &object, const QString &key, const QString &fallback = QString()) {
    const QJsonValue value = object.value(key);
    return value.isString() ? value.toString() : fallback;
}

bool readBool(const QJsonObject &object, const QString &key, bool fallback = false) {
    const QJsonValue value = object.value(key);
    return value.isBool() ? value.toBool() : fallback;
}

QStringList readStringList(const QJsonObject &object, const QString &key) {
    QStringList result;
    const QJsonValue value = object.value(key);
    if (!value.isArray()) {
        return result;
    }
    for (const QJsonValue &item : value.toArray()) {
        if (item.isString()) {
            result.push_back(item.toString());
        }
    }
    return result;
}

BinderTemplateRow parseRow(const QJsonValue &value) {
    BinderTemplateRow row;
    if (value.isArray()) {
        for (const QJsonValue &cell : value.toArray()) {
            if (cell.isString()) {
                row.cells.push_back(cell.toString());
            }
        }
        return row;
    }
    if (!value.isObject()) {
        return row;
    }

    const QJsonObject object = value.toObject();
    row.cells = readStringList(object, "cells");
    row.risk = readBool(object, "risk");
    return row;
}

BinderTemplateSection parseSection(const QJsonObject &object) {
    BinderTemplateSection section;
    section.tab = readString(object, "tab");
    section.lenses = readStringList(object, "lenses");
    section.title = readString(object, "title");
    section.purpose = readString(object, "purpose");
    section.lines = readStringList(object, "lines");
    section.badges = readStringList(object, "badges");
    section.emptyState = readString(object, "empty_state");
    section.subtle = readBool(object, "subtle");

    const QJsonArray rows = object.value("rows").toArray();
    for (const QJsonValue &rowValue : rows) {
        BinderTemplateRow row = parseRow(rowValue);
        if (!row.cells.isEmpty()) {
            section.rows.push_back(row);
        }
    }
    return section;
}

} // namespace

BinderTemplate parseBinderTemplateDocument(const QJsonDocument &document, const QString &sourcePath) {
    BinderTemplate templateRecord;
    templateRecord.sourcePath = sourcePath;
    if (!document.isObject()) {
        templateRecord.error = "binder template root must be an object";
        return templateRecord;
    }

    const QJsonObject root = document.object();
    templateRecord.templateId = readString(root, "template_id", QString::fromUtf8(kDefaultTemplateId));
    templateRecord.displayName = readString(root, "display_name");
    templateRecord.topTabs = readStringList(root, "top_tabs");
    templateRecord.contextLines = readStringList(root, "context_lines");

    const QJsonObject detailLensObject = root.value("detail_lenses").toObject();
    for (auto it = detailLensObject.begin(); it != detailLensObject.end(); ++it) {
        if (!it.value().isArray()) {
            continue;
        }
        QStringList lenses;
        for (const QJsonValue &value : it.value().toArray()) {
            if (value.isString()) {
                lenses.push_back(value.toString());
            }
        }
        templateRecord.detailLenses.insert(it.key(), lenses);
    }

    const QJsonArray sections = root.value("sections").toArray();
    for (const QJsonValue &value : sections) {
        if (!value.isObject()) {
            continue;
        }
        BinderTemplateSection section = parseSection(value.toObject());
        if (!section.tab.isEmpty() && !section.title.isEmpty()) {
            templateRecord.sections.push_back(section);
        }
    }

    templateRecord.loaded = templateRecord.error.isEmpty();
    return templateRecord;
}

namespace {

QJsonArray stringListToJson(const QStringList &values) {
    QJsonArray array;
    for (const QString &value : values) {
        array.push_back(value);
    }
    return array;
}

QJsonArray rowsToJson(const QVector<BinderTemplateRow> &rows) {
    QJsonArray array;
    for (const BinderTemplateRow &row : rows) {
        if (row.risk) {
            array.push_back(QJsonObject{
                {"risk", true},
                {"cells", stringListToJson(row.cells)},
            });
        } else {
            array.push_back(stringListToJson(row.cells));
        }
    }
    return array;
}

} // namespace

bool saveBinderTemplateFile(const BinderTemplate &templateRecord, const QString &path, QString *error) {
    if (path.isEmpty()) {
        if (error) {
            *error = "binder template path is empty";
        }
        return false;
    }

    QJsonObject detailLenses;
    for (auto it = templateRecord.detailLenses.constBegin(); it != templateRecord.detailLenses.constEnd(); ++it) {
        detailLenses[it.key()] = stringListToJson(it.value());
    }

    QJsonArray sections;
    for (const BinderTemplateSection &section : templateRecord.sections) {
        QJsonObject object;
        object["tab"] = section.tab;
        object["lenses"] = stringListToJson(section.lenses);
        object["title"] = section.title;
        object["subtle"] = section.subtle;
        if (!section.purpose.isEmpty()) {
            object["purpose"] = section.purpose;
        }
        if (!section.lines.isEmpty()) {
            object["lines"] = stringListToJson(section.lines);
        }
        if (!section.badges.isEmpty()) {
            object["badges"] = stringListToJson(section.badges);
        }
        if (!section.emptyState.isEmpty()) {
            object["empty_state"] = section.emptyState;
        }
        object["rows"] = rowsToJson(section.rows);
        sections.push_back(object);
    }

    QJsonObject root;
    root["template_id"] = templateRecord.templateId;
    root["record_version"] = 1;
    root["display_name"] = templateRecord.displayName;
    root["top_tabs"] = stringListToJson(templateRecord.topTabs);
    root["detail_lenses"] = detailLenses;
    root["context_lines"] = stringListToJson(templateRecord.contextLines);
    root["sections"] = sections;

    QSaveFile file(path);
    if (!file.open(QIODevice::WriteOnly)) {
        if (error) {
            *error = "binder template could not be opened for writing: " + path;
        }
        return false;
    }
    file.write(QJsonDocument(root).toJson(QJsonDocument::Indented));
    if (!file.commit()) {
        if (error) {
            *error = "binder template write failed: " + path;
        }
        return false;
    }
    return true;
}

} // namespace DexRepoBinderTemplate

#pragma once

#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QJsonValue>
#include <QString>
#include <QStringList>

#include <algorithm>

namespace DexBinder {

inline QString readString(const QJsonObject &object, const QString &key, const QString &fallback = QString()) {
    const QJsonValue value = object.value(key);
    return value.isString() ? value.toString() : fallback;
}

inline int readInt(const QJsonObject &object, const QString &key, int fallback = 0) {
    const QJsonValue value = object.value(key);
    return value.isDouble() ? value.toInt() : fallback;
}

inline bool readBool(const QJsonObject &object, const QString &key, bool fallback = false) {
    const QJsonValue value = object.value(key);
    return value.isBool() ? value.toBool() : fallback;
}

inline QStringList readStringList(const QJsonObject &object, const QString &key) {
    QStringList result;
    const QJsonArray values = object.value(key).toArray();
    for (const QJsonValue &value : values) {
        if (value.isString()) {
            result.push_back(value.toString());
        }
    }
    return result;
}

inline QString scalarText(const QJsonValue &value) {
    if (value.isString()) {
        return value.toString();
    }
    if (value.isBool()) {
        return value.toBool() ? "true" : "false";
    }
    if (value.isDouble()) {
        return QString::number(value.toDouble(), 'g', 8);
    }
    if (value.isNull() || value.isUndefined()) {
        return "missing";
    }
    return "complex";
}

inline QStringList objectLines(const QJsonObject &object, const QString &prefix = QString()) {
    QStringList lines;
    QStringList keys = object.keys();
    std::sort(keys.begin(), keys.end());
    for (const QString &key : keys) {
        const QJsonValue value = object.value(key);
        const QString label = prefix.isEmpty() ? key : prefix + "." + key;
        if (value.isObject()) {
            lines.append(objectLines(value.toObject(), label));
        } else if (value.isArray()) {
            QStringList items;
            for (const QJsonValue &item : value.toArray()) {
                if (item.isObject()) {
                    items.append(QString::fromUtf8(QJsonDocument(item.toObject()).toJson(QJsonDocument::Compact)));
                } else {
                    items.append(scalarText(item));
                }
            }
            lines.push_back(label + ": " + (items.isEmpty() ? "[]" : items.join(", ")));
        } else {
            lines.push_back(label + ": " + scalarText(value));
        }
    }
    return lines;
}

inline QStringList arrayObjectLines(const QJsonArray &array, const QString &prefix) {
    QStringList lines;
    for (int index = 0; index < array.size(); ++index) {
        const QJsonValue value = array.at(index);
        if (value.isObject()) {
            lines.append(prefix + QString(" %1: ").arg(index + 1) + QString::fromUtf8(QJsonDocument(value.toObject()).toJson(QJsonDocument::Compact)));
        } else {
            lines.append(prefix + QString(" %1: ").arg(index + 1) + scalarText(value));
        }
    }
    return lines;
}

inline QString firstText(const QJsonObject &object, const QStringList &keys, const QString &fallback = QString()) {
    for (const QString &key : keys) {
        const QString value = readString(object, key);
        if (!value.isEmpty()) {
            return value;
        }
    }
    return fallback;
}

} // namespace DexBinder

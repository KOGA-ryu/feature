#pragma once

#include <QString>
#include <QStringList>
#include <QVector>

#include "binder_json_helpers.h"

namespace DexBinder {

struct ProfileRoleView {
    QString roleId;
    QString label;
    QString roleKind;
    QString colorHex;
    QString terminalProfileHint;
    QStringList docsToRead;
    QStringList permissions;
    QStringList nonGoals;
};

struct ProfileView {
    bool present = false;
    QString selectedProfileId;
    QString name;
    QString description;
    int launchCount = 0;
    QVector<ProfileRoleView> roles;
    QStringList rules;
    QStringList missingFields;
};

struct RelationshipPlaceholderView {
    QString status = "Relationship records are not typed yet.";
    QStringList futureSections = {
        "inbound handoffs",
        "outbound handoffs",
        "compatibility matrix",
        "handoff failure patterns",
        "next handoff rules",
    };
};

inline ProfileView parseProfileView(const QJsonObject &root) {
    ProfileView profile;
    profile.selectedProfileId = readString(root, "selected_profile_id");
    const QJsonArray profiles = root.value("profiles").toArray();
    if (profiles.isEmpty()) {
        profile.missingFields.push_back("profiles");
        return profile;
    }

    QJsonObject selected = profiles.first().toObject();
    for (const QJsonValue &value : profiles) {
        const QJsonObject object = value.toObject();
        if (readString(object, "profile_id") == profile.selectedProfileId) {
            selected = object;
            break;
        }
    }

    profile.present = true;
    profile.selectedProfileId = readString(selected, "profile_id", profile.selectedProfileId);
    profile.name = readString(selected, "name");
    profile.description = readString(selected, "description");
    profile.rules = readStringList(selected, "rules");
    const QJsonArray roles = selected.value("roles").toArray();
    for (const QJsonValue &value : roles) {
        const QJsonObject object = value.toObject();
        ProfileRoleView role;
        role.roleId = readString(object, "role_id");
        role.label = readString(object, "label");
        role.roleKind = readString(object, "role_kind");
        role.colorHex = readString(object, "color_hex");
        role.terminalProfileHint = readString(object, "terminal_profile_hint");
        role.docsToRead = readStringList(object, "docs_to_read");
        role.permissions = readStringList(object, "permissions");
        role.nonGoals = readStringList(object, "non_goals");
        profile.roles.push_back(role);
    }

    const QJsonObject launch = root.value("launch_plan_preview").toObject();
    profile.launchCount = readInt(launch, "count", launch.value("instances").toArray().size());
    if (profile.name.isEmpty()) {
        profile.missingFields.push_back("profile.name");
    }
    if (profile.roles.isEmpty()) {
        profile.missingFields.push_back("profile.roles");
    }
    return profile;
}

} // namespace DexBinder

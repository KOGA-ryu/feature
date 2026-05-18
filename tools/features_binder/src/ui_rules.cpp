#include "ui_rules.h"

#include <QFontDatabase>

namespace dex_ui {

QString colors::bg_root() { return "#f4f6f8"; }
QString colors::bg_rail_light() { return "#f8fafb"; }
QString colors::bg_rail_mid() { return "#e6ebf0"; }
QString colors::bg_rail_dark() { return "#8c949c"; }
QString colors::bg_center() { return "#ededee"; }
QString colors::bg_context() { return "#f7f8fa"; }
QString colors::bg_section() { return "#ffffff"; }
QString colors::border_soft() { return "#c1c8ce"; }
QString colors::text_primary() { return "#202329"; }
QString colors::text_muted() { return "#4d555d"; }
QString colors::selected_bg() { return "#ffffff"; }
QString colors::hover_bg() { return "#eef2f5"; }
QString colors::risk_normal() { return "#dfe7df"; }
QString colors::risk_bulky() { return "#e8e1d3"; }
QString colors::risk_split_candidate() { return "#ead8d8"; }
QString colors::risk_generated() { return "#dce3ea"; }
QString colors::risk_inspect_first() { return "#e8dfcf"; }

QString app_font_family() {
    const QStringList installedFamilies = QFontDatabase::families();
    const QStringList preferredFamilies = {
        "Helvetica Neue",
        "Avenir Next",
        "Arial",
        "Helvetica",
    };

    for (const QString &family : preferredFamilies) {
        if (installedFamilies.contains(family)) {
            return family;
        }
    }

    const QString systemFamily = QFontDatabase::systemFont(QFontDatabase::GeneralFont).family();
    if (!systemFamily.isEmpty() && systemFamily != "Sans Serif") {
        return systemFamily;
    }

    return "Helvetica";
}

QFont app_font() {
    QFont font(app_font_family());
    font.setPointSize(metrics::font_size_body);
    return font;
}

QString risk_kind_token(const QString &kind) {
    QString token = kind.toLower().trimmed();
    token.replace(' ', '_');
    token.replace('-', '_');
    return token;
}

} // namespace dex_ui

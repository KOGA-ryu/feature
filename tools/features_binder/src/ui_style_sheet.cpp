#include "ui_rules.h"

#include "ui_style_sheet_sections.h"

namespace dex_ui {

QString app_qss() {
    const QString qss =
        style_sheet_sections::baseChromeQss()
        + style_sheet_sections::surfaceAndLabelQss()
        + style_sheet_sections::buttonQss()
        + style_sheet_sections::railRowQss()
        + style_sheet_sections::statsQss()
        + style_sheet_sections::tabQss()
        + style_sheet_sections::reportAndBadgeQss();

    return QString(qss)
        .arg(colors::bg_root())
        .arg(colors::text_primary())
        .arg(metrics::font_size_body)
        .arg(metrics::divider_width)
        .arg(metrics::border_radius)
        .arg(colors::bg_rail_mid())
        .arg(colors::bg_rail_dark())
        .arg(colors::bg_center())
        .arg(colors::bg_context())
        .arg(colors::bg_rail_light())
        .arg(colors::text_muted())
        .arg(metrics::font_size_small)
        .arg(colors::hover_bg())
        .arg(colors::border_soft())
        .arg(metrics::tab_height)
        .arg(colors::selected_bg())
        .arg(metrics::font_size_title)
        .arg(colors::risk_normal())
        .arg(colors::risk_bulky())
        .arg(colors::risk_split_candidate())
        .arg(colors::risk_generated())
        .arg(colors::risk_inspect_first())
        .arg(app_font_family());
}

} // namespace dex_ui

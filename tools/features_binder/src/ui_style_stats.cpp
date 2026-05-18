#include "ui_style_sheet_sections.h"

namespace dex_ui::style_sheet_sections {

QString statsQss() {
    return QString(R"(        QFrame#statsSection, QFrame#statsSubtleSection {
            background: #ffffff;
            border: 1px solid #bdc2c7;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: 8px;
            border-bottom-right-radius: 8px;
        }
        QFrame#statsSubtleSection {
            background: #f7f8f9;
        }
        QFrame#statsRow {
            background: #ffffff;
            border: 1px solid #d2d7dc;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: 3px;
            border-bottom-right-radius: 3px;
        }
        QFrame#statsRiskRow {
            background: #ffffff;
            border: 1px solid #d9a0a0;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: 3px;
            border-bottom-right-radius: 3px;
        }
        QFrame#statsMiniCell {
            background: #ffffff;
            border: 1px solid #d2d7dc;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: 3px;
            border-bottom-right-radius: 3px;
        }
        QLabel#statsTinyLabel {
            color: #111315;
            font-size: 8px;
            background: transparent;
        }
        QLabel#statsRiskText {
            color: #6e1f1f;
            font-size: 8px;
            background: transparent;
        }
        QLabel#statsGoodText {
            color: #1f3b24;
            font-size: 8px;
            background: transparent;
        }
        QLabel#statsSectionTitle {
            color: #111315;
            font-size: 12px;
            font-weight: 800;
            background: transparent;
        }
    )");
}

} // namespace dex_ui::style_sheet_sections

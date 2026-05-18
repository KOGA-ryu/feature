#include "ui_style_sheet_sections.h"

namespace dex_ui::style_sheet_sections {

QString surfaceAndLabelQss() {
    return QString(R"(        #projectRail {
            background: %6;
            border-right: 2px solid %7;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: %5px;
            border-bottom-right-radius: %5px;
        }
        #centerLedger {
            background: %8;
            border: 0;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: %5px;
            border-bottom-right-radius: %5px;
        }
        #rightContext {
            background: %9;
            border-left: 1px solid #c2c7cc;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: %5px;
            border-bottom-right-radius: %5px;
        }
        QScrollArea {
            background: transparent;
            border: 0;
        }
        QLabel#sectionLabel {
            color: %2;
            font-size: 12px;
            font-weight: 700;
            background: %10;
            padding-left: 0px;
        }
        QLabel#mutedLabel {
            color: %11;
            font-size: 12px;
            background: transparent;
        }
        QLabel#smallLabel {
            color: #30363c;
            font-size: %12px;
            background: transparent;
        }
    )");
}

} // namespace dex_ui::style_sheet_sections

#include "ui_style_sheet_sections.h"

namespace dex_ui::style_sheet_sections {

QString buttonQss() {
    return QString(R"(        QPushButton {
            background: transparent;
            border: 1px solid transparent;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: %5px;
            border-bottom-right-radius: %5px;
            padding: 3px 7px;
            text-align: left;
        }
        QPushButton:hover {
            background: %13;
            border-color: #aeb5bc;
        }
        QPushButton#rowAction {
            background: #fbfcfd;
            border: 1px solid #8d969f;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: %5px;
            border-bottom-right-radius: %5px;
            min-width: 24px;
            max-width: 24px;
            min-height: 20px;
            max-height: 20px;
            padding: 0;
            text-align: center;
            font-weight: 700;
        }
        QPushButton#primaryAction {
            background: #1f2328;
            color: #ffffff;
            border: 1px solid #1f2328;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: %5px;
            border-bottom-right-radius: %5px;
            min-height: 24px;
            padding: 2px 8px;
            font-size: 12px;
            font-weight: 700;
            text-align: center;
        }
        QPushButton#primaryAction:hover {
            background: #343941;
        }
        QPushButton#statsContextAction {
            background: #ffffff;
            border: 1px solid #bdc2c7;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: 4px;
            border-bottom-right-radius: 4px;
            min-height: 18px;
            max-height: 18px;
            padding: 1px 8px;
            font-size: 8px;
            font-weight: 600;
            text-align: left;
        }
    )");
}

} // namespace dex_ui::style_sheet_sections

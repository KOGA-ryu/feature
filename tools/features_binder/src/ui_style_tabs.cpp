#include "ui_style_sheet_sections.h"

namespace dex_ui::style_sheet_sections {

QString tabQss() {
    return QString(R"(        QPushButton#ledgerTab {
            background: transparent;
            color: %11;
            border: 0;
            border-right: 1px solid #aeb5bc;
            border-radius: 0px;
            min-height: %15px;
            max-height: %15px;
            padding: 2px 13px 1px 13px;
            font-size: 11px;
            font-weight: 650;
        }
        QPushButton#ledgerTab:hover {
            background: rgba(255, 255, 255, 0.45);
            color: %2;
        }
        QPushButton#ledgerTab:checked {
            background: %16;
            color: %2;
            border: 1px solid #7f8790;
            border-bottom-color: %16;
            border-top-left-radius: 9px;
            border-top-right-radius: 9px;
            border-bottom-left-radius: 0px;
            border-bottom-right-radius: 0px;
            padding: 2px 14px 1px 14px;
            font-weight: 800;
        }
        QPushButton#ledgerTab[lastTab="true"] {
            border-right: 0;
        }
        QFrame#detailLensRail {
            background: #eef1f4;
            border-left: 1px solid #9da4ac;
            border-right: 1px solid #bdc2c7;
        }
        QPushButton#detailLensTab {
            background: transparent;
            color: %11;
            border: 0;
            border-bottom: 1px solid #aeb5bc;
            border-radius: 0px;
            min-height: 30px;
            max-height: 30px;
            padding: 1px 2px;
            font-size: 9px;
            font-weight: 750;
            text-align: center;
        }
        QPushButton#detailLensTab:hover {
            background: rgba(255, 255, 255, 0.50);
            color: %2;
        }
        QPushButton#detailLensTab:checked {
            background: %16;
            color: %2;
            border: 1px solid #7f8790;
            border-left-color: %16;
            border-top-left-radius: 9px;
            border-bottom-left-radius: 9px;
            border-top-right-radius: 0px;
            border-bottom-right-radius: 0px;
            font-weight: 850;
        }
        QPushButton#detailLensTab[lastTab="true"] {
            border-bottom: 0;
        }
    )");
}

} // namespace dex_ui::style_sheet_sections

#include "ui_style_sheet_sections.h"

namespace dex_ui::style_sheet_sections {

QString railRowQss() {
    return QString(R"(        QFrame#projectRow, QFrame#workerRow, QFrame#settingsRow, QFrame#candidateRow, QFrame#compactRow {
            background: %10;
            border: 1px solid %14;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: %5px;
            border-bottom-right-radius: %5px;
        }
        QFrame#projectRow[active="true"], QFrame#projectRow:hover {
            border-color: #1f2328;
        }
        QFrame#workerRow {
            background: %13;
        }
        QFrame#workerRow[selected="true"] {
            background: #f9fbfc;
            border-color: #1f2328;
        }
        QFrame#settingsRow[active="true"] {
            background: #f9fbfc;
            border-color: #1f2328;
        }
        QFrame#settingsRow:hover, QFrame#candidateRow:hover, QFrame#compactRow:hover {
            border-color: #aeb5bc;
        }
    )");
}

} // namespace dex_ui::style_sheet_sections

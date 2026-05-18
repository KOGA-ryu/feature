#include "ui_style_sheet_sections.h"

namespace dex_ui::style_sheet_sections {

QString reportAndBadgeQss() {
    return QString(R"(        QFrame#reportEntry {
            background: %16;
            border: 1px solid #1f2328;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: %5px;
            border-bottom-right-radius: %5px;
        }
        QLabel#reportTitle {
            font-size: %17px;
            font-weight: 800;
            background: transparent;
        }
        QLabel#reportGrade {
            font-size: %17px;
            font-weight: 800;
            background: transparent;
        }
        QLabel#reportBody {
            font-size: %3px;
            font-weight: 700;
            background: transparent;
        }
        QLabel#riskBadge {
            border: 1px solid #8d969f;
            border-top-left-radius: 0px;
            border-bottom-left-radius: 0px;
            border-top-right-radius: 8px;
            border-bottom-right-radius: 8px;
            padding: 1px 6px;
            font-size: %12px;
            font-weight: 800;
            background: %18;
            color: %2;
        }
        QLabel#riskBadge[kind="bulky"] {
            background: %19;
        }
        QLabel#riskBadge[kind="split_candidate"] {
            background: %20;
        }
        QLabel#riskBadge[kind="generated"], QLabel#riskBadge[kind="proof_asset"], QLabel#riskBadge[kind="safe_ignore"] {
            background: %21;
        }
        QLabel#riskBadge[kind="inspect_first"], QLabel#riskBadge[kind="authority_boundary"] {
            background: %22;
        }
    )");
}

} // namespace dex_ui::style_sheet_sections

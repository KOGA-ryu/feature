#include "ui_style_sheet_sections.h"

#include "ui_rules.h"

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
        QFrame#textEditorBlankSlot {
            background: #f8fafb;
            border: 1px solid #d4d9de;
            border-radius: 6px;
        }
        QFrame#textEditorBlankSurface {
            background: #ffffff;
            border: 1px solid #cfd5da;
            border-radius: 6px;
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

QString textEditorWorkbenchQss() {
    return QString(R"(
        QFrame#projectRail[workspace="text_editor"] {
            background: %1;
            border-right: 1px solid #3a3f45;
            border-top-right-radius: 0px;
            border-bottom-right-radius: 0px;
        }
        QFrame#projectRail[workspace="text_editor"] QScrollArea,
        QFrame#projectRail[workspace="text_editor"] QWidget#projectRailBody,
        QWidget#projectRailBody[workspace="text_editor"] {
            background: %1;
            border: 0;
        }
        QFrame#projectRail[workspace="text_editor"] QLabel {
            background: transparent;
            color: #edf2f7;
        }
        QFrame#projectRail[workspace="text_editor"] QLabel#sectionLabel {
            color: #cbd5df;
            background: transparent;
            font-size: 11px;
            font-weight: 750;
            text-transform: uppercase;
            padding-left: 0px;
        }
        QFrame#projectRail[workspace="text_editor"] QLabel#mutedLabel,
        QFrame#projectRail[workspace="text_editor"] QLabel#smallLabel {
            color: #aeb8c2;
            background: transparent;
        }
        QFrame#projectRail[workspace="text_editor"] QFrame#settingsRow {
            background: #30353a;
            border: 1px solid #3f464d;
            border-radius: %6px;
        }
        QFrame#projectRail[workspace="text_editor"] QFrame#settingsRow:hover {
            background: #38404a;
            border-color: #59636f;
        }
        QFrame#textEditorRailBucket {
            background: #30353a;
            border: 1px solid #3f464d;
            border-radius: %6px;
        }
        QFrame#textEditorRailBucket:hover {
            background: #363c42;
            border-color: #55606a;
        }
        QLabel#textEditorBucketTitle {
            color: #ffffff;
            font-size: 12px;
            font-weight: 750;
        }
        QLabel#textEditorBucketNote {
            color: #aeb8c2;
            font-size: 11px;
        }

        QWidget#textEditorBlankBody {
            background: #f5f7f9;
        }
        QWidget#textEditorWorkbenchPage,
        QScrollArea#textEditorWorkspaceScroll {
            background: #f5f7f9;
            border: 0;
        }
        QFrame#textEditorActionStrip {
            background: %3;
            border: 1px solid #dfe5eb;
            border-radius: %6px;
        }
        QFrame#textEditorCommandPalette {
            background: #ffffff;
            border: 1px solid #b9c4cf;
            border-radius: %6px;
        }
        QLineEdit#textEditorCommandPaletteInput {
            background: #fbfcfd;
            color: #151b23;
            border: 1px solid #c8d0d8;
            border-radius: 5px;
            min-height: 28px;
            padding: 2px 8px;
            font-size: 12px;
        }
        QPushButton#textEditorCommandPaletteRow {
            background: #ffffff;
            color: #1f2933;
            border: 1px solid #d8dee5;
            border-radius: %7px;
            min-height: 26px;
            max-height: 26px;
            padding: 2px 8px;
            text-align: left;
            font-size: 11px;
            font-weight: 600;
        }
        QPushButton#textEditorCommandPaletteRow[componentState="selected"] {
            background: #eaf3ff;
            border-color: #4898ff;
            color: #12395c;
        }
        QPushButton#textEditorCommandPaletteRow[componentState="disabled"] {
            background: #f5f7f9;
            border-color: #d8dee5;
            color: #7b8794;
        }
        QFrame#textEditorToolbarDivider {
            background: #d7dde3;
            border: 0;
        }
        QPushButton#textEditorActionButton {
            background: #ffffff;
            color: #1f2933;
            border: 1px solid %5;
            border-radius: %7px;
            min-height: 24px;
            max-height: 24px;
            padding: 1px 7px;
            font-size: 11px;
            font-weight: 600;
            text-align: center;
        }
        QPushButton#textEditorActionButton:disabled {
            color: #3f4a55;
            background: #ffffff;
            border-color: #cbd3db;
        }
        QPushButton#textEditorActionButton[componentState="running"] {
            color: #12395c;
            background: #e5f1ff;
            border-color: #4898ff;
        }
        QPushButton#textEditorActionButton[componentState="success"] {
            color: #0f3d24;
            background: #e7f8ee;
            border-color: #35c66b;
        }
        QPushButton#textEditorActionButton[componentState="danger"] {
            color: #651b1b;
            background: #fdecec;
            border-color: #d66a6a;
        }
        QPushButton#textEditorActionButton[componentState="disabled"] {
            color: #7b8794;
            background: #f5f7f9;
            border-color: #d8dee5;
        }
        QPushButton#textEditorActionButton[componentState="active"],
        QPushButton#textEditorActionButton:pressed {
            border-color: #4898ff;
            background: #eaf3ff;
        }
        QFrame#textEditorBlankSurface,
        QFrame#textEditorFixtureShelf {
            background: %2;
            border: 1px solid #cfd6dd;
            border-radius: %6px;
        }
        QFrame#textEditorDocumentHeader {
            background: #f8fafc;
            border-bottom: 1px solid #d8dde3;
            border-top-left-radius: %6px;
            border-top-right-radius: %6px;
        }
        QFrame#textEditorDocumentTab {
            background: #ffffff;
            border-right: 1px solid #ccd3da;
            border-bottom: 1px solid #ffffff;
            border-top-left-radius: %7px;
            border-top-right-radius: %7px;
        }
        QFrame#textEditorSelectionPreview {
            background: #f2f6f9;
            border: 1px dashed #cbd6df;
            border-radius: 4px;
        }
        QLabel#textEditorSurfaceTitle {
            color: #20262d;
            font-size: 12px;
            font-weight: 750;
            background: transparent;
        }
        QLabel#textEditorSurfaceEmpty {
            color: #4d5965;
            font-size: 12px;
            background: transparent;
        }
        QLabel#textEditorMonoLabel {
            color: #4d5965;
            font-family: "Menlo", "Monaco", "Courier New", monospace;
            font-size: 11px;
            background: transparent;
        }
        QLabel#textEditorGutterLabel {
            color: #7a838c;
            font-family: "Menlo", "Monaco", "Courier New", monospace;
            font-size: 11px;
            background: transparent;
        }
        QPlainTextEdit#textEditorDocumentEditor,
        QPlainTextEdit#textEditorOutputPreview {
            background: #ffffff;
            color: #161b22;
            border: 0;
            padding: 10px 12px;
            font-family: "Menlo", "Monaco", "Courier New", monospace;
            font-size: 12px;
            selection-background-color: #cfe2ff;
            selection-color: #111827;
        }
        QPlainTextEdit#textEditorOutputPreview {
            border: 1px solid #d7dde4;
            border-radius: 5px;
            background: #fbfcfd;
        }
        QWidget#textEditorBlankBody QLineEdit,
        QWidget#textEditorBlankBody QSpinBox,
        QWidget#textEditorBlankBody QComboBox {
            background: #ffffff;
            color: #1f2933;
            border: 1px solid #cbd3db;
            border-radius: 5px;
            min-height: 26px;
            padding: 2px 7px;
            font-size: 11px;
        }
        QWidget#textEditorBlankBody QCheckBox {
            color: #2f3b47;
            font-size: 11px;
            background: transparent;
        }
        QFrame#textEditorFixturePanel,
        QFrame#textEditorContextPanel {
            background: #ffffff;
            border: 1px solid #d8dee5;
            border-radius: %6px;
        }
        QLabel#textEditorSurfaceEmpty[componentState="success"] {
            color: #17633b;
        }
        QLabel#textEditorSurfaceEmpty[componentState="danger"] {
            color: #8a2424;
        }
        QLabel#textEditorMonoLabel[componentState="active"] {
            color: #12395c;
        }
        QFrame#textEditorFixturePanel[componentState="success"],
        QFrame#textEditorContextPanel[componentState="success"] {
            border-left: 3px solid #35c66b;
        }
        QFrame#textEditorFixturePanel[componentState="danger"],
        QFrame#textEditorContextPanel[componentState="danger"] {
            border-left: 3px solid #d64545;
        }
        QFrame#textEditorFixturePanel[componentState="disabled"],
        QFrame#textEditorContextPanel[componentState="disabled"] {
            border-left: 3px solid #97a2ad;
        }
        QFrame#textEditorFixturePanel QLabel#textEditorSurfaceTitle {
            font-size: 11px;
            font-weight: 850;
            letter-spacing: 0px;
        }
        QFrame#textEditorContextPanel QLabel#textEditorSurfaceEmpty {
            color: #47525f;
            font-size: 11px;
        }

        QFrame#detailLensRail[workspace="text_editor"] {
            background: #eef1f4;
            border-left: 1px solid #ccd2d8;
            border-right: 1px solid #ccd2d8;
        }
        QFrame#rightContext[workspace="text_editor"] {
            background: %4;
            border-left: 1px solid %5;
            border-radius: 0px;
        }
        QFrame#rightContext[workspace="text_editor"] QLineEdit,
        QFrame#rightContext[workspace="text_editor"] QSpinBox,
        QFrame#rightContext[workspace="text_editor"] QComboBox {
            background: #ffffff;
            color: #1f2933;
            border: 1px solid #cbd3db;
            border-radius: 5px;
            min-height: 26px;
            padding: 2px 7px;
            font-size: 11px;
        }
        QFrame#rightContext[workspace="text_editor"] QLineEdit:focus,
        QFrame#rightContext[workspace="text_editor"] QSpinBox:focus {
            border-color: #4898ff;
        }
        QFrame#rightContext[workspace="text_editor"] QCheckBox {
            color: #2f3b47;
            font-size: 11px;
            background: transparent;
        }
        QFrame#rightContext[workspace="text_editor"] QLabel#sectionLabel {
            color: #111827;
            background: transparent;
            font-size: 11px;
            font-weight: 800;
            text-transform: uppercase;
        }
    )")
        .arg(text_editor_colors::rail_dark())
        .arg(text_editor_colors::editor())
        .arg(text_editor_colors::toolbar())
        .arg(text_editor_colors::inspector())
        .arg(text_editor_colors::border())
        .arg(text_editor_metrics::panel_radius)
        .arg(text_editor_metrics::button_radius);
}

} // namespace dex_ui::style_sheet_sections

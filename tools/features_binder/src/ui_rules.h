#pragma once

#include <QLabel>
#include <QFont>
#include <QString>
#include <QStringList>
#include <QWidget>

namespace dex_ui {

struct metrics {
    static constexpr int design_width = 1280;
    static constexpr int design_height = 800;
    static constexpr int rail_width = 248;
    static constexpr int right_context_width = 251;
    static constexpr int border_radius = 10;
    static constexpr int sheet_overlap = 10;
    static constexpr int min_width = 900;
    static constexpr int min_height = 620;
    static constexpr int rail_inset = 10;
    static constexpr int project_row_height = 34;
    static constexpr int worker_row_height = 28;
    static constexpr int compact_row_height = 34;
    static constexpr int section_label_height = 18;
    static constexpr int section_padding = 10;
    static constexpr int page_margin = 18;
    static constexpr int panel_gap = 8;
    static constexpr int tab_height = 24;
    static constexpr int divider_width = 1;
    static constexpr int font_size_body = 13;
    static constexpr int font_size_small = 11;
    static constexpr int font_size_title = 15;
};

struct text_editor_metrics {
    static constexpr int base_grid = 8;
    static constexpr int dense_gap = 4;
    static constexpr int region_gap = 6;
    static constexpr int section_gap = 12;
    static constexpr int panel_padding = 10;
    static constexpr int panel_padding_dense = 4;
    static constexpr int panel_radius = 6;
    static constexpr int button_radius = 4;
    static constexpr int rail_width = 208;
    static constexpr int inspector_width = 300;
    static constexpr int inspector_width_min = 280;
    static constexpr int rail_bucket_height = 54;
    static constexpr int toolbar_button_height = 20;
    static constexpr int action_strip_height = 52;
    static constexpr int document_min_height = 320;
    static constexpr int fixture_shelf_height = 250;
    static constexpr int fixture_result_min_height = 144;
    static constexpr int document_tab_height = 42;
};

struct text_editor_content_limits {
    static constexpr int max_document_chars = 200000;
    static constexpr int max_clipboard_chars = 100000;
    static constexpr int max_output_preview_chars = 12000;
    static constexpr int max_receipt_preview_chars = 4000;
};

struct colors {
    static QString bg_root();
    static QString bg_rail_light();
    static QString bg_rail_mid();
    static QString bg_rail_dark();
    static QString bg_center();
    static QString bg_context();
    static QString bg_section();
    static QString border_soft();
    static QString text_primary();
    static QString text_muted();
    static QString selected_bg();
    static QString hover_bg();
    static QString risk_normal();
    static QString risk_bulky();
    static QString risk_split_candidate();
    static QString risk_generated();
    static QString risk_inspect_first();
};

struct text_editor_colors {
    static QString rail_dark();
    static QString chrome();
    static QString editor();
    static QString toolbar();
    static QString inspector();
    static QString status();
    static QString selection();
    static QString border();
    static QString focus();
    static QString success();
};

QString app_qss();
QString app_font_family();
QFont app_font();
QString risk_kind_token(const QString &kind);

QLabel *make_label(const QString &text, const char *object_name = nullptr);
QLabel *make_page_title(const QString &text);
QLabel *make_muted_label(const QString &text);
QLabel *make_badge(const QString &text, const QString &kind);

QWidget *make_section(const QString &title, const QStringList &lines);
QWidget *make_section_container(const QString &title, const QString &subtitle = QString());
QWidget *make_compact_row(const QStringList &cells);
QWidget *make_file_inventory_row(
    const QString &path,
    const QString &size_bucket,
    const QString &approximate_size,
    const QString &role,
    const QString &risk,
    const QString &recommended_action);

} // namespace dex_ui

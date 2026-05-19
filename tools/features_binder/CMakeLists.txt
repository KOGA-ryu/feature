cmake_minimum_required(VERSION 3.22)

project(DexHomeV2 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)

find_package(Qt6 REQUIRED COMPONENTS Widgets Test)

get_filename_component(DEX_HOME_FINAL_ROOT "${CMAKE_CURRENT_LIST_DIR}" ABSOLUTE)
get_filename_component(DEX_FEATURES_REPO_ROOT "${CMAKE_CURRENT_LIST_DIR}/../.." ABSOLUTE)
set(DEX_COCKPIT_CORE_BIN "${DEX_HOME_FINAL_ROOT}/rust/target/debug/dex-cockpit-core")

add_executable(dex_home_v2
    src/agent_context_renderer.cpp
    src/agent_empty_state.cpp
    src/agent_evidence_pages.cpp
    src/agent_grade_pages.cpp
    src/agent_profile_pages.cpp
    src/agent_relationship_pages.cpp
    src/agent_stats_pages.cpp
    src/agent_transcript_pages.cpp
    src/binder_record_state.cpp
    src/binder_page_helpers.cpp
    src/binder_navigation.cpp
    src/binder_stats_state.cpp
    src/binder_stats_widgets.cpp
    src/detail_lens_rail.cpp
    src/main.cpp
    src/main_window_chrome.cpp
    src/main_window.cpp
    src/main_window_ledger.cpp
    src/main_window_loaders.cpp
    src/main_window_paths.cpp
    src/main_window_registry_editor.cpp
    src/main_window_state.cpp
    src/project_rail.cpp
    src/project_rail_elided_label.cpp
    src/project_rail_project_row.cpp
    src/project_rail_worker_row.cpp
    src/project_registry_lookup.cpp
    src/project_registry_parse.cpp
    src/project_registry_spec_model.cpp
    src/project_registry_spec_page.cpp
    src/project_registry_store.cpp
    src/render_helpers.cpp
    src/repo_activity_pages.cpp
    src/repo_authority_pages.cpp
    src/repo_binder_page_helpers.cpp
    src/repo_binder_template.cpp
    src/repo_binder_template_default.cpp
    src/repo_binder_template_render.cpp
    src/repo_binder_template_store.cpp
    src/repo_contract_check_state.cpp
    src/repo_contracts_pages.cpp
    src/repo_diff_scan_state.cpp
    src/repo_context_actions.cpp
    src/repo_context_renderer.cpp
    src/repo_context_summary.cpp
    src/repo_context_tabs.cpp
    src/repo_context_template.cpp
    src/repo_cleanup_queue_rules.cpp
    src/repo_cleanup_queue_state.cpp
    src/repo_evidence_content.cpp
    src/repo_evidence_pages.cpp
    src/repo_inventory_content.cpp
    src/repo_inventory_pages.cpp
    src/repo_map_pages.cpp
    src/repo_proof_receipt_state.cpp
    src/repo_profile_pages.cpp
    src/repo_promotion_report_state.cpp
    src/repo_quality_content.cpp
    src/repo_quality_pages.cpp
    src/right_context_panel.cpp
    src/right_context_render_helpers.cpp
    src/rust_cockpit_backend.cpp
    src/rust_cockpit_backend_commands.cpp
    src/rust_cockpit_backend_fallback.cpp
    src/rust_cockpit_backend_parse.cpp
    src/rust_cockpit_backend_paths.cpp
    src/rust_cockpit_backend_process.cpp
    src/settings_shortcuts_dialog.cpp
    src/settings_shortcuts_model.cpp
    src/settings_text_actions_page.cpp
    src/sheet_stack_body.cpp
    src/text_editor_workbench_page.cpp
    src/text_editor_rust_action_client.cpp
    src/text_editor_ui.cpp
    src/text_editor_workspace_state.cpp
    src/text_editor_workspace_blank.cpp
    src/text_action_proof_model.cpp
    src/text_action_proof_panel.cpp
    src/ui_style_base_chrome.cpp
    src/ui_style_buttons.cpp
    src/ui_style_reports.cpp
    src/ui_style_rows.cpp
    src/ui_style_stats.cpp
    src/ui_style_surfaces.cpp
    src/ui_style_tabs.cpp
    src/ui_rules.cpp
    src/ui_style_sheet.cpp
    src/ui_widget_factories.cpp
)

target_link_libraries(dex_home_v2 PRIVATE Qt6::Widgets)
target_compile_definitions(dex_home_v2 PRIVATE
    DEX_FEATURES_REPO_ROOT="${DEX_FEATURES_REPO_ROOT}"
)

add_executable(dex_home_v2_smoke
    src/binder_navigation.cpp
    src/binder_record_state.cpp
    src/binder_stats_state.cpp
    src/project_registry_lookup.cpp
    src/project_registry_parse.cpp
    src/project_registry_spec_model.cpp
    src/project_registry_store.cpp
    src/repo_binder_template.cpp
    src/repo_binder_template_default.cpp
    src/repo_binder_template_store.cpp
    src/settings_shortcuts_model.cpp
    src/text_editor_rust_action_client.cpp
    src/text_editor_ui.cpp
    src/text_editor_workspace_state.cpp
    src/text_action_proof_model.cpp
    tests/state_backend_smoke.cpp
)

target_compile_definitions(dex_home_v2_smoke PRIVATE
    DEX_COCKPIT_CORE_BIN="${DEX_COCKPIT_CORE_BIN}"
    DEX_HOME_V2_SOURCE_DIR="${CMAKE_CURRENT_LIST_DIR}"
    DEX_FEATURES_REPO_ROOT="${DEX_FEATURES_REPO_ROOT}"
)

target_include_directories(dex_home_v2_smoke PRIVATE src)
target_link_libraries(dex_home_v2_smoke PRIVATE Qt6::Test Qt6::Core Qt6::Widgets)

enable_testing()
add_test(NAME dex_home_v2_smoke COMMAND dex_home_v2_smoke)

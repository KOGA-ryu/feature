#include "text_editor_ui.h"

#include <QVBoxLayout>
#include <QSizePolicy>
#include <QStyle>
#include <QWidget>

#include <algorithm>

#include "ui_rules.h"

namespace DexTextEditorUi {

namespace {

int scaled(int value, UiDensity density) {
    if (density == UiDensity::Compact) {
        return std::max(1, (value * 3) / 4);
    }
    if (density == UiDensity::Comfortable) {
        return std::max(1, (value * 5) / 4);
    }
    return value;
}

bool commandMatches(const DexTextActions::HostActionItem &action, const QString &query) {
    if (query.isEmpty()) {
        return true;
    }
    const QString normalized = query.toLower().trimmed();
    return action.actionId.toLower().contains(normalized)
        || action.label.toLower().contains(normalized)
        || action.shortLabel.toLower().contains(normalized)
        || action.category.toLower().contains(normalized)
        || action.tooltip.toLower().contains(normalized);
}

bool categoryMatches(const DexTextActions::HostActionItem &action, const QString &categoryFilter) {
    const QString normalized = normalizedCommandPaletteCategoryFilter(categoryFilter);
    return normalized == "all" || action.category == normalized;
}

TextEditorPanelDescriptor descriptor(
    const QString &key,
    const QString &persistentName,
    PanelPosition position,
    int defaultSize,
    int minSize,
    int activationPriority,
    const QString &uiPath) {
    TextEditorPanelDescriptor record;
    record.key = key;
    record.persistentName = persistentName;
    record.position = position;
    record.defaultSize = defaultSize;
    record.minSize = minSize;
    record.activationPriority = activationPriority;
    record.uiPath = uiPath;
    return record;
}

int previewLineCount(const QString &text) {
    return text.isEmpty() ? 1 : text.count('\n') + 1;
}

QString previewFacts(const QString &text) {
    return QString("%1 lines | %2 chars").arg(previewLineCount(text)).arg(text.size());
}

} // namespace

UiTokens tokens(UiDensity density) {
    UiTokens result;
    result.baseGrid = scaled(dex_ui::text_editor_metrics::base_grid, density);
    result.denseGap = scaled(dex_ui::text_editor_metrics::dense_gap, density);
    result.regionGap = scaled(dex_ui::text_editor_metrics::region_gap, density);
    result.sectionGap = scaled(dex_ui::text_editor_metrics::section_gap, density);
    result.panelPadding = scaled(dex_ui::text_editor_metrics::panel_padding, density);
    result.panelPaddingDense = scaled(dex_ui::text_editor_metrics::panel_padding_dense, density);
    result.panelRadius = dex_ui::text_editor_metrics::panel_radius;
    result.buttonRadius = dex_ui::text_editor_metrics::button_radius;
    result.railItemHeight = scaled(dex_ui::text_editor_metrics::rail_bucket_height, density);
    result.toolbarButtonHeight = scaled(dex_ui::text_editor_metrics::toolbar_button_height, density);
    result.documentTabHeight = scaled(dex_ui::text_editor_metrics::document_tab_height, density);
    return result;
}

QString panelPositionName(PanelPosition position) {
    return position == PanelPosition::Left ? QString("left") : QString("right");
}

QVector<TextEditorPanelDescriptor> textEditorPanelDescriptors() {
    return {
        descriptor(
            "documents",
            "Documents",
            PanelPosition::Left,
            dex_ui::text_editor_metrics::rail_width,
            dex_ui::text_editor_metrics::rail_width,
            10,
            "workbench.rail.text_editor.documents"),
        descriptor(
            "clipboard",
            "Clipboard",
            PanelPosition::Left,
            dex_ui::text_editor_metrics::rail_width,
            dex_ui::text_editor_metrics::rail_width,
            20,
            "workbench.rail.text_editor.clipboard"),
        descriptor(
            "drafts",
            "Drafts",
            PanelPosition::Left,
            dex_ui::text_editor_metrics::rail_width,
            dex_ui::text_editor_metrics::rail_width,
            30,
            "workbench.rail.text_editor.drafts"),
        descriptor(
            "fixtures",
            "Fixtures",
            PanelPosition::Left,
            dex_ui::text_editor_metrics::rail_width,
            dex_ui::text_editor_metrics::rail_width,
            40,
            "workbench.rail.text_editor.fixtures"),
        descriptor(
            "action_options",
            "Action Options",
            PanelPosition::Right,
            dex_ui::text_editor_metrics::inspector_width,
            dex_ui::text_editor_metrics::inspector_width_min,
            10,
            "workbench.inspector.text_editor.options"),
        descriptor(
            "workspace_state",
            "Workspace State",
            PanelPosition::Right,
            dex_ui::text_editor_metrics::inspector_width,
            dex_ui::text_editor_metrics::inspector_width_min,
            20,
            "workbench.inspector.text_editor.context"),
        descriptor(
            "last_result",
            "Last Result",
            PanelPosition::Right,
            dex_ui::text_editor_metrics::inspector_width,
            dex_ui::text_editor_metrics::inspector_width_min,
            30,
            "workbench.inspector.text_editor.result"),
        descriptor(
            "receipt",
            "Receipt",
            PanelPosition::Right,
            dex_ui::text_editor_metrics::inspector_width,
            dex_ui::text_editor_metrics::inspector_width_min,
            40,
            "workbench.inspector.text_editor.receipts"),
    };
}

QStringList textEditorPanelKeys() {
    QStringList keys;
    for (const TextEditorPanelDescriptor &descriptor : textEditorPanelDescriptors()) {
        keys.push_back(descriptor.key);
    }
    return keys;
}

QStringList textEditorPanelUiPaths() {
    QStringList uiPaths;
    for (const TextEditorPanelDescriptor &descriptor : textEditorPanelDescriptors()) {
        uiPaths.push_back(descriptor.uiPath);
    }
    return uiPaths;
}

QStringList commandPaletteCategoryFilters() {
    return {"all", "clipboard", "selection", "lines", "cleanup"};
}

QString commandPaletteCategoryFilterLabel(const QString &categoryFilter) {
    const QString normalized = normalizedCommandPaletteCategoryFilter(categoryFilter);
    if (normalized == "all") {
        return "All";
    }
    if (normalized == "clipboard") {
        return "Clipboard";
    }
    if (normalized == "selection") {
        return "Selection";
    }
    if (normalized == "lines") {
        return "Lines";
    }
    if (normalized == "cleanup") {
        return "Cleanup";
    }
    return normalized;
}

QString normalizedCommandPaletteCategoryFilter(const QString &categoryFilter) {
    const QString normalized = categoryFilter.trimmed().toLower();
    return commandPaletteCategoryFilters().contains(normalized) ? normalized : QString("all");
}

bool isCleanupActionId(const QString &actionId) {
    static const QStringList cleanupActions = {
        "text.trim_trailing_whitespace",
        "text.clean_basic",
        "text.normalize_line_endings",
        "text.strip_ansi_escape_codes",
    };
    return cleanupActions.contains(actionId);
}

QString cleanupPreviewText(const QString &beforeText, const QString &afterText) {
    return QString("BEFORE (%1)\n%2\n\nAFTER (%3)\n%4")
        .arg(previewFacts(beforeText), beforeText, previewFacts(afterText), afterText);
}

QVector<DexTextActions::HostActionItem> filterCommandPaletteActions(
    const QVector<DexTextActions::HostActionItem> &actions,
    const QString &query) {
    return filterCommandPaletteActionsForCategory(actions, query, "all");
}

QVector<DexTextActions::HostActionItem> filterCommandPaletteActionsForCategory(
    const QVector<DexTextActions::HostActionItem> &actions,
    const QString &query,
    const QString &categoryFilter) {
    QVector<DexTextActions::HostActionItem> matches;
    for (const DexTextActions::HostActionItem &action : actions) {
        if (action.placements.contains("command_palette")
            && categoryMatches(action, categoryFilter)
            && commandMatches(action, query)) {
            matches.push_back(action);
        }
    }
    return matches;
}

int firstEnabledCommandPaletteIndex(const QVector<DexTextActions::HostActionItem> &actions) {
    for (int index = 0; index < actions.size(); ++index) {
        if (actions.at(index).enabled) {
            return index;
        }
    }
    return -1;
}

int moveCommandPaletteSelection(
    const QVector<DexTextActions::HostActionItem> &actions,
    int currentIndex,
    int direction) {
    if (actions.isEmpty()) {
        return -1;
    }
    const int firstEnabled = firstEnabledCommandPaletteIndex(actions);
    if (firstEnabled < 0) {
        return -1;
    }
    if (direction == 0) {
        return currentIndex >= 0 && currentIndex < actions.size() && actions.at(currentIndex).enabled
            ? currentIndex
            : firstEnabled;
    }

    const int step = direction > 0 ? 1 : -1;
    const int start = currentIndex >= 0 && currentIndex < actions.size() ? currentIndex : firstEnabled;
    for (int offset = 1; offset <= actions.size(); ++offset) {
        const int candidate = (start + (step * offset) + actions.size()) % actions.size();
        if (actions.at(candidate).enabled) {
            return candidate;
        }
    }
    return firstEnabled;
}

QString commandPaletteSelectedActionId(
    const QVector<DexTextActions::HostActionItem> &actions,
    int selectedIndex) {
    if (selectedIndex < 0 || selectedIndex >= actions.size() || !actions.at(selectedIndex).enabled) {
        return "none";
    }
    return actions.at(selectedIndex).actionId;
}

QString commandPaletteRowText(const DexTextActions::HostActionItem &action) {
    QStringList parts;
    parts << action.label;
    parts << "[" + action.category + "]";
    if (!action.hotkeyLabel.isEmpty()) {
        parts << action.hotkeyLabel;
    }
    if (!action.enabled) {
        parts << "disabled";
    }
    return parts.join("  ");
}

QString commandPaletteAccessibleName(const DexTextActions::HostActionItem &action) {
    QStringList parts;
    parts << action.label;
    parts << action.category;
    if (!action.hotkeyLabel.isEmpty()) {
        parts << action.hotkeyLabel;
    }
    parts << (action.enabled ? QString("enabled") : QString("disabled"));
    if (!action.enabled && !action.disabledReason.isEmpty()) {
        parts << action.disabledReason;
    }
    return parts.join(", ");
}

void setUiPath(QWidget *widget, const QString &uiPath) {
    widget->setProperty("uiPath", uiPath);
}

void setComponentState(QWidget *widget, const QString &state) {
    widget->setProperty("componentState", state);
    widget->style()->unpolish(widget);
    widget->style()->polish(widget);
}

QFrame *makePanel(const QString &objectName, const QString &uiPath) {
    auto *panel = new QFrame;
    panel->setObjectName(objectName);
    panel->setMinimumWidth(0);
    setUiPath(panel, uiPath);
    return panel;
}

QLabel *makeTextEditorLabel(const QString &text, const char *objectName, const QString &uiPath) {
    auto *label = new QLabel(text);
    if (objectName) {
        label->setObjectName(objectName);
    }
    label->setMinimumWidth(0);
    label->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Preferred);
    if (!uiPath.isEmpty()) {
        setUiPath(label, uiPath);
    }
    return label;
}

QPushButton *makeActionButton(const QString &label, const QString &uiPath) {
    auto *button = new QPushButton(label);
    button->setObjectName("textEditorActionButton");
    button->setProperty("uiPath", uiPath);
    button->setProperty("componentState", "default");
    button->setMinimumWidth(0);
    button->setFixedHeight(dex_ui::text_editor_metrics::toolbar_button_height);
    button->setSizePolicy(QSizePolicy::Ignored, QSizePolicy::Fixed);
    return button;
}

QFrame *makeRailBucket(const QString &title, const QString &note, const TextEditorPanelDescriptor &descriptor) {
    auto *bucket = new QFrame;
    bucket->setObjectName("textEditorRailBucket");
    bucket->setProperty("uiPath", descriptor.uiPath);
    bucket->setProperty("panelKey", descriptor.key);
    bucket->setProperty("persistentName", descriptor.persistentName);
    bucket->setProperty("panelPosition", panelPositionName(descriptor.position));
    bucket->setProperty("defaultSize", descriptor.defaultSize);
    bucket->setProperty("minSize", descriptor.minSize);
    bucket->setProperty("startsOpen", descriptor.startsOpen);
    bucket->setProperty("enabled", descriptor.enabled);
    bucket->setProperty("activationPriority", descriptor.activationPriority);
    bucket->setMinimumHeight(dex_ui::text_editor_metrics::rail_bucket_height);

    auto *layout = new QVBoxLayout(bucket);
    layout->setContentsMargins(
        dex_ui::text_editor_metrics::panel_padding,
        dex_ui::text_editor_metrics::panel_padding_dense,
        dex_ui::text_editor_metrics::panel_padding,
        dex_ui::text_editor_metrics::panel_padding_dense);
    layout->setSpacing(dex_ui::text_editor_metrics::dense_gap);

    auto *titleLabel = new QLabel(title);
    titleLabel->setObjectName("textEditorBucketTitle");
    titleLabel->setProperty("uiPath", descriptor.uiPath + ".title");
    layout->addWidget(titleLabel);

    auto *noteLabel = new QLabel(note);
    noteLabel->setObjectName("textEditorBucketNote");
    noteLabel->setProperty("uiPath", descriptor.uiPath + ".empty_state");
    layout->addWidget(noteLabel);

    return bucket;
}

QFrame *makeDocumentTab(const QString &label, const QString &uiPath) {
    auto *tab = makePanel("textEditorDocumentTab", uiPath);
    tab->setFixedWidth(176);
    setComponentState(tab, "active");
    auto *tabLayout = new QVBoxLayout(tab);
    tabLayout->setContentsMargins(
        dex_ui::text_editor_metrics::section_gap,
        0,
        dex_ui::text_editor_metrics::panel_padding,
        0);
    tabLayout->addWidget(makeTextEditorLabel(label, "textEditorSurfaceTitle", uiPath + ".label"));
    return tab;
}

QPlainTextEdit *makeOutputBox(const QString &placeholder, const QString &uiPath) {
    auto *box = new QPlainTextEdit;
    box->setObjectName("textEditorOutputPreview");
    box->setProperty("uiPath", uiPath);
    box->setProperty("componentState", "empty");
    box->setProperty("scrollsInternally", true);
    box->setReadOnly(true);
    box->setMinimumWidth(0);
    box->setSizePolicy(QSizePolicy::Ignored, QSizePolicy::Preferred);
    box->setMinimumHeight(dex_ui::text_editor_metrics::fixture_result_min_height);
    box->setPlaceholderText(placeholder);
    return box;
}

QFrame *makeOutputPanel(const QString &title, QPlainTextEdit *output, const QString &uiPath) {
    auto *panel = makePanel("textEditorFixturePanel", uiPath);
    panel->setMinimumWidth(0);
    panel->setSizePolicy(QSizePolicy::Ignored, QSizePolicy::Preferred);
    setComponentState(panel, "empty");
    auto *layout = new QVBoxLayout(panel);
    layout->setContentsMargins(
        dex_ui::text_editor_metrics::panel_padding,
        dex_ui::text_editor_metrics::panel_padding_dense,
        dex_ui::text_editor_metrics::panel_padding,
        dex_ui::text_editor_metrics::panel_padding_dense);
    layout->setSpacing(dex_ui::text_editor_metrics::region_gap);
    layout->addWidget(makeTextEditorLabel(title, "textEditorSurfaceTitle", uiPath + ".title"));
    layout->addWidget(output, 1);
    return panel;
}

} // namespace DexTextEditorUi

#pragma once

#include <QFrame>
#include <QLabel>
#include <QPlainTextEdit>
#include <QPushButton>
#include <QString>
#include <QStringList>
#include <QVector>

#include "text_action_proof_model.h"

namespace DexTextEditorUi {

enum class UiDensity {
    Compact,
    Default,
    Comfortable,
};

enum class PanelPosition {
    Left,
    Right,
};

struct UiTokens {
    int baseGrid = 8;
    int denseGap = 4;
    int regionGap = 8;
    int sectionGap = 16;
    int panelPadding = 12;
    int panelPaddingDense = 6;
    int panelRadius = 6;
    int buttonRadius = 4;
    int railItemHeight = 54;
    int toolbarButtonHeight = 24;
    int documentTabHeight = 42;
    int fontSizeBody = 12;
    int fontSizeSmall = 11;
};

struct TextEditorPanelDescriptor {
    QString key;
    QString persistentName;
    PanelPosition position = PanelPosition::Right;
    int defaultSize = 0;
    int minSize = 0;
    bool startsOpen = true;
    bool enabled = true;
    int activationPriority = 0;
    QString uiPath;
};

UiTokens tokens(UiDensity density = UiDensity::Default);
QString panelPositionName(PanelPosition position);
QVector<TextEditorPanelDescriptor> textEditorPanelDescriptors();
QStringList textEditorPanelKeys();
QStringList textEditorPanelUiPaths();
QVector<DexTextActions::HostActionItem> filterCommandPaletteActions(
    const QVector<DexTextActions::HostActionItem> &actions,
    const QString &query);

void setUiPath(QWidget *widget, const QString &uiPath);
void setComponentState(QWidget *widget, const QString &state);
QFrame *makePanel(const QString &objectName, const QString &uiPath);
QLabel *makeTextEditorLabel(const QString &text, const char *objectName, const QString &uiPath = QString());
QPushButton *makeActionButton(const QString &label, const QString &uiPath);
QFrame *makeRailBucket(const QString &title, const QString &note, const TextEditorPanelDescriptor &descriptor);
QFrame *makeDocumentTab(const QString &label, const QString &uiPath);
QPlainTextEdit *makeOutputBox(const QString &placeholder, const QString &uiPath);
QFrame *makeOutputPanel(const QString &title, QPlainTextEdit *output, const QString &uiPath);

} // namespace DexTextEditorUi

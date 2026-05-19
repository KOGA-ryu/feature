#pragma once

class QVBoxLayout;
class QWidget;

#include <QString>

namespace DexTextEditorWorkspace {

class TextEditorWorkspaceController;

QWidget *buildBlankWorkspaceBody(TextEditorWorkspaceController *controller);
void addBlankWorkspaceContext(QVBoxLayout *layout, TextEditorWorkspaceController *controller);

} // namespace DexTextEditorWorkspace

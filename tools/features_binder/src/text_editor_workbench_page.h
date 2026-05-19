#pragma once

class QWidget;

namespace DexTextEditorWorkspace {
class TextEditorWorkspaceController;
}

namespace DexTextEditorPages {

QWidget *buildTextEditorWorkbenchPage(DexTextEditorWorkspace::TextEditorWorkspaceController *textEditorWorkspace = nullptr);

} // namespace DexTextEditorPages

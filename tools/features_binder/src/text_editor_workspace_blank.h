#pragma once

class QVBoxLayout;
class QWidget;

#include <QString>

namespace DexTextEditorWorkspace {

QWidget *buildBlankWorkspaceBody();
void addBlankWorkspaceContext(QVBoxLayout *layout, const QString &detailLens);

} // namespace DexTextEditorWorkspace

#pragma once

#include <QString>

class QPushButton;
class QVBoxLayout;

namespace DexRightContext {

void addWrappedLine(QVBoxLayout *contentLayout, const QString &text);
void addContextSection(QVBoxLayout *contentLayout, const QString &title);
QPushButton *makeContextActionButton(const QString &action, int width = 150, const QString &tooltip = QString(), bool enabled = true);

} // namespace DexRightContext

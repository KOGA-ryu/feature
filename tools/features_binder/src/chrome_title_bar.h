#pragma once

#include <QFrame>
#include <QPoint>
#include <QToolButton>

#include <functional>

class ChromeTitleBar final : public QFrame {
public:
    ChromeTitleBar(
        std::function<void()> on_close,
        std::function<void()> on_minimize,
        std::function<void()> on_zoom,
        QWidget *parent = nullptr);

    QToolButton *railToggle() const;
    QToolButton *bottomToggle() const;
    QToolButton *agentDividersToggle() const;
    QToolButton *contextToggle() const;

protected:
    void mousePressEvent(QMouseEvent *event) override;
    void mouseMoveEvent(QMouseEvent *event) override;
    void mouseReleaseEvent(QMouseEvent *event) override;
    void mouseDoubleClickEvent(QMouseEvent *event) override;

private:
    QToolButton *makeWindowButton(const QString &kind, std::function<void()> callback);
    QToolButton *makeToggle(const QString &text, const QString &tool_tip);
    bool isDragSurface(const QPoint &point) const;

    QToolButton *railToggle_ = nullptr;
    QToolButton *bottomToggle_ = nullptr;
    QToolButton *agentDividersToggle_ = nullptr;
    QToolButton *contextToggle_ = nullptr;
    std::function<void()> onClose_;
    std::function<void()> onMinimize_;
    std::function<void()> onZoom_;
    bool dragging_ = false;
    QPoint dragOffset_;
};

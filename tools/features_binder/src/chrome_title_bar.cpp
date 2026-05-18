#include "chrome_title_bar.h"

#include <QHBoxLayout>
#include <QLabel>
#include <QMouseEvent>
#include <QVBoxLayout>

#include "ui_rules.h"

ChromeTitleBar::ChromeTitleBar(
    std::function<void()> on_close,
    std::function<void()> on_minimize,
    std::function<void()> on_zoom,
    QWidget *parent)
    : QFrame(parent),
      onClose_(std::move(on_close)),
      onMinimize_(std::move(on_minimize)),
      onZoom_(std::move(on_zoom)) {
    setObjectName("titleBar");
    setFixedHeight(36);

    auto *layout = new QHBoxLayout(this);
    layout->setContentsMargins(10, 0, 10, 0);
    layout->setSpacing(8);

    layout->addWidget(makeWindowButton("close", [this]() { onClose_(); }));
    layout->addWidget(makeWindowButton("minimize", [this]() { onMinimize_(); }));
    layout->addWidget(makeWindowButton("zoom", [this]() { onZoom_(); }));

    auto *drag_pill = new QLabel;
    drag_pill->setObjectName("titleDragPill");
    drag_pill->setFixedSize(56, 14);
    layout->addWidget(drag_pill);

    auto *title_stack = new QWidget;
    auto *title_layout = new QVBoxLayout(title_stack);
    title_layout->setContentsMargins(2, 2, 8, 1);
    title_layout->setSpacing(0);
    title_layout->addWidget(dex_ui::make_label("Dex Home v2", "titleText"));
    title_layout->addWidget(dex_ui::make_label("Project rail / binder ledger", "titleSubtext"));
    layout->addWidget(title_stack, 1);

    railToggle_ = makeToggle("left", "Toggle left project rail");
    bottomToggle_ = makeToggle("bottom", "Toggle bottom shelf");
    bottomToggle_->setChecked(false);
    agentDividersToggle_ = makeToggle("agents", "Toggle agent page dividers (Ctrl+Shift+A)");
    contextToggle_ = makeToggle("right", "Toggle right context panel");

    layout->addWidget(railToggle_);
    layout->addWidget(bottomToggle_);
    layout->addWidget(agentDividersToggle_);
    layout->addWidget(contextToggle_);
}

QToolButton *ChromeTitleBar::railToggle() const { return railToggle_; }
QToolButton *ChromeTitleBar::bottomToggle() const { return bottomToggle_; }
QToolButton *ChromeTitleBar::agentDividersToggle() const { return agentDividersToggle_; }
QToolButton *ChromeTitleBar::contextToggle() const { return contextToggle_; }

void ChromeTitleBar::mousePressEvent(QMouseEvent *event) {
    if (event->button() == Qt::LeftButton && isDragSurface(event->position().toPoint())) {
        dragging_ = true;
        dragOffset_ = event->globalPosition().toPoint() - window()->frameGeometry().topLeft();
        event->accept();
        return;
    }
    QFrame::mousePressEvent(event);
}

void ChromeTitleBar::mouseMoveEvent(QMouseEvent *event) {
    if (dragging_ && (event->buttons() & Qt::LeftButton)) {
        window()->move(event->globalPosition().toPoint() - dragOffset_);
        event->accept();
        return;
    }
    QFrame::mouseMoveEvent(event);
}

void ChromeTitleBar::mouseReleaseEvent(QMouseEvent *event) {
    dragging_ = false;
    QFrame::mouseReleaseEvent(event);
}

void ChromeTitleBar::mouseDoubleClickEvent(QMouseEvent *event) {
    if (event->button() == Qt::LeftButton && isDragSurface(event->position().toPoint())) {
        onZoom_();
        event->accept();
        return;
    }
    QFrame::mouseDoubleClickEvent(event);
}

QToolButton *ChromeTitleBar::makeWindowButton(const QString &kind, std::function<void()> callback) {
    auto *button = new QToolButton;
    button->setObjectName("windowControl");
    button->setProperty("kind", kind);
    button->setToolTip(kind);
    button->setCursor(Qt::PointingHandCursor);
    connect(button, &QToolButton::clicked, this, [callback = std::move(callback)]() {
        callback();
    });
    return button;
}

QToolButton *ChromeTitleBar::makeToggle(const QString &text, const QString &tool_tip) {
    auto *button = new QToolButton;
    button->setObjectName("titleToggle");
    button->setText(text);
    button->setCheckable(true);
    button->setChecked(true);
    button->setToolTip(tool_tip);
    button->setCursor(Qt::PointingHandCursor);
    return button;
}

bool ChromeTitleBar::isDragSurface(const QPoint &point) const {
    if (QWidget *child = childAt(point)) {
        return qobject_cast<QToolButton *>(child) == nullptr;
    }
    return true;
}

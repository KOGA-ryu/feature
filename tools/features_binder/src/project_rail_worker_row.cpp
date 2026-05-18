#include "project_rail_rows.h"

#include <QFrame>
#include <QHBoxLayout>
#include <QMouseEvent>

#include "project_rail_elided_label.h"
#include "ui_rules.h"

namespace {

class WorkerRow final : public QFrame {
public:
    WorkerRow(
        const QString &workerId,
        const QString &role,
        const QString &displayName,
        const QString &status,
        bool selected,
        std::function<void(QString)> onSelected,
        QWidget *parent = nullptr)
        : QFrame(parent),
          workerId_(workerId),
          onSelected_(std::move(onSelected)) {
        setObjectName("workerRow");
        setProperty("selected", selected);
        setFixedHeight(dex_ui::metrics::worker_row_height);
        setCursor(Qt::PointingHandCursor);
        setToolTip("Select " + displayName + " worker lens");

        auto *layout = new QHBoxLayout(this);
        layout->setContentsMargins(8, 2, 8, 2);
        layout->setSpacing(6);

        auto *roleLabel = makeRailElidedLabel(role);
        roleLabel->setMaximumWidth(72);
        layout->addWidget(roleLabel);

        auto *nameLabel = makeRailElidedLabel(displayName, "mutedLabel");
        layout->addWidget(nameLabel, 1);

        auto *statusLabel = makeRailElidedLabel(status, "smallLabel");
        statusLabel->setMaximumWidth(36);
        statusLabel->setAlignment(Qt::AlignRight | Qt::AlignVCenter);
        layout->addWidget(statusLabel);
    }

protected:
    void mousePressEvent(QMouseEvent *event) override {
        if (event->button() == Qt::LeftButton && onSelected_) {
            onSelected_(workerId_);
            event->accept();
            return;
        }
        QFrame::mousePressEvent(event);
    }

private:
    QString workerId_;
    std::function<void(QString)> onSelected_;
};

} // namespace

namespace DexProjectRailRows {

QWidget *makeWorkerRow(
    const QString &workerId,
    const QString &role,
    const QString &displayName,
    const QString &status,
    bool selected,
    std::function<void(QString)> onSelected) {
    return new WorkerRow(workerId, role, displayName, status, selected, std::move(onSelected));
}

} // namespace DexProjectRailRows

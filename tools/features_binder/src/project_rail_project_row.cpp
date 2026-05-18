#include "project_rail_rows.h"

#include <QFrame>
#include <QHBoxLayout>
#include <QLabel>
#include <QMouseEvent>

#include "project_rail_elided_label.h"
#include "ui_rules.h"

namespace {

class ProjectRow final : public QFrame {
public:
    ProjectRow(
        const QString &projectId,
        const QString &name,
        const QString &path,
        const QString &status,
        bool active,
        std::function<void(QString)> onSelected = {},
        QWidget *parent = nullptr)
        : QFrame(parent),
          projectId_(projectId),
          path_(path),
          onSelected_(std::move(onSelected)) {
        setObjectName("projectRow");
        setProperty("active", active);
        setFixedHeight(dex_ui::metrics::project_row_height);
        setMouseTracking(true);
        setToolTip(name + "\n" + path + "\nstatus: " + status);
        if (onSelected_) {
            setCursor(Qt::PointingHandCursor);
        }

        auto *layout = new QHBoxLayout(this);
        layout->setContentsMargins(8, 5, 8, 5);
        layout->setSpacing(8);

        auto *icon = new QLabel;
        icon->setFixedSize(18, 18);
        icon->setStyleSheet("background:#eef2f5; border:1px solid #8d969f; border-radius:4px;");
        layout->addWidget(icon);

        auto *nameLabel = makeRailElidedLabel(name);
        nameLabel->setMinimumWidth(0);
        nameLabel->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Preferred);
        layout->addWidget(nameLabel, 1);
    }

protected:
    void enterEvent(QEnterEvent *event) override {
        QFrame::enterEvent(event);
        setToolTip(path_);
    }

    void mousePressEvent(QMouseEvent *event) override {
        if (event->button() == Qt::LeftButton && onSelected_) {
            onSelected_(projectId_);
            event->accept();
            return;
        }
        QFrame::mousePressEvent(event);
    }

private:
    QString projectId_;
    QString path_;
    std::function<void(QString)> onSelected_;
};

} // namespace

namespace DexProjectRailRows {

QWidget *makeProjectRow(
    const QString &projectId,
    const QString &name,
    const QString &path,
    const QString &status,
    bool active,
    std::function<void(QString)> onSelected) {
    return new ProjectRow(projectId, name, path, status, active, std::move(onSelected));
}

} // namespace DexProjectRailRows

#include "project_rail_elided_label.h"

#include <QResizeEvent>
#include <QSizePolicy>

namespace {

class RailElidedLabel final : public QLabel {
public:
    explicit RailElidedLabel(const QString &text, const char *objectName = nullptr, QWidget *parent = nullptr)
        : QLabel(parent),
          fullText_(text) {
        setText(text);
        setToolTip(text);
        setMinimumWidth(0);
        setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Preferred);
        if (objectName) {
            setObjectName(objectName);
        }
    }

protected:
    void resizeEvent(QResizeEvent *event) override {
        QLabel::resizeEvent(event);
        const QString elided = fontMetrics().elidedText(fullText_, Qt::ElideRight, width());
        QLabel::setText(elided);
    }

private:
    QString fullText_;
};

} // namespace

QLabel *makeRailElidedLabel(const QString &text, const char *objectName) {
    return new RailElidedLabel(text, objectName);
}

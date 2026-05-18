#include "sheet_stack_body.h"

#include <QResizeEvent>

#include <algorithm>

#include "ui_rules.h"

SheetStackBody::SheetStackBody(QWidget *parent) : QWidget(parent) {
    setObjectName("rootSurface");
}

void SheetStackBody::setSurfaces(QWidget *projectRail, QWidget *ledger, QWidget *detailLensRail, QWidget *rightContext) {
    projectRail_ = projectRail;
    ledger_ = ledger;
    detailLensRail_ = detailLensRail;
    rightContext_ = rightContext;
    relayoutSheets();
}

void SheetStackBody::resizeEvent(QResizeEvent *event) {
    QWidget::resizeEvent(event);
    relayoutSheets();
}

int SheetStackBody::visibleWidth(QWidget *widget) const {
    return widget && !widget->isHidden() ? widget->width() : 0;
}

void SheetStackBody::relayoutSheets() {
    if (!projectRail_ || !ledger_ || !detailLensRail_ || !rightContext_) {
        return;
    }

    const int fullWidth = width();
    const int fullHeight = height();
    const int overlap = dex_ui::metrics::sheet_overlap;
    const int railWidth = visibleWidth(projectRail_);
    const int contextWidth = visibleWidth(rightContext_);
    const int detailWidth = visibleWidth(detailLensRail_);
    const int ledgerX = railWidth > 0 ? std::max(0, railWidth - overlap) : 0;
    const int contextX = contextWidth > 0 ? fullWidth - contextWidth : fullWidth;
    const int detailX = contextWidth > 0 ? std::max(ledgerX, contextX - detailWidth - overlap) : fullWidth - detailWidth;
    const int ledgerRight = detailWidth > 0 ? detailX + overlap : contextX;
    const int ledgerWidth = std::max(120, ledgerRight - ledgerX);

    if (!projectRail_->isHidden()) {
        projectRail_->setGeometry(0, 0, railWidth, fullHeight);
    }
    ledger_->setGeometry(ledgerX, 0, ledgerWidth, fullHeight);
    if (!detailLensRail_->isHidden()) {
        detailLensRail_->setGeometry(detailX, 0, detailWidth, fullHeight);
    }
    if (!rightContext_->isHidden()) {
        rightContext_->setGeometry(contextX, 0, contextWidth, fullHeight);
    }

    ledger_->raise();
    if (!projectRail_->isHidden()) {
        projectRail_->raise();
    }
    if (!detailLensRail_->isHidden()) {
        detailLensRail_->raise();
    }
    if (!rightContext_->isHidden()) {
        rightContext_->raise();
    }
}

#include "detail_lens_rail.h"

#include <QButtonGroup>
#include <QPushButton>
#include <QVBoxLayout>

#include "binder_navigation.h"
#include "render_helpers.h"

DetailLensRail::DetailLensRail(std::function<void(QString)> onLensSelected, QWidget *parent)
    : QFrame(parent),
      onLensSelected_(std::move(onLensSelected)) {
    setObjectName("detailLensRail");
    setFixedWidth(54);

    layout_ = new QVBoxLayout(this);
    layout_->setContentsMargins(4, 24, 4, 8);
    layout_->setSpacing(5);

    tabGroup_ = new QButtonGroup(this);
    tabGroup_->setExclusive(true);
    connect(tabGroup_, &QButtonGroup::idClicked, this, [this](int index) {
        const QString lens = lenses_.value(index, QString());
        if (lens.isEmpty()) {
            return;
        }
        currentLens_ = lens;
        onLensSelected_(currentLens_);
    });
}

void DetailLensRail::setTopTab(const QString &topTab, const QString &requestedLens, bool repoMode) {
    clearLayout(layout_);
    lenses_ = detailLensTabsFor(topTab, repoMode);
    const int requestedIndex = lenses_.contains(requestedLens)
        ? lenses_.indexOf(requestedLens)
        : 0;

    delete tabGroup_;
    tabGroup_ = new QButtonGroup(this);
    tabGroup_->setExclusive(true);
    connect(tabGroup_, &QButtonGroup::idClicked, this, [this](int index) {
        const QString lens = lenses_.value(index, QString());
        if (lens.isEmpty()) {
            return;
        }
        currentLens_ = lens;
        onLensSelected_(currentLens_);
    });

    for (int i = 0; i < lenses_.size(); ++i) {
        auto *button = new QPushButton(detailLensShortLabel(lenses_.at(i)));
        button->setObjectName("detailLensTab");
        button->setProperty("tabIndex", i);
        button->setProperty("firstTab", i == 0);
        button->setProperty("lastTab", i == lenses_.size() - 1);
        button->setCheckable(true);
        button->setToolTip(lenses_.at(i));
        if (i == requestedIndex) {
            button->setChecked(true);
        }
        tabGroup_->addButton(button, i);
        layout_->addWidget(button);
    }
    layout_->addStretch(1);
    currentLens_ = lenses_.value(requestedIndex, QString("Summary"));
}

QString DetailLensRail::currentLens() const {
    return currentLens_;
}

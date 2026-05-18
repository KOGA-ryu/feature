#pragma once

#include <QFrame>

#include <functional>

class QButtonGroup;
class QVBoxLayout;

class DetailLensRail final : public QFrame {
public:
    explicit DetailLensRail(std::function<void(QString)> onLensSelected, QWidget *parent = nullptr);

    void setTopTab(const QString &topTab, const QString &requestedLens, bool repoMode);
    QString currentLens() const;

private:
    QVBoxLayout *layout_ = nullptr;
    QButtonGroup *tabGroup_ = nullptr;
    QStringList lenses_;
    QString currentLens_;
    std::function<void(QString)> onLensSelected_;
};

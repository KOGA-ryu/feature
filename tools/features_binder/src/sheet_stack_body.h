#pragma once

#include <QWidget>

class SheetStackBody final : public QWidget {
public:
    explicit SheetStackBody(QWidget *parent = nullptr);

    void setSurfaces(QWidget *projectRail, QWidget *ledger, QWidget *detailLensRail, QWidget *rightContext);
    void relayoutSheets();

protected:
    void resizeEvent(QResizeEvent *event) override;

private:
    int visibleWidth(QWidget *widget) const;

    QWidget *projectRail_ = nullptr;
    QWidget *ledger_ = nullptr;
    QWidget *detailLensRail_ = nullptr;
    QWidget *rightContext_ = nullptr;
};

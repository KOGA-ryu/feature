#pragma once

#include <QWidget>

#include "text_action_proof_model.h"

class QCheckBox;
class QComboBox;
class QGridLayout;
class QLabel;
class QLineEdit;
class QPlainTextEdit;
class QSpinBox;

class TextActionProofPanel final : public QWidget {
public:
    explicit TextActionProofPanel(QWidget *parent = nullptr);

private:
    void rebuildActionButtons();
    void executeAction(const QString &actionId);
    void loadSelectedFixture();
    void runSelectedFixture();
    void runAllFixtures();
    DexTextActions::TextActionProofInput currentInput() const;
    QString selectedText() const;
    void renderResult(const DexTextActions::HostActionResult &result);
    void renderFixtureResult(const DexTextActions::TextActionFixtureResult &result);
    void renderFixtureSuiteResult(const DexTextActions::TextActionFixtureSuiteResult &result);

    QComboBox *fixturePicker_ = nullptr;
    QPlainTextEdit *editor_ = nullptr;
    QLineEdit *language_ = nullptr;
    QLineEdit *source_ = nullptr;
    QSpinBox *startLine_ = nullptr;
    QSpinBox *endLine_ = nullptr;
    QCheckBox *stripAnsi_ = nullptr;
    QGridLayout *actionsLayout_ = nullptr;
    QPlainTextEdit *output_ = nullptr;
    QLabel *status_ = nullptr;
    QLabel *receipt_ = nullptr;
};

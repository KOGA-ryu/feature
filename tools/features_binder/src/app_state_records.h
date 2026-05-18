#pragma once

#include <QString>
#include <QStringList>

struct ProjectSummary {
    QString id;
    QString name;
    QString path;
    QString status;
    bool pinned = false;
    QStringList workerIds;
};

struct WorkerSummary {
    QString id;
    QString role;
    QString displayName;
    QString status;
    QString codexSessionId;
    QString codexModel;
    QString codexDirectory;
    QString codexPermissions;
    QString codexContextWindow;
    QString codexAccount;
};

struct LedgerEntry {
    QString recordId;
    QString sequence;
    QString timestamp;
    QString kind;
    QString projectId;
    QString projectPath;
    QString workerRole;
    QString sessionId;
    QString title;
    QString summary;
    QString status;
    QString grade;
    QStringList notes;
    QStringList proofRefs;
    QStringList transcriptRefs;
    QStringList changedFiles;
    QStringList reviewerNotes;
    QString packetId;
    QString decision;
    QString proofStatus;
    QStringList tags;
    QString source;
};

struct BenchCandidate {
    QString id;
    QString name;
    QString repoPath;
    QString classification;
    QString status;
    QString contribution;
    QStringList risks;
    QString nextAction;
};

struct ContextPanelState {
    QString title;
    QString subtitle;
    QStringList lines;
};

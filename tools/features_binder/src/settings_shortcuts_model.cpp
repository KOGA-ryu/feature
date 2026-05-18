#include "settings_shortcuts_model.h"

namespace DexSettingsShortcuts {

QVector<ShortcutCommandTab> shortcutCommandTabs() {
    return {
        {
            "Features Binder",
            {
                {
                    "Open Features Binder",
                    "~/dev/features/tools/features_binder/proof",
                    "Proof command; rebuilds, tests, and refreshes screenshots.",
                },
                {
                    "Build binder",
                    "cmake --build ~/dev/features/tools/features_binder/build",
                    "Compile the Qt binder after code changes.",
                },
                {
                    "Run binder smoke tests",
                    "ctest --test-dir ~/dev/features/tools/features_binder/build --output-on-failure",
                    "Runs the state/backend smoke test.",
                },
                {
                    "Open Text Editor",
                    "--no-settings --tab \"Text Editor\"",
                    "Launch the main Text Editor workbench.",
                },
            },
        },
        {
            "Text Editor",
            {
                {
                    "Copy Plain",
                    "Ctrl+C / Cmd+C",
                    "Host shortcut target for text.copy_plain.",
                },
                {
                    "Select All",
                    "Ctrl+A / Cmd+A",
                    "Host shortcut target for text.select_all.",
                },
                {
                    "Run Fixture",
                    "button",
                    "Compares generated action output against expected fixture output.",
                },
                {
                    "Load Fixture",
                    "button",
                    "Loads fixture input and expected output into the proof panel.",
                },
                {
                    "Clean Basic",
                    "command palette",
                    "Named cleanup action; no silent text transforms.",
                },
            },
        },
        {
            "OpenAI Codex",
            {
                {
                    "Archive chat",
                    "⇧⌘A",
                    "Codex app shortcut: Archive the current chat.",
                },
                {
                    "New chat",
                    "⌘N / ⇧⌘O",
                    "Codex app shortcut: Start a new chat.",
                },
                {
                    "Open side chat",
                    "Unassigned",
                    "Codex app shortcut: Open the current chat in a side chat.",
                },
                {
                    "Open in new window",
                    "Unassigned",
                    "Codex app shortcut: Open the current chat in a new window.",
                },
                {
                    "New quick chat",
                    "⌥⌘N",
                    "Codex app shortcut: Start a lightweight chat in the quick composer.",
                },
                {
                    "Toggle pin",
                    "⌥⌘P",
                    "Codex app shortcut: Pin or unpin the current chat.",
                },
                {
                    "Find",
                    "⌘F",
                    "Codex app shortcut: Search the current chat.",
                },
                {
                    "Focus browser address bar",
                    "⌘L",
                    "Codex app shortcut: Focus the in-app browser address bar.",
                },
                {
                    "Back",
                    "⌘[",
                    "Codex app shortcut: Go back in navigation history.",
                },
                {
                    "Forward",
                    "⌘]",
                    "Codex app shortcut: Go forward in navigation history.",
                },
                {
                    "Next chat",
                    "⇧⌘]",
                    "Codex app shortcut: Switch to the next chat.",
                },
                {
                    "Previous chat",
                    "⇧⌘[",
                    "Codex app shortcut: Switch to the previous chat.",
                },
                {
                    "Open browser tab",
                    "⌘T",
                    "Codex app shortcut: Open a browser tab.",
                },
                {
                    "Open review tab",
                    "Unassigned",
                    "Codex app shortcut: Open the review tab.",
                },
                {
                    "Toggle browser panel",
                    "⇧⌘B",
                    "Codex app shortcut: Show or hide the browser panel.",
                },
                {
                    "Toggle sidebar",
                    "⌘B",
                    "Codex app shortcut: Show or hide the sidebar.",
                },
                {
                    "Toggle side panel",
                    "⌥⌘B",
                    "Codex app shortcut: Show or hide the side panel.",
                },
                {
                    "Toggle terminal",
                    "⌘J",
                    "Codex app shortcut: Show or hide the terminal panel.",
                },
                {
                    "Open folder",
                    "⌘O",
                    "Codex app shortcut: Add a local project to Codex.",
                },
                {
                    "Force reload skills",
                    "Unassigned",
                    "Codex app shortcut: Refresh the skill catalog for the current context.",
                },
                {
                    "Go to skills",
                    "Unassigned",
                    "Codex app shortcut: Browse installed and recommended skills.",
                },
                {
                    "Install Codex Workspace",
                    "Unassigned",
                    "Codex app shortcut: Install dependencies for advanced local features.",
                },
                {
                    "Keyboard shortcuts",
                    "Unassigned",
                    "Codex app shortcut: Customize keyboard shortcuts.",
                },
                {
                    "MCP",
                    "Unassigned",
                    "Codex app shortcut: Configure MCP servers.",
                },
                {
                    "Personality",
                    "Unassigned",
                    "Codex app shortcut: Adjust tone and response style.",
                },
                {
                    "Feedback",
                    "Unassigned",
                    "Codex app shortcut: Send product feedback to the Codex team.",
                },
                {
                    "Log out",
                    "Unassigned",
                    "Codex app shortcut: Sign out of Codex.",
                },
                {
                    "Manage automations",
                    "Unassigned",
                    "Codex app shortcut: Create or manage automations from the current page.",
                },
                {
                    "Wake Pet",
                    "Unassigned",
                    "Codex app shortcut: Open the pet overlay.",
                },
                {
                    "Open control window",
                    "Unassigned",
                    "Codex app shortcut: Open the voice control window.",
                },
                {
                    "Settings",
                    "⌘,",
                    "Codex app shortcut: Open Codex settings.",
                },
                {
                    "Approve request",
                    "⏎",
                    "Codex app shortcut: Approve the active request.",
                },
                {
                    "Decline request",
                    "Escape",
                    "Codex app shortcut: Decline the active request.",
                },
                {
                    "Close",
                    "⌘W",
                    "Codex app shortcut: Close the active tab or window.",
                },
                {
                    "Open model picker",
                    "⌃⇧M",
                    "Codex app shortcut: Open the composer model picker.",
                },
                {
                    "Start dictation",
                    "⌃⇧D",
                    "Codex app shortcut: Start dictation in the current composer.",
                },
                {
                    "Toggle voice mode",
                    "⌃⇧V",
                    "Codex app shortcut: Start or stop voice mode.",
                },
                {
                    "Copy as Markdown",
                    "Unassigned",
                    "Codex app shortcut: Copy the current chat as Markdown.",
                },
                {
                    "Copy conversation path",
                    "⌥⇧⌘C",
                    "Codex app shortcut: Copy the current chat path.",
                },
                {
                    "Copy deeplink",
                    "⌥⌘L",
                    "Codex app shortcut: Copy a deeplink to the current chat.",
                },
                {
                    "Copy session id",
                    "⌥⌘C",
                    "Codex app shortcut: Copy the current chat session ID.",
                },
                {
                    "Copy working directory",
                    "⇧⌘C",
                    "Codex app shortcut: Copy the current chat working directory.",
                },
                {
                    "Hold-to-dictate hotkey",
                    "Unassigned",
                    "Codex app shortcut: Hold anywhere on desktop to dictate where your cursor is.",
                },
                {
                    "Toggle dictation hotkey",
                    "Unassigned",
                    "Codex app shortcut: Press once anywhere on desktop to dictate, then press again to stop.",
                },
                {
                    "Force Reload Browser Page",
                    "⇧⌘R",
                    "Codex app shortcut: Force reload the active browser page.",
                },
                {
                    "Popout Window hotkey",
                    "Unassigned",
                    "Codex app shortcut: Show or hide Popout Window from anywhere on desktop.",
                },
                {
                    "New Window",
                    "⇧⌘N",
                    "Codex app shortcut: Open a new window.",
                },
                {
                    "Next recently viewed chat",
                    "⌃Tab",
                    "Codex app shortcut: Cycle to the next recently viewed chat.",
                },
                {
                    "Open command menu",
                    "⌘K / ⇧⌘P",
                    "Codex app shortcut: Open the command menu.",
                },
                {
                    "Previous recently viewed chat",
                    "⌃⇧Tab",
                    "Codex app shortcut: Cycle to the previous recently viewed chat.",
                },
                {
                    "Reload Browser Page",
                    "⌘R",
                    "Codex app shortcut: Reload the active browser page.",
                },
                {
                    "Rename chat",
                    "⌥⌘R",
                    "Codex app shortcut: Rename the current chat.",
                },
                {
                    "Search Chats…",
                    "⌘G",
                    "Codex app shortcut: Search chats.",
                },
                {
                    "Search Files…",
                    "⌘P",
                    "Codex app shortcut: Search files.",
                },
                {
                    "Show keyboard shortcuts",
                    "⌘?",
                    "Codex app shortcut: Show the shortcuts available right now.",
                },
                {
                    "Go to chat 1",
                    "⌘1",
                    "Codex app shortcut: Open the visible chat in this shortcut slot.",
                },
                {
                    "Go to chat 2",
                    "⌘2",
                    "Codex app shortcut: Open the visible chat in this shortcut slot.",
                },
                {
                    "Go to chat 3",
                    "⌘3",
                    "Codex app shortcut: Open the visible chat in this shortcut slot.",
                },
                {
                    "Go to chat 4",
                    "⌘4",
                    "Codex app shortcut: Open the visible chat in this shortcut slot.",
                },
                {
                    "Go to chat 5",
                    "⌘5",
                    "Codex app shortcut: Open the visible chat in this shortcut slot.",
                },
                {
                    "Go to chat 6",
                    "⌘6",
                    "Codex app shortcut: Open the visible chat in this shortcut slot.",
                },
                {
                    "Go to chat 7",
                    "⌘7",
                    "Codex app shortcut: Open the visible chat in this shortcut slot.",
                },
                {
                    "Go to chat 8",
                    "⌘8",
                    "Codex app shortcut: Open the visible chat in this shortcut slot.",
                },
                {
                    "Go to chat 9",
                    "⌘9",
                    "Codex app shortcut: Open the visible chat in this shortcut slot.",
                },
                {
                    "Toggle File Tree",
                    "⇧⌘E",
                    "Codex app shortcut: Toggle the file tree panel.",
                },
                {
                    "Start Trace Recording",
                    "⇧⌘S",
                    "Codex app shortcut: Start or stop trace recording.",
                },
            },
        },
    };
}

} // namespace DexSettingsShortcuts

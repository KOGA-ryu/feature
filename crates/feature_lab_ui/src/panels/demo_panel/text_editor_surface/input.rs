use eframe::egui;
use text_editor_plain::{EditorCommand, TextEditorPlain};

pub(super) fn handle_focused_input(
    ui: &mut egui::Ui,
    editor: &mut TextEditorPlain,
    has_focus: bool,
) {
    if !has_focus {
        return;
    }

    let mut handled_input = false;
    for event in ui.input(|input| input.events.clone()) {
        match event {
            egui::Event::Text(text) => {
                let filtered = filter_printable_text(&text);
                if !filtered.is_empty() {
                    editor.apply(EditorCommand::InsertText(filtered));
                    handled_input = true;
                }
            }
            egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } => {
                if let Some(command) = command_for_key(key, modifiers) {
                    editor.apply(command);
                    handled_input = true;
                }
            }
            _ => {}
        }
    }

    if handled_input {
        ui.ctx().request_repaint();
    }
}

fn filter_printable_text(text: &str) -> String {
    text.chars()
        .filter(|character| {
            *character != '\n'
                && *character != '\r'
                && *character != '\t'
                && !character.is_control()
        })
        .collect()
}

fn command_for_key(key: egui::Key, modifiers: egui::Modifiers) -> Option<EditorCommand> {
    let shortcut = modifiers.command || modifiers.ctrl;
    match key {
        egui::Key::Enter => Some(EditorCommand::InsertNewline),
        egui::Key::Backspace => Some(EditorCommand::Backspace),
        egui::Key::Delete => Some(EditorCommand::DeleteForward),
        egui::Key::ArrowLeft => Some(EditorCommand::MoveLeft {
            extend: modifiers.shift,
        }),
        egui::Key::ArrowRight => Some(EditorCommand::MoveRight {
            extend: modifiers.shift,
        }),
        egui::Key::ArrowUp => Some(EditorCommand::MoveUp {
            extend: modifiers.shift,
        }),
        egui::Key::ArrowDown => Some(EditorCommand::MoveDown {
            extend: modifiers.shift,
        }),
        egui::Key::Home => Some(EditorCommand::MoveLineStart {
            extend: modifiers.shift,
        }),
        egui::Key::End => Some(EditorCommand::MoveLineEnd {
            extend: modifiers.shift,
        }),
        egui::Key::A if shortcut => Some(EditorCommand::SelectAll),
        egui::Key::Z if shortcut && modifiers.shift => Some(EditorCommand::Redo),
        egui::Key::Z if shortcut => Some(EditorCommand::Undo),
        egui::Key::Y if modifiers.ctrl => Some(EditorCommand::Redo),
        _ => None,
    }
}

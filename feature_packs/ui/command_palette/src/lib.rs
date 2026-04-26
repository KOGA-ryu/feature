use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.command_palette";
const EMPTY_STATE_MESSAGE: &str = "No commands match the current query.";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandItem {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub category: String,
    pub keywords: Vec<String>,
    pub enabled: bool,
    pub shortcut: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandFixture {
    pub commands: Vec<CommandItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandGroup {
    pub category: String,
    pub commands: Vec<CommandItem>,
}

#[derive(Debug, Clone)]
pub struct CommandPalette {
    commands: Vec<CommandItem>,
    query: String,
    selected_index: usize,
}

impl CommandPalette {
    pub fn new(commands: Vec<CommandItem>) -> Self {
        Self {
            commands,
            query: String::new(),
            selected_index: 0,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let fixture: CommandFixture =
            serde_json::from_str(raw).map_err(|error| error.to_string())?;
        Ok(Self::new(fixture.commands))
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.clamp_selected_index();
    }

    pub fn set_selected_index(&mut self, selected_index: usize) {
        self.selected_index = selected_index;
        self.clamp_selected_index();
    }

    pub fn selected_index(&self) -> usize {
        self.clamped_selected_index_for(self.visible_commands().len())
    }

    pub fn visible_commands(&self) -> Vec<&CommandItem> {
        let query = normalized_query(&self.query);
        self.commands
            .iter()
            .filter(|command| matches_query(command, &query))
            .collect()
    }

    pub fn grouped_results(&self) -> Vec<CommandGroup> {
        let mut groups: Vec<CommandGroup> = Vec::new();
        for command in self.visible_commands() {
            if let Some(group) = groups
                .iter_mut()
                .find(|group| group.category == command.category)
            {
                group.commands.push(command.clone());
            } else {
                groups.push(CommandGroup {
                    category: command.category.clone(),
                    commands: vec![command.clone()],
                });
            }
        }
        groups
    }

    pub fn move_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
        self.clamp_selected_index();
    }

    pub fn move_down(&mut self) {
        let visible_len = self.visible_commands().len();
        if visible_len == 0 {
            self.selected_index = 0;
            return;
        }
        self.selected_index = (self.selected_index + 1).min(visible_len - 1);
    }

    pub fn selected_command(&self) -> Option<&CommandItem> {
        let visible = self.visible_commands();
        visible.get(self.selected_index()).copied()
    }

    pub fn activate_selected(&self) -> Option<String> {
        self.selected_command()
            .and_then(|command| command.enabled.then(|| command.id.clone()))
    }

    pub fn empty_state_message(&self) -> Option<&'static str> {
        self.visible_commands()
            .is_empty()
            .then_some(EMPTY_STATE_MESSAGE)
    }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_commands.json")
}

pub fn sample_palette() -> Result<CommandPalette, String> {
    CommandPalette::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}

fn normalized_query(query: &str) -> String {
    query.trim().to_lowercase()
}

fn matches_query(command: &CommandItem, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    command.id.to_lowercase().contains(query)
        || command.title.to_lowercase().contains(query)
        || command.subtitle.to_lowercase().contains(query)
        || command.category.to_lowercase().contains(query)
        || command
            .keywords
            .iter()
            .any(|keyword| keyword.to_lowercase().contains(query))
}

impl CommandPalette {
    fn clamp_selected_index(&mut self) {
        self.selected_index = self.clamped_selected_index_for(self.visible_commands().len());
    }

    fn clamped_selected_index_for(&self, visible_len: usize) -> usize {
        match visible_len {
            0 => 0,
            len => self.selected_index.min(len - 1),
        }
    }
}

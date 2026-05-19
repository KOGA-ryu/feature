use std::io::{self, Read};

use serde::{Deserialize, Serialize};
use text_editor_actions::{TextActionId, TextActionInput};
use text_editor_host_adapter::{HostActionResult, execute_host_action};
use text_editor_plain::{EditorCommand, EditorPosition, EditorSelection, TextEditorPlain};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RunnerRequest {
    action_id: String,
    document_text: String,
    #[serde(default)]
    selection: Option<EditorSelection>,
    #[serde(default)]
    input: TextActionInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RunnerResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<HostActionResult>,
    editor_text: String,
    selection: EditorSelection,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn main() {
    let response = read_request()
        .and_then(run_request)
        .unwrap_or_else(error_response);

    println!(
        "{}",
        serde_json::to_string_pretty(&response).expect("runner response should serialize")
    );
}

fn read_request() -> Result<RunnerRequest, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("failed to read stdin: {error}"))?;
    serde_json::from_str(&input).map_err(|error| format!("invalid request json: {error}"))
}

fn run_request(request: RunnerRequest) -> Result<RunnerResponse, String> {
    let action_id = TextActionId::parse(&request.action_id)
        .ok_or_else(|| format!("unknown action id: {}", request.action_id))?;

    let mut editor = TextEditorPlain::from_text(request.document_text);
    if let Some(selection) = request.selection {
        editor.apply(EditorCommand::SetSelection(selection));
    }

    let result = execute_host_action(&mut editor, action_id, request.input);
    Ok(RunnerResponse {
        ok: true,
        result: Some(result),
        editor_text: editor.text().to_owned(),
        selection: editor.selection(),
        error: None,
    })
}

fn error_response(error: String) -> RunnerResponse {
    RunnerResponse {
        ok: false,
        result: None,
        editor_text: String::new(),
        selection: EditorSelection::collapsed(EditorPosition::default()),
        error: Some(error),
    }
}

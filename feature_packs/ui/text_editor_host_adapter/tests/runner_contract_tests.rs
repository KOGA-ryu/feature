use std::io::Write;
use std::process::{Command, Stdio};

use serde_json::{Value, json};

fn run_runner(request: Value) -> Value {
    let runner = env!("CARGO_BIN_EXE_text_editor_action_runner");
    let mut child = Command::new(runner)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("runner should spawn");

    child
        .stdin
        .as_mut()
        .expect("stdin should exist")
        .write_all(request.to_string().as_bytes())
        .expect("request should write");

    let output = child.wait_with_output().expect("runner should finish");
    assert!(output.status.success());
    serde_json::from_slice(&output.stdout).expect("runner stdout should be json")
}

#[test]
fn runner_executes_copy_plain_with_selected_text() {
    let response = run_runner(json!({
        "action_id": "text.copy_plain",
        "document_text": "alpha\nbeta\nomega",
        "selection": {
            "anchor": {"line": 1, "column": 0},
            "caret": {"line": 1, "column": 4}
        },
        "input": {}
    }));

    assert_eq!(response["ok"], true);
    assert_eq!(response["result"]["kind"], "text");
    assert_eq!(response["result"]["clipboard_text"], "beta");
}

#[test]
fn runner_executes_prompt_block_with_source() {
    let response = run_runner(json!({
        "action_id": "text.copy_prompt_block",
        "document_text": "build the next slice",
        "input": {"source": "features_binder"}
    }));

    assert_eq!(response["ok"], true);
    assert_eq!(
        response["result"]["clipboard_text"],
        "Source: features_binder\n\n```text\nbuild the next slice\n```"
    );
}

#[test]
fn runner_returns_updated_selection_for_select_all() {
    let response = run_runner(json!({
        "action_id": "text.select_all",
        "document_text": "alpha\nbeta",
        "input": {}
    }));

    assert_eq!(response["ok"], true);
    assert_eq!(response["result"]["kind"], "none");
    assert_eq!(response["selection"]["anchor"]["line"], 0);
    assert_eq!(response["selection"]["anchor"]["column"], 0);
    assert_eq!(response["selection"]["caret"]["line"], 1);
    assert_eq!(response["selection"]["caret"]["column"], 4);
}

#[test]
fn runner_preserves_disabled_result_for_empty_copy() {
    let response = run_runner(json!({
        "action_id": "text.copy_plain",
        "document_text": "",
        "input": {}
    }));

    assert_eq!(response["ok"], true);
    assert_eq!(response["result"]["kind"], "disabled");
    assert_eq!(response["result"]["clipboard_text"], Value::Null);
    assert_eq!(response["result"]["warnings"][0], "document is empty");
}

#[test]
fn runner_reports_clean_basic_changes_and_warnings() {
    let response = run_runner(json!({
        "action_id": "text.clean_basic",
        "document_text": "one  \r\ntwo\t\r\n\u{001b}[31mred\u{001b}[0m",
        "input": {"strip_ansi_escape_codes": true}
    }));

    assert_eq!(response["ok"], true);
    assert_eq!(response["result"]["kind"], "clipboard_transform");
    assert_eq!(response["result"]["clipboard_text"], "one\ntwo\nred");
    assert_eq!(response["result"]["receipt_summary"]["change_count"], 4);
    assert_eq!(response["result"]["receipt_summary"]["warning_count"], 0);
}

#[test]
fn runner_rejects_unknown_action_id() {
    let response = run_runner(json!({
        "action_id": "text.nope",
        "document_text": "alpha",
        "input": {}
    }));

    assert_eq!(response["ok"], false);
    assert!(response["error"].as_str().unwrap().contains("unknown action id"));
}

use std::io;

use crate::launcher::{EntryId, Event};

use super::Launcher;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum EventNames {
    InputChange,
    CustomKey,
    ActiveEntry,
    SelectEntry,
    DeleteEntry,
    ExecCustomInput,
}

#[derive(Deserialize, Debug)]
struct IncomingMessage {
    #[serde(rename(deserialize = "name"))]
    event: EventNames,
    value: String,
    data: String,
}

#[derive(Serialize, Deserialize)]
struct ExtraMessageData {
    entry_id: usize,
}

#[derive(Serialize, Default)]
struct OutgoingMessageLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    urgent: bool,
    highlight: bool,
    markup: bool,
    nonselectable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
}

#[derive(Serialize)]
struct OutgoingMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overlay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input action")]
    input_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event format")]
    event_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "active entry")]
    active_entry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<Vec<OutgoingMessageLine>>,
}

impl Default for OutgoingMessage {
    fn default() -> Self {
        Self {
            message: Default::default(),
            overlay: Default::default(),
            prompt: Default::default(),
            input: Default::default(),
            input_action: Some(String::from("send")),
            event_format: Some(String::from(
                r#"{"name":"{{name_enum}}","value":"{{value_escaped}}","data":"{{data_escaped}}"}"#,
            )),
            active_entry: Default::default(),
            lines: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct RofiLauncher {}

impl RofiLauncher {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Launcher for RofiLauncher {
    fn wait(&self) -> anyhow::Result<super::Event> {
        loop {
            let mut line = String::new();
            let _ = io::stdin().read_line(&mut line);
            eprintln!("parsing: {}", line);
            let msg = serde_json::from_str::<IncomingMessage>(&line)?;
            eprintln!("{:?}", msg.event);

            match msg.event {
                EventNames::SelectEntry => {
                    let id = match msg.data.parse::<EntryId>() {
                        Ok(id) => id,
                        Err(err) => {
                            eprintln!("failed to parse entry id: {err}");
                            continue;
                        }
                    };
                    return Ok(Event::SelectEntry(id));
                }
                _ => continue,
            }
        }
    }

    fn update(&mut self, entries: &mut Vec<super::Entry>) -> anyhow::Result<()> {
        let msg_lines = entries
            .drain(..)
            .map(|x| OutgoingMessageLine {
                text: Some(x.text),
                icon: x.icon,
                data: Some(x.id.to_string()),
                ..Default::default()
            })
            .collect();
        let msg = OutgoingMessage {
            lines: Some(msg_lines),
            ..Default::default()
        };
        println!("{}", &serde_json::to_string(&msg)?);
        Ok(())
    }
}

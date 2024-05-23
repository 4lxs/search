use std::io;

use crate::{launcher::Event, searcher};

use super::Launcher;
use anyhow::anyhow;
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
    #[serde(rename(deserialize = "value"))]
    _value: String,
    data: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct ExtraMessageData {
    entry_id: usize,
}

#[derive(Serialize, Default)]
struct OutgoingMessageLine<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    urgent: bool,
    highlight: bool,
    markup: bool,
    nonselectable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<&'a str>,
    data: String,
}

#[derive(Serialize)]
struct OutgoingMessage<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overlay: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input action")]
    input_action: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event format")]
    event_format: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "active entry")]
    active_entry: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<Vec<OutgoingMessageLine<'a>>>,
}

impl<'a> Default for OutgoingMessage<'a> {
    fn default() -> Self {
        Self {
            message: Default::default(),
            overlay: Default::default(),
            prompt: Default::default(),
            input: Default::default(),
            input_action: Some("send"),
            event_format: Some(
                r#"{"name":"{{name_enum}}","value":"{{value_escaped}}","data":"{{data_escaped}}"}"#,
            ),
            active_entry: Default::default(),
            lines: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct RofiLauncher {
    entries: Vec<searcher::Entry>,
}

impl RofiLauncher {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> Launcher<'a> for RofiLauncher {
    fn wait(&'a self) -> anyhow::Result<Event<'a>> {
        loop {
            let mut line = String::new();
            let _ = io::stdin().read_line(&mut line);
            eprintln!("parsing: {}", line);
            let msg = serde_json::from_str::<IncomingMessage>(&line)?;
            eprintln!("{:?}", msg.event);

            match msg.event {
                EventNames::SelectEntry => {
                    return if let Ok(data) = serde_json::from_str::<ExtraMessageData>(&msg.data) {
                        if let Some(entry) = self.entries.get(data.entry_id) {
                            Ok(Event::SelectEntry(entry))
                        } else {
                            Err(anyhow!("Invalid entry id. shouldn't happen"))
                        }
                    } else {
                        Err(anyhow!("expected data with SelectEntry"))
                    }
                }
                _ => continue,
            }
        }
    }

    fn update<'b>(&mut self, entries: impl Iterator<Item = searcher::Entry>) -> anyhow::Result<()> {
        self.entries = entries.collect();
        let msg_lines = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, x)| OutgoingMessageLine {
                text: Some(&x.text),
                icon: x.icon.as_deref(),
                data: serde_json::to_string(&ExtraMessageData { entry_id: i }).unwrap(),
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

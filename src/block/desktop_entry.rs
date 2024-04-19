use anyhow::{anyhow, Context};
use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Write,
    path::PathBuf,
    process::Command,
};

use super::{Block, BlockLine, BlockLineData};

pub struct LineData {
    cmd: String,
}

impl LineData {
    fn exec(&self) -> anyhow::Result<()> {
        // TODO: redo this with actual implementation. this is shit

        let mut previsperc = false;
        let cmd: String = self
            .cmd
            .chars()
            .filter(|c| {
                let res = c != &'%' && !previsperc;
                previsperc = c == &'%';
                return res;
            })
            .collect();

        eprintln!("running {cmd}");

        Command::new("/bin/sh")
            .args(["-c", &cmd])
            .spawn()
            .with_context(|| format!("running cmd={}", cmd))?;
        Ok(())
    }
}

#[derive(Default)]
pub struct DesktopEntryBlock {
    desktop_entries: HashMap<String, PathBuf>,
}

impl Block for DesktopEntryBlock {
    fn get_lines<'a>(&mut self, _input: String) -> Vec<BlockLine> {
        let mut lines = vec![];
        for path in Iter::new(default_paths()) {
            eprintln!("reading {}", path.to_str().unwrap());
            let bytes = match std::fs::read_to_string(&path) {
                Ok(bytes) => Box::new(bytes),
                Err(err) => {
                    eprintln!("ignoring error: {}", err);
                    continue;
                }
            };
            let entry = match DesktopEntry::decode(&path, &bytes) {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("ignoring error: {}", err);
                    continue;
                }
            };

            eprintln!(
                "Name: {:?}, Exec: {}",
                entry.name(Option::None),
                entry.exec().unwrap_or("/none/")
            );

            if entry.no_display() {
                continue;
            }

            match entry.type_() {
                Some(type_) if type_ == "Application" => {}
                _ => continue,
            }

            let mut text = match entry.name(Option::None) {
                Some(name) => name.to_string(),
                None => continue,
            };
            if let Some(generic_name) = entry.generic_name(Option::None) {
                write!(
                    text,
                    " <span weight='light' size='small'><i>({})</i></span>",
                    generic_name.to_string()
                )
                .unwrap();
            }

            // TODO: onlyshowin, notshowin

            // TODO: default icon?
            let icon = entry.icon().map(str::to_string);

            let cmd = match entry.exec() {
                Some(cmd) => cmd.into(),
                None => continue,
            };

            let entry_id = entry.id().to_string();
            let path = match self.desktop_entries.entry(entry_id.clone()) {
                Entry::Occupied(..) => continue,
                Entry::Vacant(e) => e.insert(path.clone()),
            };

            let line_data = LineData { cmd };

            eprintln!("entry {:?}: {}", path, text);
            let entryline = BlockLine {
                text,
                icon,
                data: BlockLineData::DesktopEntryData(line_data),
            };
            lines.push(entryline);
        }

        return lines;
    }

    fn run(&self, entry: &BlockLine) -> anyhow::Result<()> {
        let data = match &entry.data {
            BlockLineData::DesktopEntryData(data) => data,
            _ => return Err(anyhow!("invalid entry. shouldn't ever happen")),
        };
        data.exec()
    }
}

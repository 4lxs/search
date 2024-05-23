use anyhow::{anyhow, Context};
use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};
use std::fmt::Write;
use std::{
    cell::RefCell,
    collections::{hash_map, HashMap},
    path::PathBuf,
    process::Command,
    rc::Rc,
};

use super::{Entry, EntryData, Searcher};

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
                res
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
pub struct DesktopEntrySearcher {
    desktop_entries: HashMap<String, PathBuf>,
}

impl Searcher for DesktopEntrySearcher {
    fn get_lines<'a>(
        &'a mut self,
        me: Rc<RefCell<dyn Searcher>>,
        _input: String,
    ) -> Box<dyn Iterator<Item = Entry>> {
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

            if entry.type_() != Some("Application") {
                continue;
            }

            let mut text = match entry.name(Option::None) {
                Some(name) => name.to_string(),
                None => continue,
            };
            if let Some(generic_name) = entry.generic_name(Option::None) {
                write!(
                    text,
                    " <span weight='light' size='small'><i>({})</i></span>",
                    generic_name
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
                hash_map::Entry::Occupied(..) => continue,
                hash_map::Entry::Vacant(e) => e.insert(path.clone()),
            };

            let line_data = LineData { cmd };

            eprintln!("entry {:?}: {}", path, text);
            let entryline = Entry {
                text,
                icon,
                data: EntryData::DesktopEntry(line_data),
                parent: me.clone(),
            };
            lines.push(entryline);
        }

        Box::from(lines.into_iter())
    }

    fn run(&self, entry: &Entry) -> anyhow::Result<()> {
        let data = match &entry.data {
            EntryData::DesktopEntry(data) => data,
            _ => return Err(anyhow!("invalid entry. shouldn't ever happen")),
        };
        data.exec()
    }
}

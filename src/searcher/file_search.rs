use std::{cell::RefCell, process::Command, rc::Rc};

use anyhow::{anyhow, Context};

use crate::searcher::EntryData;

use super::{Entry, Searcher};

#[derive(Default)]
pub struct FileSearcher {}

pub struct LineData {
    file_path: String,
}

impl Searcher for FileSearcher {
    fn get_lines(
        &mut self,
        me: Rc<RefCell<dyn Searcher>>,
        _input: String,
    ) -> Box<dyn Iterator<Item = Entry>> {
        Box::from(
            vec![Entry {
                text: "hi".into(),
                icon: None,
                data: EntryData::FileSearch(LineData {
                    file_path: "/home/svl/Downloads/Limbo/Uvod.pdf".into(),
                }),
                parent: me,
            }]
            .into_iter(),
        )
    }

    fn run(&self, entry: &Entry) -> anyhow::Result<()> {
        let data = match &entry.data {
            EntryData::FileSearch(data) => data,
            _ => return Err(anyhow!("invalid entry data")),
        };
        Command::new("xdg-open")
            .args([&data.file_path])
            .spawn()
            .with_context(|| format!("running file={}", data.file_path))?;
        Ok(())
    }
}

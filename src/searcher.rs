use std::{cell::RefCell, rc::Rc};

use self::{desktop_entry::DesktopEntrySearcher, file_search::FileSearcher};

mod desktop_entry;
mod file_search;

enum EntryData {
    DesktopEntry(desktop_entry::LineData),
    FileSearch(file_search::LineData),
}

pub struct Entry {
    pub text: String,
    pub icon: Option<String>,
    data: EntryData,
    parent: Rc<RefCell<dyn Searcher>>,
}

impl Entry {
    pub fn run(&self) -> anyhow::Result<()> {
        let parent = self.parent.borrow();
        parent.run(self)
    }
}

pub trait Searcher {
    fn get_lines(
        &mut self,
        me: Rc<RefCell<dyn Searcher>>,
        input: String,
    ) -> Box<dyn Iterator<Item = Entry>>;
    fn run(&self, entry: &Entry) -> anyhow::Result<()>;
}

pub fn all_searchers() -> Vec<Rc<RefCell<dyn Searcher>>> {
    vec![
        Rc::new(RefCell::<DesktopEntrySearcher>::default()) as Rc<RefCell<dyn Searcher>>,
        Rc::new(RefCell::<FileSearcher>::default()) as Rc<RefCell<dyn Searcher>>,
    ]
}

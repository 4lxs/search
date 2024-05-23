mod launcher;
mod searcher;

use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use anyhow::Result;
use searcher::Searcher;

use crate::{
    launcher::{rofi::RofiLauncher, Launcher},
    searcher::all_searchers,
};

struct BlockData<'a> {
    block: Rc<RefCell<dyn Searcher>>,
    lines: Box<dyn Iterator<Item = &'a searcher::Entry>>,
}

fn main() -> Result<()> {
    let searchers = all_searchers();

    let initial_entries = searchers.into_iter().flat_map(|mut searcher| {
        let src = Rc::clone(&searcher);
        (**searcher.borrow_mut())
            .borrow_mut()
            .get_lines(src, "".into())
    });
    let mut launcher = RofiLauncher::new();
    launcher.update(initial_entries)?;

    let entry = loop {
        match launcher.wait()? {
            launcher::Event::InputChange(_input) => {}
            launcher::Event::SelectEntry(entry) => {
                break entry;
            }
        }
    };

    entry.run()
}

mod tests {
    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}

use crate::searcher;

pub mod rofi;

/// the text that the user entered so far
pub struct Input {
    _input: String,
}

pub enum Event<'a> {
    InputChange(Input),
    SelectEntry(&'a searcher::Entry),
}

pub trait Launcher<'a> {
    fn wait(&'a self) -> anyhow::Result<Event<'a>>;
    fn update<'b>(&mut self, entries: impl Iterator<Item = searcher::Entry>) -> anyhow::Result<()>;
}
